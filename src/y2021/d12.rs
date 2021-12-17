use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    io::BufRead,
};

use anyhow::{bail, Result};
use bitvec::{array::BitArray, order::Lsb0};
use combine::{
    many1,
    parser::char::{char, letter},
    Parser,
};

use crate::parse::combine_parse;

type RoomIndex = (bool, usize);
type Visited = BitArray<Lsb0, [u8; 1]>;

struct Graph {
    edges: [Vec<Vec<RoomIndex>>; 2],
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            edges: [vec![], vec![]],
        }
    }

    fn insert_directed(&mut self, a: RoomIndex, b: RoomIndex) {
        let l = &mut self.edges[a.0 as usize];
        l.resize(l.len().max(a.1 + 1), vec![]);
        l[a.1].push(b);
    }

    pub fn insert(&mut self, a: RoomIndex, b: RoomIndex) {
        self.insert_directed(a, b);
        self.insert_directed(b, a);
    }

    pub fn get(&self, idx: RoomIndex) -> &Vec<RoomIndex> {
        &self.edges[idx.0 as usize][idx.1]
    }
}

fn rec(
    current: RoomIndex,
    graph: &Graph,
    ends: &HashSet<RoomIndex>,
    mut visited: Visited,
    mut double: bool,
) -> [i64; 2] {
    let mut out = [0; 2];

    if !current.0 {
        if visited[current.1] {
            if double {
                return out;
            } else {
                double = true;
            }
        } else {
            visited.set(current.1, true);
        }
    }

    if ends.contains(&current) {
        out[1] += 1;
        if !double {
            out[0] += 1;
        }
    }

    for e in graph.get(current) {
        let [s, d] = rec(*e, graph, ends, visited, double);
        out[0] += s;
        out[1] += d;
    }

    out
}

struct Interner {
    room_ids: HashMap<[u8; 2], RoomIndex>,
    pub nr_small_rooms: usize,
    pub nr_big_rooms: usize,
}

impl Interner {
    pub fn new() -> Interner {
        Interner {
            room_ids: HashMap::new(),
            nr_small_rooms: 0,
            nr_big_rooms: 0,
        }
    }

    pub fn intern(&mut self, room: &str) -> Option<RoomIndex> {
        let mut iter = room.bytes();
        let arr = [iter.next()?, iter.next().unwrap_or(b'a')];
        if iter.next().is_some() {
            return None;
        }

        let is_big = arr[0] >= b'A' && arr[0] <= b'Z';
        let counter = if is_big {
            &mut self.nr_big_rooms
        } else {
            &mut self.nr_small_rooms
        };
        let idx = (is_big, *counter);

        Some(match self.room_ids.entry(arr) {
            Entry::Vacant(e) => {
                e.insert(idx);
                *counter += 1;
                idx
            }
            Entry::Occupied(e) => *e.get(),
        })
    }
}

pub fn solve(buf: impl BufRead) -> Result<(i64, i64)> {
    let mut ends = HashSet::new();
    let mut starts = vec![];

    let mut interner = Interner::new();
    let mut graph = Graph::new();

    for line in buf.lines() {
        let parser = many1(letter()).skip(char('-')).and(many1(letter()));
        let (mut a, mut b): (String, String) = combine_parse(parser, &*line?)?;

        if a.len() < b.len() {
            std::mem::swap(&mut a, &mut b);
        }

        let id_b = interner
            .intern(&b)
            .ok_or_else(|| anyhow::anyhow!("invalid room \"{}\"", b))?;

        if a == "start" {
            starts.push(id_b);
        } else if a == "end" {
            ends.insert(id_b);
        } else {
            let id_a = interner
                .intern(&a)
                .ok_or_else(|| anyhow::anyhow!("invalid room \"{}\"", a))?;
            graph.insert(id_a, id_b);
        }
    }

    if interner.nr_small_rooms > 8 {
        bail!("Too many small rooms: {}", interner.nr_small_rooms);
    }
    let visited = Visited::new([0]);

    let mut total_single = 0;
    let mut total_double = 0;
    for room in starts {
        let [s, d] = rec(room, &graph, &ends, visited, false);
        total_single += s;
        total_double += d;
    }

    Ok((total_single, total_double))
}
