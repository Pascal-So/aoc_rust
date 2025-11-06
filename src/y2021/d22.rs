use std::ops::RangeInclusive;

use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, one_of, space1},
    combinator::{all_consuming, opt, value},
    IResult,
};

use crate::octtree::Octtree;

// on x=-20..26,y=-36..17,z=-47..7
#[derive(PartialEq, Eq, Debug)]
struct Instruction {
    fill: bool,
    bounds: [RangeInclusive<i64>; 3],
}

impl Instruction {
    pub fn is_small(&self) -> bool {
        let range = -50..=50;
        range.contains(self.bounds[0].start())
            && range.contains(self.bounds[0].end())
            && range.contains(self.bounds[1].start())
            && range.contains(self.bounds[1].end())
            && range.contains(self.bounds[2].start())
            && range.contains(self.bounds[2].end())
    }
}

fn number(input: &str) -> IResult<&str, i64> {
    let (input, sign) = opt(tag("-"))(input)?;
    let (input, digits) = digit1(input)?;
    let mut num = digits.parse().unwrap();
    if sign.is_some() {
        num *= -1;
    }

    Ok((input, num))
}
fn range(input: &str) -> IResult<&str, RangeInclusive<i64>> {
    let (input, _) = one_of("xyz")(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, lower) = number(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, upper) = number(input)?;
    Ok((input, lower..=upper))
}
fn line(input: &str) -> IResult<&str, Instruction> {
    let (input, on) = alt((value(true, tag("on")), value(false, tag("off"))))(input)?;
    let (input, _) = space1(input)?;
    let (input, x) = range(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = range(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = range(input)?;

    Ok((
        input,
        Instruction {
            fill: on,
            bounds: [x, y, z],
        },
    ))
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|l| all_consuming(line)(l).unwrap().1)
        .collect()
}

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let instructions = parse(input);
    let mut split_points = [vec![], vec![], vec![]];
    for instruction in &instructions {
        for i in 0..3 {
            split_points[i].push(*instruction.bounds[i].start());
            split_points[i].push(*instruction.bounds[i].end() + 1);
        }
    }

    for sp in &mut split_points {
        sp.sort();
        *sp = sp.drain(..).unique().collect();
    }
    let mut tree = Octtree::new(split_points);

    let mut result_a = None;

    for instruction in instructions {
        if !instruction.is_small() && result_a.is_none() {
            result_a = Some(tree.nr_filled());
        }
        tree.set_region(instruction.bounds, instruction.fill)
            .unwrap();
    }
    let result_b = tree.nr_filled();
    Ok((result_a.unwrap_or(result_b), result_b))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let test = "on x=-20..26,y=-36..17,z=-47..7";
        let res = parse(test);
        assert_eq!(
            res,
            vec![Instruction {
                bounds: [-20..=26, -36..=17, -47..=7],
                fill: true
            }]
        );
    }
}
