use std::{io::BufRead, error::Error, str::FromStr};

use anyhow::{Result, Context};


// todo: generalize this to a block parser once more such input formats appear, then move it to crate::io
fn parse_opt<B, T>(buf: B, sep: u8, skip_empty: bool) -> impl Iterator<Item = Option<Result<T>>>
where
    B: BufRead,
    T: FromStr,
    <T as FromStr>::Err: Error + Send + Sync + 'static,
{
    buf.split(sep).zip(1..).map(move |(entry, nr)| {
        (|| {
            let s = String::from_utf8(entry.context(format!("Cannot read entry {}", nr))?)
                .context(format!("Cannot read entry {}", nr))?;
            let trimmed = s.trim();
            if trimmed.is_empty() && skip_empty {
                Ok(None)
            } else {
                Ok(Some(trimmed.parse::<T>()?))
            }
        })()
        .transpose()
    })
}

pub fn solve(buf: impl BufRead) -> Result<(i32, i32)> {
    let (max, _) = parse_opt::<_, i32>(buf, b'\n', true).chain(std::iter::once(None)).try_fold(
        ([0; 3], 0),
        |(mut max, current_sum), line| -> Result<([i32; 3], i32)> {
            Ok(if let Some(line) = line {
                let calories = line.context("i32 parse error")?;
                (max, current_sum + calories)
            } else {
                let mut insert = current_sum;
                for v in &mut max {
                    if insert > *v {
                        std::mem::swap(v, &mut insert);
                    }
                }
                (max, 0)
            })
        },
    )?;

    Ok((max[0], max[0] + max[1] + max[2]))
}
