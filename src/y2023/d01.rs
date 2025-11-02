use aho_corasick;
use anyhow::Result;

fn parse_line_a(line: &str) -> (i32, i32) {
    let mut d1 = None;
    let mut d2 = 0;

    for c in line.chars() {
        if let Some(d) = c.to_digit(10) {
            let d = d as i32;
            d1 = d1.or_else(|| Some(d));
            d2 = d;
        }
    }
    (d1.unwrap_or(0), d2)
}

fn parse_line_b(line: &str) -> (i32, i32) {
    let ac = aho_corasick::AhoCorasick::new(&[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ])
    .unwrap();

    let mut d1 = None;
    let mut d2 = None;

    for mat in ac.find_overlapping_iter(line) {
        let digit = match mat.pattern().as_i32() {
            i @ 0..=8 => i + 1,
            i @ 9..=17 => i - 8,
            _ => unreachable!(),
        };

        d1 = d1.or_else(|| Some(digit));
        d2 = Some(digit);
    }

    (d1.unwrap(), d2.unwrap())
}

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let mut sum_a = 0;
    let mut sum_b = 0;
    for line in input.lines() {
        let (d1_a, d2_a) = parse_line_a(line);
        let (d1_b, d2_b) = parse_line_b(line);
        sum_a += d1_a * 10 + d2_a;
        sum_b += d1_b * 10 + d2_b;
    }

    Ok((sum_a, sum_b))
}
