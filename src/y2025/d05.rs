use std::ops::RangeInclusive;

use anyhow::Result;
pub fn solve(input: &str) -> Result<(i64, i64)> {
    let (ranges, ingredients) = parse(input);

    let ranges = merge_ranges(ranges);

    let a = ingredients
        .into_iter()
        .filter(|ing| ranges.iter().any(|range| range.contains(ing)))
        .count();

    let b = ranges
        .iter()
        .fold(0, |acc, e| acc + e.end() + 1 - e.start());

    Ok((a as i64, b as i64))
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    ranges.sort_unstable_by_key(|range| (*range.start(), *range.end()));

    let mut out = vec![];
    let mut current_range: Option<RangeInclusive<i64>> = None;
    for r in ranges {
        match &mut current_range {
            Some(merged) => {
                if merged.end() + 1 >= *r.start() {
                    // extend the current range
                    let new_end = *r.end().max(merged.end());
                    *merged = *merged.start()..=new_end;
                } else {
                    // start a new range
                    out.push(merged.clone());
                    current_range = Some(r);
                }
            }
            None => current_range = Some(r),
        }
    }

    if let Some(merged) = current_range {
        out.push(merged);
    }

    out
}

fn parse(input: &str) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let mut ranges = vec![];
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let (a, b) = line.split_once('-').expect("line should contain a dash");
        ranges.push(a.parse().unwrap()..=b.parse().unwrap());
    }

    let ingredients = lines.map(|line| line.parse::<i64>().unwrap()).collect();
    (ranges, ingredients)
}
