use std::{
    collections::{hash_map::Entry, HashMap},
    io::BufRead,
    iter::successors,
};

use anyhow::{anyhow, Result};
mod nom {
    pub use ::nom::{
        bytes::complete::tag,
        character::complete::{char, i32},
        sequence::separated_pair,
        IResult,
    };
}

type Point = (i32, i32);
type Line = (Point, Point);

fn parse_point(i: &[u8]) -> nom::IResult<&[u8], Point, ()> {
    nom::separated_pair(nom::i32, nom::tag(b","), nom::i32)(i)
}

fn parse_line(i: &[u8]) -> Result<Line> {
    let mut line = nom::separated_pair(parse_point, nom::tag(b" -> "), parse_point);
    Ok(line(i)?.1)
}

pub fn parse(buf: impl BufRead) -> Result<Vec<Line>> {
    buf.split(b'\n')
        .zip(1..)
        .map(|(line, idx)| -> Result<Line> {
            parse_line(&line?).map_err(|e| anyhow!("Cannot parse line {}: {}", idx, e))
        })
        .collect()
}

pub fn range(((sx, sy), (ex, ey)): Line) -> impl Iterator<Item = Point> {
    let len = (ex - sx).abs().max((ey - sy).abs()) as usize;
    let step_x = (ex - sx).signum();
    let step_y = (ey - sy).signum();
    successors(Some((sx, sy)), move |(cx, cy)| {
        Some((cx + step_x, cy + step_y))
    })
    .take(len + 1)
}

fn is_axis_aligned(((sx, sy), (ex, ey)): &Line) -> bool {
    sx == ex || sy == ey
}

pub fn solve(buf: impl BufRead) -> Result<(i32, i32)> {
    let mut all_lines = parse(buf)?;
    let nr_axis_aligned = itertools::partition(all_lines.iter_mut(), is_axis_aligned);

    let mut filled = HashMap::new();
    let mut total = 0;
    let mut fill_in = |lines: &[Line]| {
        for line in lines {
            for point in range(*line) {
                total += match filled.entry(point) {
                    Entry::Occupied(mut e) => {
                        let v = e.get_mut();
                        *v += 1;
                        if *v == 2 {
                            1
                        } else {
                            0
                        }
                    }
                    Entry::Vacant(e) => {
                        e.insert(1);
                        0
                    }
                };
            }
        }
        total
    };

    Ok((
        fill_in(&all_lines[..nr_axis_aligned]),
        fill_in(&all_lines[nr_axis_aligned..]),
    ))
}
