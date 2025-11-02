use anyhow::Result;
use std::iter;

use crate::io::parse_entries;

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

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let nums = parse_entries(input, '\n')?;
    Ok((solve_a(&nums), solve_b(&nums)))
}
