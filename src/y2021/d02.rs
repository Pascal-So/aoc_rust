use std::io::BufRead;

use anyhow::{bail, Context, Result};

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(PartialEq, Eq, Debug)]
struct Command {
    pub direction: Direction,
    pub units: i64,
}

impl Command {
    pub fn new(d: Direction, u: i64) -> Command {
        Command {
            direction: d,
            units: u,
        }
    }

    pub fn parse(line: &[u8]) -> Result<Command> {
        use Direction::*;

        if line.is_empty() {
            bail!("Empty move command!");
        }

        let get_distance = |prefix: &[u8]| -> Result<i64> {
            Ok(std::str::from_utf8(&line[prefix.len()..])?.parse()?)
        };

        Ok(match line[0] {
            b'f' => Command::new(Forward, get_distance(b"forward ")?),
            b'd' => Command::new(Down, get_distance(b"down ")?),
            b'u' => Command::new(Up, get_distance(b"up ")?),
            _ => bail!("Invalid command {}", std::str::from_utf8(line)?),
        })
    }
}

#[derive(Debug)]
struct State {
    horizontal: i64,
    vertical: i64,
    aim: i64,
}

impl State {
    pub fn new() -> State {
        State {
            horizontal: 0,
            vertical: 0,
            aim: 0,
        }
    }

    pub fn normal_advance(mut self, cmd: &Command) -> State {
        use Direction::*;
        match &cmd.direction {
            Up => self.vertical -= cmd.units,
            Down => self.vertical += cmd.units,
            Forward => self.horizontal += cmd.units,
        }

        self
    }

    pub fn aimed_advance(mut self, cmd: &Command) -> State {
        use Direction::*;
        match &cmd.direction {
            Up => self.aim -= cmd.units,
            Down => self.aim += cmd.units,
            Forward => {
                self.horizontal += cmd.units;
                self.vertical += self.aim * cmd.units;
            }
        }

        self
    }

    pub fn product(&self) -> i64 {
        self.horizontal * self.vertical
    }
}

pub fn solve(buf: impl BufRead) -> Result<(i64, i64)> {
    let (normal, aimed) = buf
        .split(b'\n')
        .map(|line| -> Result<Command> { Command::parse(&line.context("invalid line in input")?) })
        .try_fold(
            (State::new(), State::new()),
            |(normal, aimed), cmd: Result<Command>| -> Result<(State, State)> {
                let cmd = cmd?;
                Ok((normal.normal_advance(&cmd), aimed.aimed_advance(&cmd)))
            },
        )?;

    Ok((normal.product(), aimed.product()))
}
