use std::io::BufRead;

use anyhow::Result;

pub fn solve(buf: impl BufRead) -> Result<(i64, i64)> {
    let sums = buf.split(b'\n').enumerate().try_fold(
        [0; 5],
        |mut sums, (i, line)| -> Result<[i64; 5]> {
            let has_tree: Vec<bool> = line?.into_iter().map(|c| c == b'#').collect();

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
