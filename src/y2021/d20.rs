use std::{io::BufRead, iter::repeat};

use anyhow::{Context, Result};

type Map = Vec<Vec<bool>>;
const PADDING: usize = 52;

#[allow(clippy::ptr_arg)]
fn count(map: &Map) -> usize {
    map.iter()
        .map(|row| row.iter().filter(|b| **b).count())
        .sum()
}

pub fn solve(buf: impl BufRead) -> Result<(usize, usize)> {
    let mut lines = buf.split(b'\n');
    let table_line = lines.next().context("Empty input")??;
    let table: Vec<bool> = table_line.into_iter().map(|c| c == b'#').collect();

    let mut map: Map = vec![];
    let side_padding = [false; PADDING];
    lines.next();
    for line in lines {
        let it = line?.into_iter().map(|c| c == b'#');
        map.push(
            side_padding
                .into_iter()
                .chain(it)
                .chain(side_padding.into_iter())
                .collect(),
        );
    }

    let width = map[0].len();
    let empty_line = vec![false; width];
    map.splice(0..0, repeat(&empty_line).take(PADDING).cloned());
    for _ in 0..PADDING {
        map.push(empty_line.clone());
    }
    let height = map.len();

    let mut count_a = 0;

    let mut map2 = map.clone();
    for i in 0..50 {
        if i == 2 {
            count_a = count(&map);
        }

        for (y, row) in map2.iter_mut().enumerate().skip(1).take(height - 2) {
            for (x, val) in row.iter_mut().enumerate().skip(1).take(width - 2) {
                let idx = map[y - 1][x - 1] as usize * 256
                    + map[y - 1][x] as usize * 128
                    + map[y - 1][x + 1] as usize * 64
                    + map[y][x - 1] as usize * 32
                    + map[y][x] as usize * 16
                    + map[y][x + 1] as usize * 8
                    + map[y + 1][x - 1] as usize * 4
                    + map[y + 1][x] as usize * 2
                    + map[y + 1][x + 1] as usize;

                *val = table[idx];
            }
        }
        if table[0] {
            let value = i % 2 == 0;
            for j in 0..width {
                map2[0][j] = value;
                map2[height - 1][j] = value;
            }
            for row in map2.iter_mut() {
                row[0] = value;
                row[width - 1] = value;
            }
        }

        std::mem::swap(&mut map, &mut map2);
    }

    let count_b = count(&map);
    Ok((count_a, count_b))
}
