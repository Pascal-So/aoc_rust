use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use anyhow::{anyhow, Context, Result};

use crate::io;

const TARGET: i32 = 2020;

pub fn solve_a(nums: &[i32]) -> Option<i32> {
    let mut map = HashSet::new();
    for n in nums {
        let difference = TARGET - n;
        if map.contains(&difference) {
            return Some(n * difference);
        }
        map.insert(n);
    }

    dbg!(map);
    None
}

pub fn solve_b(nums: &[i32]) -> Option<i32> {
    let mut twos = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        let difference = TARGET - n;
        if let Some(prod) = twos.get(&difference) {
            return Some(n * prod);
        }

        for lower in &nums[0..i] {
            twos.insert(lower + n, lower * n);
        }
    }

    None
}

pub fn solve(buf: impl BufRead) -> Result<(i32, i32)> {
    let lines = io::parse_vec(buf, b'\n').context("Cannot parse input")?;
    let a = solve_a(&lines).ok_or_else(|| anyhow!("No solution found for first subtask"))?;
    let b = solve_b(&lines).ok_or_else(|| anyhow!("No solution found for second subtask"))?;

    Ok((a, b))
}
