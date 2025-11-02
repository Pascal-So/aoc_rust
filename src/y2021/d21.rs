use anyhow::{bail, Result};
use itertools::Itertools;

fn solve_a(mut positions: [i32; 2]) -> u64 {
    let mut scores = [0u64; 2];

    for ((rolls, player), count) in (1..=100)
        .cycle()
        .chunks(3)
        .into_iter()
        .zip((0..2).cycle())
        .zip(1..)
    {
        let roll: i32 = rolls.sum();
        let next = (positions[player] - 1 + roll) % 10 + 1;
        positions[player] = next;
        scores[player] += next as u64;

        if scores[player] >= 1000 {
            return scores[1 - player] * count * 3;
        }
    }
    unreachable!()
}

fn solve_b(positions: [i32; 2]) -> u64 {
    // rolls[i] is the number of universes in which a sum
    // of i has been rolled, for one turn with three rolls.
    let mut rolls = [0; 10];
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                rolls[a + b + c] += 1;
            }
        }
    }
    let rolls = rolls;

    // state[p0][s0][p1][s1] is the number of universes where the players are at
    // positions p0+1 and p1+1, and their scores are s0 and s1
    let empty_state = [[[[0u64; 21]; 10]; 21]; 10];

    let mut state = empty_state;
    state[positions[0] as usize - 1][0][positions[1] as usize - 1][0] = 1;

    let mut wins = [0u64; 2];

    for player in (0..=1).cycle() {
        let mut next_state = empty_state;

        let mut keep_going = false;
        for p0 in 0..10 {
            for s0 in 0..21 {
                for p1 in 0..10 {
                    for s1 in 0..21 {
                        let universes = state[p0][s0][p1][s1];
                        if universes > 0 {
                            if player == 0 {
                                for i in 1..=9 {
                                    let new_universes = universes * rolls[i];
                                    let new_pos = (p0 + i) % 10;
                                    let new_score = s0 + new_pos + 1;
                                    if new_score >= 21 {
                                        wins[player] += new_universes;
                                    } else {
                                        keep_going = true;
                                        next_state[new_pos][new_score][p1][s1] += new_universes;
                                    }
                                }
                            } else {
                                for i in 1..=9 {
                                    let new_universes = universes * rolls[i];
                                    let new_pos = (p1 + i) % 10;
                                    let new_score = s1 + new_pos + 1;
                                    if new_score >= 21 {
                                        wins[player] += new_universes;
                                    } else {
                                        keep_going = true;
                                        next_state[p0][s0][new_pos][new_score] += new_universes;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if !keep_going {
            break;
        }
        state = next_state;
    }

    wins[0].max(wins[1])
}

pub fn solve(input: &str) -> Result<(u64, u64)> {
    let positions = input
        .lines()
        .map(|line| Ok(line[28..].parse()?))
        .collect::<Result<Vec<i32>>>()?;

    if positions.len() != 2 {
        bail!("Expected exactly 2 lines!");
    }
    let positions = [positions[0], positions[1]];
    Ok((solve_a(positions), solve_b(positions)))
}
