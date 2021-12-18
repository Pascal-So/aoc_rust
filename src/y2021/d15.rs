use std::io::{self, BufRead};

use anyhow::{anyhow, Result};
use pathfinding::prelude::astar;

fn wrap(i: u8) -> u8 {
    match i {
        9 => 1,
        _ => i + 1,
    }
}

pub fn solve(buf: impl BufRead) -> Result<(i32, i32)> {
    let mut map = buf.split(b'\n').collect::<io::Result<Vec<_>>>()?;
    let height = map.len();
    let width = map[0].len();

    for line in map.iter_mut() {
        line.resize(width * 5, 0);
        for v in line.iter_mut().take(width) {
            *v -= b'0';
        }
    }
    map.resize(height * 5, vec![0; width * 5]);
    for line in map.iter_mut() {
        for ix in 1..5 {
            for x in 0..width {
                line[ix * width + x] = wrap(line[(ix - 1) * width + x]);
            }
        }
    }
    for iy in 1..5 {
        for y in 0..height {
            for x in 0..(width * 5) {
                map[iy * height + y][x] = wrap(map[(iy - 1) * height + y][x]);
            }
        }
    }

    let solve = |target: (usize, usize)| -> Result<i32> {
        astar(
            &(0, 0),
            |&(x, y)| {
                let map = &map;
                [(1, 0), (0, 1), (-1, 0), (0, -1)]
                    .iter()
                    .filter_map(move |(dx, dy)| {
                        let y = y as i32 + dy;
                        let x = x as i32 + dx;

                        if y >= 0 && y <= target.1 as i32 && x >= 0 && x <= target.0 as i32 {
                            Some((
                                (x as usize, y as usize),
                                (map[y as usize][x as usize]) as i32,
                            ))
                        } else {
                            None
                        }
                    })
            },
            |&(x, y)| (x as i32 - target.0 as i32).abs() + (y as i32 - target.1 as i32).abs(),
            |&pos| pos == target,
        )
        .ok_or_else(|| anyhow!("No path to {:?} found", target))
        .map(|(_, c)| c)
    };

    Ok((
        solve((width - 1, height - 1))?,
        solve((width * 5 - 1, height * 5 - 1))?,
    ))
}
