use anyhow::Result;

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let mut a = 0;
    let mut b = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let battery_bank: Vec<_> = line
            .as_bytes()
            .into_iter()
            .map(|b| i64::from(b - b'0'))
            .collect();

        a += max_joltage(2, &battery_bank);
        b += max_joltage(12, &battery_bank);
    }

    Ok((a, b))
}

fn max_joltage(nr_batteries: usize, battery_bank: &[i64]) -> i64 {
    let mut joltage = 0;
    let mut start_pos = 0;

    for idx in 0..nr_batteries {
        let remaining = nr_batteries - idx - 1;
        // we need to leave space to the right to fit the remaining required batteries
        let search_segment = &battery_bank[start_pos..battery_bank.len() - remaining];

        // find the maximal battery and the first pos at which it was found
        let max = search_segment.iter().max().unwrap();
        let max_pos = search_segment.iter().position(|d| d == max).unwrap() + start_pos;

        // next search starts one after the current battery
        start_pos = max_pos + 1;
        joltage *= 10;
        joltage += max;
    }

    joltage
}
