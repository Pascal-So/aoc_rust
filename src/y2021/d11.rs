use std::io::BufRead;

use anyhow::{Context, Result};
use itertools::Itertools;

pub fn solve(buf: impl BufRead) -> Result<(usize, usize)> {
    let mut energy: Vec<Vec<u8>> = buf
        .split(b'\n')
        .map_ok(|line| line.iter().map(|c| c - b'0').collect())
        .collect::<std::io::Result<Vec<Vec<u8>>>>()
        .context("Cannot parse input")?;

    let mut flashes = 0;
    let mut flashes_after_100 = None;
    let mut synchronized_step = None;

    for step in 1.. {
        let flashes_before_this_step = flashes;

        for line in energy.iter_mut() {
            for val in line.iter_mut() {
                *val += 1;
            }
        }

        let mut changed = true;
        while changed {
            changed = false;
            for y in 0..energy.len() {
                for x in 0..energy[y].len() {
                    if energy[y][x] > 9 && energy[y][x] < 100 {
                        energy[y][x] = 100;
                        changed = true;
                        flashes += 1;
                        for yp in y.saturating_sub(1)..=y + 1 {
                            for xp in x.saturating_sub(1)..=x + 1 {
                                energy
                                    .get_mut(yp)
                                    .and_then(|line| line.get_mut(xp).map(|v| *v += 1));
                            }
                        }
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

        if flashes - flashes_before_this_step == energy.len() * energy[0].len() {
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
