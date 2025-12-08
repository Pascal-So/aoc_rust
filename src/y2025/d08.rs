use std::collections::BTreeMap;

use anyhow::Result;

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let coords = parse(input);

    // Compute all pairwise distances. this is n^2 but n=1000 so it probably
    // doesn't even make sense to use anything intelligent here.
    let mut distances = BTreeMap::new();
    for (i, coord) in coords.iter().enumerate() {
        for (j, other_coord) in coords[..i].iter().enumerate() {
            let dist = coord.sq_dist(*other_coord);
            distances.insert(dist, (j, i));
        }
    }

    // run union find
    let mut uf = UnionFind::new(coords.len());

    let number_of_connections = if coords.len() < 1000 {
        10 // example
    } else {
        1000 // full task
    };

    let closest_pairs: Vec<_> = distances.into_values().collect();
    for (i, j) in &closest_pairs[..number_of_connections] {
        uf.unify(*i, *j);
    }

    assert!(uf.nr_groups() > 1, "groups should not yet be all connected");

    let mut group_sizes = uf.group_sizes();
    group_sizes.sort_unstable();
    group_sizes.reverse();
    let task_a = group_sizes.into_iter().take(3).product();

    let mut task_b = None;
    // do the remaining connections
    for (i, j) in &closest_pairs[number_of_connections..] {
        uf.unify(*i, *j);
        if uf.nr_groups() == 1 {
            // this connection has merged the last groups
            let xi = i64::from(coords[*i].x);
            let xj = i64::from(coords[*j].x);
            task_b = Some(xi * xj);
            break;
        }
    }

    Ok((task_a, task_b.unwrap()))
}

struct UnionFind {
    parents: Vec<Option<usize>>,
    group_size: Vec<i64>,

    nr_groups: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parents: vec![None; size],
            group_size: vec![1; size],
            nr_groups: size,
        }
    }

    /// Merge two groups
    pub fn unify(&mut self, a: usize, b: usize) {
        let a = self.find_root(a);
        let b = self.find_root(b);
        if a != b {
            self.parents[a] = Some(b);
            self.group_size[b] += self.group_size[a];
            self.nr_groups -= 1;
        }
    }

    /// how many yet disconnected components does the graph have?
    pub fn nr_groups(&self) -> usize {
        self.nr_groups
    }

    /// Return the sizes of the goups. Note that the order in which the sizes
    /// are returned is implementation defined and should not be relied upon.
    pub fn group_sizes(&self) -> Vec<i64> {
        self.group_size
            .iter()
            .zip(self.parents.iter())
            .filter_map(|(size, parent)| {
                if parent.is_none() {
                    // this is a root node
                    Some(*size)
                } else {
                    // we're not at a root node, skip this node
                    None
                }
            })
            .collect()
    }

    fn find_root(&mut self, node: usize) -> usize {
        match self.parents[node] {
            Some(p) => {
                let root = self.find_root(p);

                if root != node {
                    // Update the parent of the current node to point directly to the
                    // root in order to shorten the chain.
                    self.parents[node] = Some(root);
                }
                root
            }
            None => node,
        }
    }
}

/// A 3D cartesian coordinate
#[derive(Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    /// Compute the squared euclidean distance to the other coordinate
    pub fn sq_dist(self, other: Coord) -> i64 {
        let dx = i64::from(self.x - other.x);
        let dy = i64::from(self.y - other.y);
        let dz = i64::from(self.z - other.z);
        dx * dx + dy * dy + dz * dz
    }
}

fn parse(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(",");
            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            let z = iter.next().unwrap().parse().unwrap();
            Coord { x, y, z }
        })
        .collect()
}
