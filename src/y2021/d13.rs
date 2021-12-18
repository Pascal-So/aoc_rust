use std::{collections::HashSet, io::BufRead};

use anyhow::{bail, Context, Result};

pub fn solve(buf: impl BufRead) -> Result<usize> {
    let mut points = HashSet::new();
    let mut lines = buf.split(b'\n');

    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let sep_pos = line.iter().position(|&c| c == b',').unwrap();

        points.insert(
            (|| -> Result<(i32, i32)> {
                Ok((
                    std::str::from_utf8(&line[..sep_pos])?.parse()?,
                    std::str::from_utf8(&line[sep_pos + 1..])?.parse()?,
                ))
            })()
            .context("Cannot parse coordinates")?,
        );
    }

    let subtask = 1;

    for line in lines {
        let line = line?;
        if line.len() < 14 {
            bail!(
                "Fold instructon line too short: '{}'",
                String::from_utf8(line)?
            );
        }
        let axis = line[11];
        let pos: i32 = std::str::from_utf8(&line[13..])?.parse()?;

        let mut folded = HashSet::new();
        for &(x, y) in points.iter() {
            let new_coords = match axis {
                b'x' => (x - 2 * (x - pos).max(0), y),
                b'y' => (x, y - 2 * (y - pos).max(0)),
                _ => bail!("Unexpected axis {}", axis as char),
            };
            folded.insert(new_coords);
        }
        std::mem::swap(&mut folded, &mut points);

        if subtask == 1 {
            break;
        }
    }

    if subtask == 2 {
        let (max_x, max_y) = points
            .iter()
            .fold((0, 0), |(lx, ly), (cx, cy)| (lx.max(*cx), ly.max(*cy)));

        for y in 0..=max_y {
            for x in 0..=max_x {
                if points.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    Ok(points.len())
}
