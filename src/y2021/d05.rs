use std::iter::successors;

use anyhow::{anyhow, Result};

mod nom {
    pub use ::nom::{
        bytes::complete::tag, character::complete::i32, sequence::separated_pair, IResult,
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

pub fn parse(input: &str) -> Result<Vec<Line>> {
    input
        .split('\n')
        .zip(1..)
        .filter(|(line, _)| !line.is_empty())
        .map(|(line, idx)| -> Result<Line> {
            parse_line(line.as_bytes()).map_err(|e| anyhow!("Cannot parse line {}: {}", idx, e))
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

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let mut all_lines = parse(input)?;
    let nr_axis_aligned = itertools::partition(all_lines.iter_mut(), is_axis_aligned);

    let mut maxx = 0;
    let mut maxy = 0;
    for ((sx, sy), (ex, ey)) in &all_lines {
        maxx = maxx.max(*sx);
        maxx = maxx.max(*ex);
        maxy = maxy.max(*sy);
        maxy = maxy.max(*ey);
    }

    let mut field = vec![vec![0_u8; maxy as usize + 1]; maxx as usize + 1];
    let mut total = 0;

    let mut fill_in = |lines: &[Line]| {
        for line in lines {
            for (x, y) in range(*line) {
                let cell = &mut field[x as usize][y as usize];
                *cell += 1;
                if *cell == 2 {
                    total += 1;
                }
            }
        }
        total
    };

    Ok((
        fill_in(&all_lines[..nr_axis_aligned]),
        fill_in(&all_lines[nr_axis_aligned..]),
    ))
}
