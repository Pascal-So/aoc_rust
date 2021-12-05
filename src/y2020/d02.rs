use std::io::BufRead;

use anyhow::{anyhow, Result};

mod nom {
    pub use ::nom::{
        bytes::complete::tag, bytes::complete::take, character::complete::i32, combinator::rest,
        sequence::separated_pair, IResult,
    };
}

#[derive(PartialEq, Eq, Debug)]
struct Rule(usize, usize, u8, Vec<u8>);

fn rule_parser(i: &[u8]) -> nom::IResult<&[u8], Rule> {
    let (i, (fst, snd)) = nom::separated_pair(nom::i32, nom::tag(b"-"), nom::i32)(i)?;
    let (i, _) = nom::tag(b" ")(i)?;
    let (i, c) = nom::take(1_usize)(i)?;
    let (i, _) = nom::tag(b": ")(i)?;
    let (i, pwd) = nom::rest(i)?;

    Ok((i, Rule(fst as usize, snd as usize, c[0], pwd.to_owned())))
}

fn parse_rule(i: &[u8]) -> Result<Rule> {
    Ok(rule_parser(i)
        .map_err(|e| anyhow!("Cannot parse password line: {}", e))?
        .1)
}

fn valid_for_sled_rental(Rule(lower, upper, c, pwd): &Rule) -> bool {
    let count = pwd.iter().filter(|x| *x == c).count();
    *lower <= count && count <= *upper
}

fn valid_for_toboggan_rental(Rule(fst, snd, c, pwd): &Rule) -> bool {
    (pwd[*fst - 1] == *c) != (pwd[*snd - 1] == *c)
}

pub fn solve(buf: impl BufRead) -> Result<(i32, i32)> {
    buf.split(b'\n')
        .map(|line| -> Result<Rule> { parse_rule(&line?) })
        .try_fold((0, 0), |(count_sled, count_toboggan), rule| {
            let rule = rule?;
            Ok((
                count_sled + valid_for_sled_rental(&rule) as i32,
                count_toboggan + valid_for_toboggan_rental(&rule) as i32,
            ))
        })
}
