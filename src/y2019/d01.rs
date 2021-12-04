use anyhow::Result;
use std::{io::BufRead, iter};

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

pub fn solve(r: impl BufRead) -> Result<(i32, i32)> {
    let nums = crate::io::parse_vec(r, b'\n', false)?;
    Ok((solve_a(&nums), solve_b(&nums)))
}
