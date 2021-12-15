use std::io::BufRead;

use anyhow::{Context, Result};
use itertools::Itertools;

pub fn solve(buf: impl BufRead) -> Result<(usize, usize)> {
    let mut energy: Vec<Vec<u8>> = buf
        .split(b'\n')
        .map_ok(|line| line.iter().map(|c| c - b'0').collect())
        .collect::<std::io::Result<Vec<Vec<u8>>>>()
        .context("Cannot parse input")?;

    let height = energy.len();
    let width = energy[0].len();
    let squids = height * width;

    let mut flashes = 0;
    let mut flashes_after_100 = None;
    let mut synchronized_step = None;

    let mut flash_coords = Vec::with_capacity(squids);

    for step in 1.. {
        flash_coords.clear();
        let flashes_before_this_step = flashes;

        for (y, line) in energy.iter_mut().enumerate() {
            for (x, val) in line.iter_mut().enumerate() {
                *val += 1;
                if *val > 9 {
                    *val = 50;
                    flash_coords.push((y, x));
                }
            }
        }

        for flash_idx in 0.. {
            if flash_idx >= flash_coords.len() {
                break;
            }

            let (y, x) = flash_coords[flash_idx];
            if energy[y][x] >= 100 {
                continue;
            }
            flashes += 1;
            energy[y][x] = 100;
            for (yp, line) in energy
                .iter_mut()
                .take(y + 2)
                .enumerate()
                .skip(y.saturating_sub(1))
            {
                for (xp, val) in line
                    .iter_mut()
                    .take(x + 2)
                    .enumerate()
                    .skip(x.saturating_sub(1))
                {
                    *val += 1;
                    if *val > 9 && *val < 50 {
                        *val = 50;
                        flash_coords.push((yp, xp));
                    }
                }
            }
        }

        for line in energy.iter_mut() {
            for val in line.iter_mut() {
                if *val >= 100 {
                    *val = 0;
                }
            }
        }

        if flashes - flashes_before_this_step == squids {
            synchronized_step = Some(step);
        }

        if step == 100 {
            flashes_after_100 = Some(flashes);
        }

        if synchronized_step.is_some() && flashes_after_100.is_some() {
            break;
        }
    }

    Ok((flashes_after_100.unwrap(), synchronized_step.unwrap()))
}
