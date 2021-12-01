use anyhow::Result;
use std::{collections::VecDeque, fmt::Display, io::BufRead};

use crate::io::parse_iter;

#[derive(PartialEq, Eq, Debug)]
struct State {
    pub count: i32,
    previous: VecDeque<i32>,
    sum: i32,
    window_size: usize,
}

impl State {
    pub fn new(window_size: usize) -> State {
        State {
            count: 0,
            previous: VecDeque::with_capacity(window_size + 1),
            sum: 0,
            window_size,
        }
    }

    pub fn update(mut self, next: i32) -> State {
        self.previous.push_back(next);
        let mut newsum = self.sum + next;

        if self.previous.len() > self.window_size {
            if self.previous.len() > self.window_size {
                newsum -= self.previous.pop_front().unwrap_or(0);
            }

            if newsum > self.sum {
                self.count += 1;
            }
        }

        self.sum = newsum;
        self
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Solution {
    pub raw: i32,
    pub windowed: i32,
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Year 2021 Day 02")?;
        writeln!(f, "  Raw: {}", self.raw)?;
        writeln!(f, "  Windowed: {}", self.windowed)
    }
}

pub fn solve(buf: impl BufRead) -> Result<Solution> {
    let final_state = parse_iter::<_, i32>(buf, b'\n').try_fold(
        [State::new(1), State::new(3)],
        |[s1, s3], n| -> Result<[State; 2]> {
            let n = n?;
            Ok([s1.update(n), s3.update(n)])
        },
    )?;

    Ok(Solution {
        raw: final_state[0].count,
        windowed: final_state[1].count,
    })
}
