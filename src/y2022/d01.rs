use anyhow::Result;

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let numbers = input
        .split('\n')
        .map(|entry| -> Result<Option<i32>> {
            Ok(if entry.is_empty() {
                None
            } else {
                Some(entry.parse::<i32>()?)
            })
        })
        .chain(std::iter::once(Ok(None)))
        .collect::<Result<Vec<_>, _>>()?;

    let (max, _) = numbers.into_iter().fold(
        ([0; 3], 0),
        |(mut max, current_sum), line| -> ([i32; 3], i32) {
            if let Some(line) = line {
                let calories = line;
                (max, current_sum + calories)
            } else {
                let mut insert = current_sum;
                println!("inserting {insert} in {max:?}");
                for v in &mut max {
                    if insert > *v {
                        std::mem::swap(v, &mut insert);
                    }
                }
                (max, 0)
            }
        },
    );

    Ok((max[0], max[0] + max[1] + max[2]))
}
