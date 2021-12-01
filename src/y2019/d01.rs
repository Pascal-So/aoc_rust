use anyhow::Result;
use std::{fmt::Display, io::BufRead, iter};

fn fuel(mass: &i32) -> i32 {
    mass / 3 - 2
}

fn solve_a(nums: &[i32]) -> i32 {
    nums.iter().map(fuel).sum()
}

fn solve_b(nums: &[i32]) -> i32 {
    nums.iter()
        .map(|&m| {
            iter::successors(Some(m), |m| Some(fuel(m)))
                .take_while(|&m| m > 0)
                .skip(1)
        })
        .flatten()
        .sum()
}

#[derive(PartialEq, Eq, Debug)]
pub struct Solution {
    pub simple_fuel: i32,
    pub iterated_fuel: i32,
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Year 2019 Day 01")?;
        writeln!(f, "  Simple fuel: {}", self.simple_fuel)?;
        writeln!(f, "  Iterated fuel: {}", self.iterated_fuel)
    }
}

pub fn solve(r: impl BufRead) -> Result<Solution> {
    let nums = crate::io::parse_vec(r, b'\n')?;
    Ok(Solution {
        simple_fuel: solve_a(&nums),
        iterated_fuel: solve_b(&nums),
    })
}
