use std::{collections::VecDeque, io::BufRead};

use anyhow::Result;

use crate::io;

pub fn solve(buf: impl BufRead) -> Result<(i64, i64)> {
    let mut fishies_at = VecDeque::from(vec![0; 9]);
    for timer in io::parse_vec(buf, b',', false)? {
        fishies_at[timer] += 1;
    }

    let mut a = 0;
    let mut b = 0;
    for day in 1..=256 {
        let fishies_at_zero = fishies_at.pop_front().unwrap();
        fishies_at.push_back(fishies_at_zero);
        fishies_at[6] += fishies_at_zero;

        if day == 80 {
            a = fishies_at.iter().sum();
        } else if day == 256 {
            b = fishies_at.iter().sum();
        }
    }
    Ok((a, b))
}
