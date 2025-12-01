use anyhow::Result;

#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

fn parse(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let dir = if &line[..1] == "R" {
                Direction::Right
            } else {
                Direction::Left
            };
            let num = line[1..].parse::<i32>().unwrap();
            Some((dir, num))
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let mut pos = 50;
    let mut a = 0;
    let mut b = 0;

    for (dir, mut num) in parse(input) {
        b += num / 100;
        num %= 100;

        match dir {
            Direction::Left => {
                if num >= pos && pos > 0 {
                    b += 1;
                }

                num *= -1;
            }
            Direction::Right => {
                if num + pos >= 100 {
                    b += 1;
                }
            }
        }

        pos = (pos + num + 100) % 100;
        if pos == 0 {
            a += 1;
        }
    }
    Ok((a, b))
}
