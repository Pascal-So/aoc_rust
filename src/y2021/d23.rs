use anyhow::{anyhow, bail, Result};
use pathfinding::prelude::astar;
use std::fmt::Display;
use std::io::BufRead;

type Cost = i64;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
enum Amphipod {
    None,
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Amphipod::None => write!(f, "."),
            Amphipod::Amber => write!(f, "A"),
            Amphipod::Bronze => write!(f, "B"),
            Amphipod::Copper => write!(f, "C"),
            Amphipod::Desert => write!(f, "D"),
        }
    }
}

impl Amphipod {
    fn is_free(&self) -> bool {
        *self == Amphipod::None
    }

    pub fn motion_cost(&self) -> Cost {
        match self {
            Amphipod::None => 0,
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    pub fn target_room(&self) -> usize {
        match self {
            Amphipod::None => 0,
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
        }
    }
}

/// All the information about a game state.
///
/// Only visitable positions are indexed, so the
/// spaces in front of the rooms don't have an
/// index.
///
/// ```text
/// #############
/// #01.2.3.4.56# <- hallway
/// ###3#3#3#3###
///   #2#2#2#2#
///   #1#1#1#1# <- index within a room
///   #0#0#0#0#    increases upwards
///   #########
///    0 1 2 3  <- room index
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct State<const N: usize> {
    rooms: [[Amphipod; N]; 4],
    hallway: [Amphipod; 7],
}

impl<const N: usize> Display for State<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#")?;
        for pos in 0..7 {
            write!(f, "{}", self.hallway[pos])?;
            if pos > 0 && pos < 5 {
                write!(f, " ")?;
            }
        }
        writeln!(f, "#")?;

        write!(f, "###")?;
        for room in 0..4 {
            write!(f, "{}#", self.rooms[room][N - 1])?;
        }
        writeln!(f, "##")?;

        for offset in (0..N - 1).rev() {
            write!(f, "  #")?;
            for room in 0..4 {
                write!(f, "{}#", self.rooms[room][offset])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl State<2> {
    pub fn extend(&self) -> State<4> {
        let a = Amphipod::Amber;
        let b = Amphipod::Bronze;
        let c = Amphipod::Copper;
        let d = Amphipod::Desert;
        let mut rooms = [[a, d, d, a], [a, b, c, a], [a, a, b, a], [a, c, a, a]];
        for (old, new) in self.rooms.iter().zip(rooms.iter_mut()) {
            new[0] = old[0];
            new[3] = old[1];
        }
        State::<4> {
            hallway: self.hallway,
            rooms,
        }
    }
}

impl<const N: usize> State<N> {
    pub fn target() -> State<N> {
        use Amphipod::*;
        State {
            rooms: [[Amber; N], [Bronze; N], [Copper; N], [Desert; N]],
            hallway: [None; 7],
        }
    }

    pub fn next(&self) -> impl IntoIterator<Item = (State<N>, Cost)> {
        let mut v = vec![];

        // Move from Room to Hallway
        for room in 0..4 {
            let mut offset = N + 1;
            for i in (0..N).rev() {
                if !self.rooms[room][i].is_free() {
                    offset = i;
                    break;
                }
            }
            if offset == N + 1 {
                continue;
            }

            let factor = self.rooms[room][offset].motion_cost();

            for (left, dist) in (0..=room + 1).rev().zip((N as Cost + 1..).step_by(2)) {
                if self.hallway[left].is_free() {
                    let mut next = *self;
                    std::mem::swap(&mut next.hallway[left], &mut next.rooms[room][offset]);
                    let cost = (dist - offset as Cost - (left == 0) as Cost) * factor;
                    v.push((next, cost));
                } else {
                    break;
                }
            }
            for (right, dist) in (room + 2..7).zip((N as Cost + 1..).step_by(2)) {
                if self.hallway[right].is_free() {
                    let mut next = *self;
                    std::mem::swap(&mut next.hallway[right], &mut next.rooms[room][offset]);
                    let cost = (dist - offset as Cost - (right == 6) as Cost) * factor;
                    v.push((next, cost));
                } else {
                    break;
                }
            }
        }
        // Move from Hallway to Room
        for pos in 0..7 {
            let amphipod = self.hallway[pos];
            if amphipod.is_free() {
                continue;
            }
            // We can only move to the correct target room.
            let room = amphipod.target_room();

            for offset in 0..N {
                let occupant = self.rooms[room][offset];
                if occupant.is_free() {
                    // Move to the lowest free spot in the room.

                    let (range, mut cost) = if room + 1 >= pos {
                        (
                            pos + 1..room + 2,
                            (room as Cost + 1 - pos as Cost) * 2 - (pos == 0) as Cost,
                        )
                    } else {
                        (
                            room + 2..pos,
                            (pos as Cost - room as Cost - 2) * 2 - (pos == 6) as Cost,
                        )
                    };

                    // Is the path in the hallway to the room clear?
                    if range.into_iter().all(|p| self.hallway[p].is_free()) {
                        cost = cost + 1 + (N - offset) as Cost;
                        cost *= amphipod.motion_cost();

                        let mut next = *self;
                        std::mem::swap(&mut next.hallway[pos], &mut next.rooms[room][offset]);
                        v.push((next, cost));
                    }
                    break;
                } else if occupant != amphipod {
                    // We're only allowed to move into the room if all occupants that are already
                    // there are of the correct colour and thus at their correct final spot.
                    break;
                }
            }
        }

        v.into_iter()
    }

    // We calculate the cost of moving every amphipod to the lower slot
    // of their room and then substract the 4 overconted steps at the
    // end.
    pub fn min_cost(&self) -> Cost {
        let mut total = 0;
        for room in 0..4 {
            for offset in 0..2 {
                let amphipod = self.rooms[room][offset];
                if !amphipod.is_free() {
                    let target_room = amphipod.target_room();
                    total += amphipod.motion_cost()
                        * if room == target_room {
                            offset as Cost
                        } else {
                            (target_room as Cost - room as Cost).abs() * 2 + 4 - offset as Cost
                        };
                }
            }
        }
        for pos in 0..7 {
            let amphipod = self.hallway[pos];
            let target_room = amphipod.target_room();
            total += amphipod.motion_cost()
                * if pos < target_room + 2 {
                    (target_room + 1 - pos) as Cost * 2 - (pos == 0) as Cost + 3
                } else {
                    (pos - target_room - 2) as Cost * 2 - (pos == 6) as Cost + 3
                };
        }
        let factor = N * (N - 1) / 2;
        total - 1111 * factor as Cost
    }
}

fn parse(buf: impl BufRead) -> Result<State<2>> {
    let mut lines = buf.split(b'\n');
    lines.next();
    lines.next();
    let a = lines
        .next()
        .ok_or_else(|| anyhow!("Not enough lines in input"))??;
    let b = lines
        .next()
        .ok_or_else(|| anyhow!("Not enough lines in input"))??;

    let hallway = [Amphipod::None; 7];
    let mut rooms = [[Amphipod::None; 2]; 4];

    for (v, offset) in &[(a, 1), (b, 0)] {
        for (room, &pos) in [3, 5, 7, 9].iter().enumerate() {
            let amphipod = match v[pos] {
                b'A' => Amphipod::Amber,
                b'B' => Amphipod::Bronze,
                b'C' => Amphipod::Copper,
                b'D' => Amphipod::Desert,
                _ => bail!("unexpected char: {}", v[pos] as char),
            };

            rooms[room][*offset] = amphipod;
        }
    }

    Ok(State { hallway, rooms })
}

pub fn solve(buf: impl BufRead) -> Result<(Cost, Cost)> {
    let start = parse(buf)?;
    let target = State::target();

    let cost_a = astar(&start, State::next, State::min_cost, |state| {
        state == &target
    })
    .ok_or_else(|| anyhow!("No path found"))?
    .1;

    let full = start.extend();
    let target_full = State::target();
    let cost_b = astar(&full, State::next, State::min_cost, |state| {
        state == &target_full
    })
    .ok_or_else(|| anyhow!("No path found"))?
    .1;

    Ok((cost_a, cost_b))
}
