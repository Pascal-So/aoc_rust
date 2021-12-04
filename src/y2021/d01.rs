use anyhow::Result;
use std::{collections::VecDeque, io::BufRead};

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

pub fn solve(buf: impl BufRead) -> Result<(i32, i32)> {
    let final_state = parse_iter::<_, i32>(buf, b'\n', false).try_fold(
        [State::new(1), State::new(3)],
        |[s1, s3], n| -> Result<[State; 2]> {
            let n = n?;
            Ok([s1.update(n), s3.update(n)])
        },
    )?;

    Ok((final_state[0].count, final_state[1].count))
}
