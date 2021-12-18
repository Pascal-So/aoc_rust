use std::{collections::HashMap, io::BufRead};

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;

const NR_CHARS: usize = to_index(b'Z') + 1;

const fn to_index(c: u8) -> usize {
    (c - b'A') as usize
}

fn get_quantity_diff(
    field: &[[u64; NR_CHARS]; NR_CHARS],
    first: usize,
    last: usize,
) -> Result<u64> {
    let mut counts = [0; NR_CHARS];
    for (a, line) in field.iter().enumerate() {
        for (b, count) in line.iter().enumerate() {
            counts[a] += count;
            counts[b] += count;
        }
    }

    counts[first] += 1;
    counts[last] += 1;

    (|| -> Option<u64> {
        let max = counts.iter().max()?;
        let min = counts.iter().filter(|&n| *n > 0).min()?;
        Some((max - min) / 2)
    })()
    .ok_or_else(|| anyhow!("No items found in `counts` array."))
}

pub fn solve(buf: impl BufRead) -> Result<(u64, u64)> {
    let mut lines = buf.split(b'\n');

    let init = lines.next().ok_or_else(|| anyhow!("empty input!"))??;
    let mut rules = HashMap::with_capacity(100);
    lines.next();
    for line in lines {
        let line = line?;
        if let [a, b, b' ', b'-', b'>', b' ', c] = line[..] {
            rules.insert((to_index(a), to_index(b)), to_index(c));
        } else {
            bail!(
                "line does not match expected format: '{}'",
                String::from_utf8(line)?
            );
        }
    }

    let mut field_1 = [[0_u64; NR_CHARS]; NR_CHARS];
    #[allow(unused_assignments)]
    let mut field_2 = field_1;

    for (a, b) in init.iter().copied().map(to_index).tuple_windows() {
        field_1[a][b] += 1;
    }

    let first = to_index(init[0]);
    let last = to_index(init[init.len() - 1]);

    let mut diff_10 = 0;
    for i in 1..=40 {
        field_2 = [[0; NR_CHARS]; NR_CHARS];
        for (a, line) in field_1.iter().enumerate() {
            for (b, count) in line.iter().enumerate() {
                if let Some(&c) = rules.get(&(a, b)) {
                    field_2[a][c] += count;
                    field_2[c][b] += count;
                } else {
                    field_2[a][b] += count;
                }
            }
        }
        std::mem::swap(&mut field_1, &mut field_2);

        if i == 10 {
            diff_10 = get_quantity_diff(&field_1, first, last)?;
        }
    }

    Ok((diff_10, get_quantity_diff(&field_1, first, last)?))
}
