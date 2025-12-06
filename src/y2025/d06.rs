use anyhow::Result;
use itertools::Itertools;

pub fn solve(input: &str) -> Result<(i64, i64)> {
    Ok((
        solve_problems(parse_a(input)),
        solve_problems(parse_b(input)),
    ))
}

fn solve_problems(problems: Vec<(Vec<i64>, Op)>) -> i64 {
    problems
        .into_iter()
        .map(|(nums, op)| nums.into_iter().reduce(|a, b| op.apply(a, b)).unwrap())
        .sum()
}

#[derive(PartialEq, Eq, Debug)]
enum Op {
    Plus,
    Times,
}

impl Op {
    pub fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Plus => a + b,
            Op::Times => a * b,
        }
    }
}

fn parse_a(input: &str) -> Vec<(Vec<i64>, Op)> {
    let mut lines = input.lines();
    let line = lines.next().expect("first line missing");

    let mut inputs: Vec<_> = line
        .split_whitespace()
        .map(|num| {
            let n = num.parse::<i64>().expect("cannot parse number");
            vec![n]
        })
        .collect();

    for line in lines {
        if line.trim_start().chars().next().unwrap().is_numeric() {
            for (i, num) in line.split_whitespace().enumerate() {
                let n = num.parse::<i64>().expect("cannot parse number");
                inputs[i].push(n);
            }
        } else {
            return inputs.into_iter().zip(parse_operations(line)).collect();
        }
    }
    unreachable!("should have returned in last loop iteration")
}

fn parse_b(input: &str) -> Vec<(Vec<i64>, Op)> {
    let mut num_lines = vec![];
    let mut operations = None;

    for line in input.lines() {
        if line.trim_start().starts_with(&['+', '*']) {
            operations = Some(parse_operations(line));
            break;
        } else {
            num_lines.push(line.as_bytes());
        }
    }

    let operations = operations.expect("no operations line found");

    let mut nums = vec![];
    for i in 0..num_lines[0].len() {
        let mut n = 0;
        for l in &num_lines {
            let char = l[i];
            match char {
                b' ' => {}
                b'0'..=b'9' => {
                    n *= 10;
                    n += i64::from(char - b'0');
                }
                _ => panic!("unexpected char {char}"),
            }
        }

        nums.push(n);
    }
    let groups = nums.into_iter().group_by(|n| *n != 0);

    let nums = groups
        .into_iter()
        .filter_map(|(keep, group)| keep.then_some(group))
        .map(|group| group.collect::<Vec<_>>());

    nums.zip(operations).collect()
}

fn parse_operations(line: &str) -> impl IntoIterator<Item = Op> + use<'_> {
    line.split_whitespace().map(|c| {
        if c == "+" {
            Op::Plus
        } else if c == "*" {
            Op::Times
        } else {
            panic!("unknown {c}")
        }
    })
}
