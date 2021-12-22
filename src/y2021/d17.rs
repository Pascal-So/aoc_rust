use std::io::BufRead;

use anyhow::{anyhow, Result};

use crate::io::get_first_line_string;

type Range = (i32, i32);
type Region = (Range, Range);

/// Takes a string like "-5..32" and returns (-5, 32)
fn parse_range(text: &str) -> Result<Range> {
    let (lower, upper) = text.split_once("..").ok_or_else(|| {
        anyhow!(
            "Invalid format, did not find separator \"..\" in \"{}\".",
            text
        )
    })?;

    Ok((lower.parse()?, upper.parse()?))
}

fn parse(line: &str) -> Result<Region> {
    let (x_str, y_str) = line[15..]
        .split_once(", y=")
        .ok_or_else(|| anyhow!("Invalid format, did not find separator \", y=\" in input."))?;

    let x_range = parse_range(x_str)?;
    let y_range = parse_range(y_str)?;

    Ok((x_range, y_range))
}

/// x position after `steps` for a probe with given x `vel`.
fn x_dist(vel: i32, steps: i32) -> i32 {
    if steps >= vel {
        vel * (vel + 1) / 2
    } else {
        vel * steps - steps * (steps - 1) / 2
    }
}

fn y_dist(vel: i32, steps: i32) -> i32 {
    vel * steps - steps * (steps - 1) / 2
}

pub fn solve(buf: impl BufRead) -> Result<(i32, i32)> {
    let (x_range, y_range) = parse(&get_first_line_string(buf)?)?;
    assert!(x_range.0 > 0);
    assert!(y_range.1 < 0);

    let mut sum = 0;

    let mut first_hit = 1;
    let mut last_hit = 1;

    for vx in (1..=x_range.1).rev() {
        if x_dist(vx, vx) < x_range.0 {
            // Can't reach target region at this velocity
            break;
        }

        while first_hit < vx && x_dist(vx, first_hit) < x_range.0 {
            first_hit += 1;
        }
        while last_hit < vx && x_dist(vx, last_hit + 1) <= x_range.1 {
            last_hit += 1;
        }

        if last_hit >= vx {
            last_hit = -y_range.0 * 2;
        }

        let mut last_upper = i32::MIN / 2;
        for time in first_hit..=last_hit {
            let offset = time * (time - 1) / 2;
            let lower = (y_range.0 + offset + time - 1).div_euclid(time);
            let upper = (y_range.1 + offset).div_euclid(time);

            sum += upper - lower + 1 - (last_upper - lower + 1).max(0);
            last_upper = upper;
        }
    }

    let max_vy = -y_range.0 - 1;
    Ok((y_dist(max_vy, max_vy), sum))
}
