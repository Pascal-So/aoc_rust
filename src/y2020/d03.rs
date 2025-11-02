use anyhow::Result;

use crate::io;

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let sums = io::split_entries(input, '\n')
        .into_iter()
        .enumerate()
        .try_fold(
            [0; 5],
            |mut sums, (i, line)| -> Result<[i64; 5]> {
                let has_tree: Vec<bool> = line.chars().map(|c| c == '#').collect();

                sums[0] += has_tree[i % has_tree.len()] as i64;
                sums[1] += has_tree[(i * 3) % has_tree.len()] as i64;
                sums[2] += has_tree[(i * 5) % has_tree.len()] as i64;
                sums[3] += has_tree[(i * 7) % has_tree.len()] as i64;

                if i % 2 == 0 {
                    sums[4] += has_tree[(i / 2) % has_tree.len()] as i64;
                }

                Ok(sums)
            },
        )?;

    Ok((sums[1], sums.iter().product()))
}
