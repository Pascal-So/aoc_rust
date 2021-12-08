use std::io::BufRead;

use anyhow::Result;

use crate::io;

pub fn solve(buf: impl BufRead) -> Result<(i64, i64)> {
    let mut positions: Vec<i64> = io::parse_vec(buf, b',', false)?;
    positions.sort_unstable();
    let len = positions.len();
    let median = positions[len / 2];
    let mean = (positions.iter().sum::<i64>() + len as i64 / 2) / len as i64;

    let mut sum_a = 0;
    let mut sum_b = [0; 3];
    for p in positions {
        sum_a += (p - median).abs();

        // we might be off by one in either direction
        for (i, p_shifted) in (p - 1..=p + 1).enumerate() {
            let d_mean = (p_shifted - mean).abs();
            sum_b[i] += d_mean * (d_mean + 1) / 2;
        }
    }

    Ok((sum_a, sum_b.into_iter().min().unwrap()))
}
