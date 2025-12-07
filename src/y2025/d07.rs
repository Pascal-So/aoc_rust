use anyhow::Result;

const START: u8 = b'S';
const SPLIT: u8 = b'^';

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let mut beams: Vec<i64> = lines[0]
        .iter()
        .map(|c| if *c == START { 1 } else { 0 })
        .collect();

    // number of splits
    let mut a = 0;

    for line in &lines[2..] {
        for (i, c) in line.iter().enumerate() {
            if c == &SPLIT && beams[i] > 0 {
                assert!(i > 0);
                a += 1;

                beams[i - 1] += beams[i];
                beams[i + 1] += beams[i];
                beams[i] = 0;
            }
        }
    }

    // number of timelines
    let b = beams.iter().sum();

    Ok((a, b))
}
