use anyhow::{anyhow, Result};

use crate::io;

type Num = u32;

fn parse_line(line: &str) -> Num {
    line.chars()
        .fold(0, |acc, c| match c {
            '0'..='9' => 2 * acc + (c as u32 - '0' as u32) as Num,
            _ => panic!("Invalid char {}", c),
        })
}

fn parse(input: &str) -> (Vec<Num>, usize) {
    let mut nr_bits = 0;
    let vec = io::split_entries(input, '\n')
        .map(|line| {
            nr_bits = line.len();
            parse_line(&line)
        })
        .collect();

    (vec, nr_bits)
}

fn find_life_support(nums: &[Num], nr_bits: usize, oxygen: bool) -> Option<Num> {
    let narrowed = (0..nr_bits).rev().fold(nums, |view, index| {
        if view.len() <= 1 {
            return view;
        }

        let boundary = view.partition_point(|num| (num & 1 << index) == 0);

        if (boundary > view.len() / 2) == oxygen {
            &view[..boundary]
        } else {
            &view[boundary..]
        }
    });

    narrowed.first().copied()
}

pub fn solve_b(nums: &mut [Num], nr_bits: usize) -> Result<u64> {
    nums.sort_unstable();

    let oxygen = find_life_support(nums, nr_bits, true)
        .ok_or_else(|| anyhow!("No oxygen left after binary search."))?;
    let co2 = find_life_support(nums, nr_bits, false)
        .ok_or_else(|| anyhow!("No CO2 left after binary search."))?;

    Ok(oxygen as u64 * co2 as u64)
}

pub fn solve_a(nums: &[Num], nr_bits: usize) -> usize {
    let mut counts = vec![0; nr_bits];

    for num in nums {
        for (i, c) in counts.iter_mut().enumerate() {
            *c += (num & (1 << i) > 0) as usize;
        }
    }

    let gamma: usize = counts
        .into_iter()
        .enumerate()
        .map(|(index, c)| (1 << index) * (c > nums.len() / 2) as usize)
        .sum();

    let epsilon = !gamma & ((1 << nr_bits) - 1);
    gamma * epsilon
}

pub fn solve(input: &str) -> Result<(usize, u64)> {
    let (mut nums, nr_bits) = parse(input);

    let power = solve_a(&nums, nr_bits);
    let life = solve_b(&mut nums, nr_bits)?;

    Ok((power, life))
}
