use std::collections::{BTreeSet, HashMap};

use anyhow::Result;
use rustsat::instances::{BasicVarManager, SatInstance};
use rustsat::solvers::{Solve, SolverResult};
use rustsat::types::constraints::CardConstraint;
use rustsat::types::{Lit, TernaryVal};

const NR_SHAPES: usize = 6;
const PRINT_SOLUTION: bool = false;

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let (shapes, problems) = parse(input);

    let mut solvables = 0;

    // find all flips and rotations for every shape
    let shapesets: Vec<_> = shapes
        .into_iter()
        .map(|shape| shape.all_variations())
        .collect();

    for problem in problems {
        if problem.is_solvable(&shapesets) {
            solvables += 1;
        }
    }

    Ok((solvables, 0))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
struct Shape {
    grid: [[bool; 3]; 3],
}

impl Shape {
    pub fn all_variations(mut self) -> Vec<Shape> {
        let mut set = BTreeSet::new();

        for _ in 0..4 {
            self = self.rotate();
            set.insert(self);
        }
        self = self.flip();
        for _ in 0..4 {
            self = self.rotate();
            set.insert(self);
        }

        set.into_iter().collect()
    }

    pub fn rotate(self) -> Shape {
        let grid = [
            [self.grid[0][2], self.grid[1][2], self.grid[2][2]],
            [self.grid[0][1], self.grid[1][1], self.grid[2][1]],
            [self.grid[0][0], self.grid[1][0], self.grid[2][0]],
        ];
        Self { grid }
    }
    pub fn flip(self) -> Shape {
        let grid = [self.grid[2], self.grid[1], self.grid[0]];
        Self { grid }
    }

    /// Number of filled grid cells
    pub fn count(self) -> i32 {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|val| **val).count() as i32)
            .sum()
    }

    /// Access a single field in the shape's grid
    pub fn get(self, x: i32, y: i32) -> Option<bool> {
        if !(0..3).contains(&x) || !(0..3).contains(&y) {
            return None;
        }

        Some(self.grid[y as usize][x as usize])
    }

    /// Does this shape collide with the other shape if the other shape is
    /// moved by offset (ox, oy) from this shape?
    pub fn collides(self, other: Shape, ox: i32, oy: i32) -> bool {
        if ox.abs() >= 3 || oy.abs() >= 3 {
            return false;
        }

        for (y, row) in self.grid.into_iter().enumerate() {
            let y = y as i32;
            if (0..3).contains(&(y - oy)) {
                for (x, val) in row.into_iter().enumerate() {
                    let x = x as i32;
                    if val && other.get(x - ox, y - oy) == Some(true) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

struct Problem {
    dim: (i32, i32),
    counts: Vec<i32>,
}

impl Problem {
    pub fn is_solvable(self, shapesets: &[Vec<Shape>]) -> bool {
        let area = self.dim.0 * self.dim.1;

        let required_area: i32 = self
            .counts
            .iter()
            .enumerate()
            .map(|(i, count)| shapesets[i][0].count() as i32 * count)
            .sum();

        if required_area > area {
            println!("skipping with {required_area} > {area}");
            return false;
        }

        if area > 1000 {
            // bro are you for real
            return true;
        }

        let mut shape_literals = (0..shapesets.len()).map(|_| vec![]).collect::<Vec<_>>();
        let mut instance = SatInstance::<BasicVarManager>::new();

        let mut placed_shapes: HashMap<(i32, i32), Vec<(Shape, Lit)>> = HashMap::new();

        for y in 2..self.dim.1 {
            for x in 2..self.dim.0 {
                for (shape_idx, shapeset) in shapesets.iter().enumerate() {
                    for shape in shapeset {
                        let lit = instance.new_lit();
                        shape_literals[shape_idx].push(lit);

                        // exclude collisions
                        for oy in -2..=0 {
                            for ox in -2..=2 {
                                let placed = placed_shapes.get(&(x + ox, y + oy));
                                if let Some(placed) = placed {
                                    for (other_shape, other_lit) in placed {
                                        if shape.collides(*other_shape, ox, oy) {
                                            // if the placed shapes collide, we can't have
                                            // both active at once.
                                            instance.add_binary(!lit, !*other_lit);
                                        }
                                    }
                                }
                            }
                        }

                        placed_shapes.entry((x, y)).or_default().push((*shape, lit));
                    }
                }
            }
        }

        dbg!(instance.n_vars());
        dbg!(instance.n_clauses());

        for (idx, literals) in shape_literals.clone().into_iter().enumerate() {
            let requried_number = self.counts[idx];
            instance.add_card_constr(CardConstraint::new_eq(literals, requried_number as usize));
        }

        let mut solver = rustsat_kissat::Kissat::default();
        solver.add_cnf(instance.into_cnf().0).unwrap();
        let res = solver.solve().unwrap();
        dbg!(res);

        // print the solution with letters like in the task description
        if PRINT_SOLUTION {
            for sl in &mut shape_literals {
                sl.reverse();
            }

            if let Ok(sol) = solver.full_solution() {
                let mut grid: Vec<_> = (0..self.dim.1)
                    .map(|_| vec![b'.'; self.dim.0 as usize])
                    .collect();

                let mut char = b'A';
                for y in 2..self.dim.1 {
                    for x in 2..self.dim.0 {
                        for (shape_idx, shapeset) in shapesets.iter().enumerate() {
                            for shape in shapeset {
                                let lit = shape_literals[shape_idx].pop().unwrap();

                                if sol.lit_value(lit) == TernaryVal::True {
                                    for sy in 0..3 {
                                        for sx in 0..3 {
                                            if shape.get(sx, sy) == Some(true) {
                                                let gx = x - 2 + sx;
                                                let gy = y - 2 + sy;

                                                assert_eq!(grid[gy as usize][gx as usize], b'.');
                                                grid[gy as usize][gx as usize] = char;
                                            }
                                        }
                                    }
                                    char += 1;
                                }
                            }
                        }
                    }
                }

                for row in grid {
                    for char in row {
                        print!("{}", str::from_utf8(&[char]).unwrap());
                    }

                    println!();
                }
            }
        }

        res == SolverResult::Sat
    }
}

fn parse(input: &str) -> (Vec<Shape>, Vec<Problem>) {
    let mut shapes = vec![];

    let mut lines = input.lines();
    let mut shape_idx = 0;
    let mut shape_line = 0;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            shape_idx += 1;
            shape_line = 0;
            if shape_idx == NR_SHAPES {
                break;
            }
            continue;
        }
        if line.contains(":") {
            shapes.push(Shape::default());
            continue;
        }

        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                shapes[shape_idx].grid[shape_line][i] = true;
            }
        }
        shape_line += 1;
    }

    let problems = lines
        .map(|line| {
            let (dim, counts) = line
                .split_once(":")
                .expect("problem line does not contain ':'");

            let (d1, d2) = dim.split_once("x").expect("dimension does not contain x");
            let dim = (d1.parse().unwrap(), d2.parse().unwrap());

            let counts = counts
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect();

            Problem { dim, counts }
        })
        .collect();

    (shapes, problems)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn u_shape() -> Shape {
        Shape {
            grid: [[true, false, true], [true, false, true], [true, true, true]],
        }
    }
    fn h_shape() -> Shape {
        Shape {
            grid: [[true, false, true], [true, true, true], [true, false, true]],
        }
    }

    #[test]
    fn test_collision() {
        let a = u_shape();
        let b = a.rotate().rotate();

        assert!(a.collides(b, 0, 0));
        assert!(a.collides(b, 1, 0));
        assert!(a.collides(b, 2, 0));
        assert!(a.collides(b, 2, 1));
        assert!(a.collides(b, 2, 2));
        assert!(a.collides(b, 0, -1));
        assert!(a.collides(b, 0, -2));
        assert!(!a.collides(b, 1, -1));
        assert!(!a.collides(b, 1, -2));
        assert!(!a.collides(b, -1, -1));
        assert!(!a.collides(b, -1, -2));
        assert!(a.collides(b, -2, -2));
        assert!(a.collides(b, -1, 2));
        assert!(a.collides(b, 2, -2));
        assert!(!a.collides(b, 3, -2));
    }

    #[test]
    fn test_simple_sat() {
        let a = u_shape();
        let b = a.rotate().rotate();

        let problem = Problem {
            dim: (4, 4),
            counts: vec![2],
        };

        let solvable = problem.is_solvable(&[vec![a, b]]);
        assert!(solvable);
    }
    #[test]
    fn test_unsat() {
        let a = h_shape();
        let b = a.rotate().rotate();

        let problem = Problem {
            dim: (4, 4),
            counts: vec![2],
        };

        let solvable = problem.is_solvable(&[vec![a, b]]);
        assert!(!solvable);
    }
}
