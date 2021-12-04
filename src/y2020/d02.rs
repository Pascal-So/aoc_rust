use std::io::BufRead;

use anyhow::{anyhow, Result};

use ::nom::Finish;
mod nom {
    pub use ::nom::{
        bytes::complete::tag,
        character::complete::{anychar, char},
        combinator::rest,
        number::complete::float,
        IResult,
    };
}

#[derive(PartialEq, Eq, Debug)]
struct Rule(usize, usize, char, String);

fn rule_parser(i: &str) -> nom::IResult<&str, Rule> {
    let (i, fst) = nom::float(i)?;
    let (i, _) = nom::char('-')(i)?;
    let (i, snd) = nom::float(i)?;
    let (i, _) = nom::char(' ')(i)?;
    let (i, c) = nom::anychar(i)?;
    let (i, _) = nom::tag(": ")(i)?;
    let (i, pwd) = nom::rest(i)?;

    Ok((i, Rule(fst as usize, snd as usize, c, pwd.to_string())))
}

fn valid_for_sled_rental(Rule(lower, upper, c, pwd): &Rule) -> bool {
    let count = pwd.chars().filter(|x| x == c).count();
    *lower <= count && count <= *upper
}

fn valid_for_toboggan_rental(Rule(fst, snd, c, pwd): &Rule) -> bool {
    let oc = Some(*c);
    (pwd.chars().nth(*fst - 1) == oc) != (pwd.chars().nth(*snd - 1) == oc)
}

pub fn solve(buf: impl BufRead) -> Result<(i32, i32)> {
    buf.lines()
        .map(|line| -> Result<Rule> {
            let rule = rule_parser(&line?)
                .finish()
                .map_err(|e| anyhow!("Cannot parse password line: {}", e))?
                .1;
            Ok(rule)
        })
        .try_fold((0, 0), |(count_sled, count_toboggan), rule| {
            let rule = rule?;
            Ok((
                count_sled + valid_for_sled_rental(&rule) as i32,
                count_toboggan + valid_for_toboggan_rental(&rule) as i32,
            ))
        })
}
