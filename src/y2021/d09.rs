use std::io::BufRead;

use anyhow::Result;

use crate::union_find::UnionFind;

type Data = (i32, u8);
pub fn merger((size_a, min_a): Data, (size_b, min_b): Data) -> Data {
    (size_a + size_b, min_a.min(min_b))
}

pub fn solve(buf: impl BufRead) -> Result<(i32, i32)> {
    let mut uf = UnionFind::with_capacity(100);

    let mut last = vec![];
    let mut current = vec![];
    for l in buf.split(b'\n') {
        let l = l?;
        if current.is_empty() {
            current = vec![None; l.len()];
            last = vec![None; l.len()];
        }

        for (i, c) in l.into_iter().enumerate() {
            let val = c - b'0';
            current[i] = if val == 9 {
                None
            } else {
                Some(uf.new_set((1, val)))
            };

            if let Some(mid) = current[i] {
                if i > 0 {
                    if let Some(left) = current[i - 1] {
                        uf.merge_with(mid, left, merger);
                    }
                }
                if let Some(up) = last[i] {
                    uf.merge_with(mid, up, merger);
                }
            }
        }

        std::mem::swap(&mut last, &mut current);
    }

    let mut basins: Vec<_> = uf.into_iter().map(|e| e.1).collect();
    basins.sort_unstable();

    let risks = basins.iter().map(|d| d.1 as i32 + 1).sum();
    let sizes = basins.iter().rev().take(3).map(|d| d.0).product();

    Ok((risks, sizes))
}
