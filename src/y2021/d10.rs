use std::io::BufRead;

use anyhow::{bail, Result};

fn bracket_type(c: u8) -> Result<(bool, u8)> {
    Ok(match c {
        b'(' => (true, 1),
        b')' => (false, 1),
        b'[' => (true, 2),
        b']' => (false, 2),
        b'{' => (true, 3),
        b'}' => (false, 3),
        b'<' => (true, 4),
        b'>' => (false, 4),
        _ => bail!("Invalid bracket: '{}'", c as char),
    })
}

fn calc_corruption_score(t: u8) -> i64 {
    match t {
        1 => 3,
        2 => 57,
        3 => 1197,
        4 => 25137,
        _ => panic!("internal error, invalid bracket type {}", t),
    }
}

fn calc_completion_score(stack: &[u8]) -> i64 {
    stack
        .iter()
        .rev()
        .fold(0, |last, curr| last * 5 + *curr as i64)
}

pub fn solve(buf: impl BufRead) -> Result<(i64, i64)> {
    let mut stack = vec![];
    let mut corruption_score = 0;
    let mut completion_scores = vec![];
    for line in buf.split(b'\n') {
        stack.clear();
        let mut corrupted = false;
        for c in line? {
            let (open, btype) = bracket_type(c)?;
            if open {
                stack.push(btype);
            } else if stack.pop() != Some(btype) {
                corruption_score += calc_corruption_score(btype);
                corrupted = true;
                break;
            }
        }

        if !corrupted {
            completion_scores.push(calc_completion_score(&stack));
        }
    }

    completion_scores.sort_unstable();

    Ok((
        corruption_score,
        completion_scores[completion_scores.len() / 2],
    ))
}
