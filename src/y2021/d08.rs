use std::{collections::HashMap, io::BufRead};

use anyhow::{bail, Result};
use combine::{
    many1,
    parser::byte::{byte, lower, space},
    sep_by1, sep_end_by1, Parser,
};

use crate::parse::combine_parse;

type Digits = Vec<Vec<u8>>;

fn parse_line(buf: &[u8]) -> Result<(Digits, Digits)> {
    let (mut p, mut o): (Digits, Digits) = combine_parse(
        sep_end_by1(many1(lower()), space()).and(
            byte(b'|')
                .with(space())
                .with(sep_by1(many1(lower()), space())),
        ),
        buf,
    )?;

    p.iter_mut().for_each(|v| v.sort_unstable());
    o.iter_mut().for_each(|v| v.sort_unstable());
    Ok((p, o))
}

fn single_diff(big: &[u8], small: &[u8]) -> Option<u8> {
    assert_eq!(big.len(), small.len() + 1);
    let mut missing = None;
    let mut i = 0;
    for c in small.iter() {
        if big[i] == *c {
            i += 1;
            continue;
        } else {
            if missing.is_some() {
                return None;
            }
            missing = Some(big[i]);

            if big[i + 1] != *c {
                return None;
            }
            i += 2;
        }
    }
    if missing.is_none() && i == small.len() {
        Some(big[i])
    } else {
        missing
    }
}

fn add(digit: &[u8], new: u8) -> Vec<u8> {
    let mut out = Vec::from(digit);
    out.push(new);
    out.sort_unstable();
    out
}

fn sub(digit: &[u8], remove: u8) -> Vec<u8> {
    let mut out = Vec::with_capacity(digit.len() - 1);
    for d in digit {
        if *d != remove {
            out.push(*d);
        }
    }
    assert_eq!(out.len() + 1, digit.len());
    out
}

pub fn solve(buf: impl BufRead) -> Result<(usize, usize)> {
    let mut unique = 0;
    let mut total = 0;
    for line in buf.split(b'\n') {
        let (patterns, outputs) = parse_line(&*line?)?;

        let mut idxs = vec![99; 10]; // digit to pattern index
        let mut has5 = vec![];
        let mut has6 = vec![];
        for (i, p) in patterns.iter().enumerate() {
            match p.len() {
                2 => idxs[1] = i,
                3 => idxs[7] = i,
                4 => idxs[4] = i,
                5 => has5.push(i),
                6 => has6.push(i),
                7 => idxs[8] = i,
                nr => bail!("Encountered invalid digit that uses {} segments.", nr),
            }
        }

        // Deduce top segment from 1 and 7
        let top_seg = single_diff(&patterns[idxs[7]], &patterns[idxs[1]]).unwrap();

        // Find 9 by combining 4 with top segment and looking for letter with single diff
        let nine_without_bottom = add(&patterns[idxs[4]], top_seg);
        let mut bottom_seg = 0;
        for i in &has6 {
            if let Some(s) = single_diff(&patterns[*i], &nine_without_bottom) {
                bottom_seg = s;
                idxs[9] = *i;
                break;
            }
        }
        if bottom_seg == 0 {
            bail!("Could not find digit 9 in input.");
        }

        // Find 3 by combining 7 + bottom segment and looking for letter with single diff
        let three_without_middle = add(&patterns[idxs[7]], bottom_seg);
        let mut middle_seg = 0;
        for i in &has5 {
            if let Some(s) = single_diff(&patterns[*i], &three_without_middle) {
                middle_seg = s;
                idxs[3] = *i;
                break;
            }
        }
        if middle_seg == 0 {
            bail!("Could not find digit 3 in input.");
        }

        // Find 0 by removing middle from 8
        let zero = sub(&patterns[idxs[8]], middle_seg);

        // Find 6 as the remaining 6-segment digit
        for &i in &has6 {
            if i != idxs[9] && patterns[i] != zero {
                idxs[6] = i;
            }
        }
        if idxs[6] == 99 {
            bail!("Could not find digit 6 in input.");
        }

        // Find upper right segment as difference of 8 and 6
        let upper_right_seg = single_diff(&patterns[idxs[8]], &patterns[idxs[6]]).unwrap();

        // Find 5 as 9 minus upper right
        let five = sub(&patterns[idxs[9]], upper_right_seg);

        // Find 2 as the remaining 5-segment digit
        for &i in &has5 {
            if i != idxs[3] && patterns[i] != five {
                idxs[2] = i;
            }
        }
        if idxs[2] == 99 {
            bail!("Could not find digit 2 in input.");
        }

        let map = HashMap::from([
            (zero, 0),
            (patterns[idxs[1]].clone(), 1),
            (patterns[idxs[2]].clone(), 2),
            (patterns[idxs[3]].clone(), 3),
            (patterns[idxs[4]].clone(), 4),
            (five, 5),
            (patterns[idxs[6]].clone(), 6),
            (patterns[idxs[7]].clone(), 7),
            (patterns[idxs[8]].clone(), 8),
            (patterns[idxs[9]].clone(), 9),
        ]);

        let mut number = 0;
        for digit in outputs {
            let val = map[&digit];
            number *= 10;
            number += val;

            if [2, 3, 4, 7].contains(&digit.len()) {
                unique += 1;
            }
        }
        total += number;
    }

    Ok((unique, total))
}
