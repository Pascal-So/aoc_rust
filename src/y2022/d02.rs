use anyhow::Result;

#[derive(Clone, Copy)]
#[repr(i32)]
enum Move {
    Rock = 0,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(i32)]
enum Outcome {
    Loss = 0,
    Draw,
    Win,
}

impl Outcome {
    pub fn score(self) -> i32 {
        self as i32 * 3
    }

    pub fn required_move_against(self, other: Move) -> Move {
        let increment = match self {
            Outcome::Draw => 0,
            Outcome::Win => 1,
            Outcome::Loss => 2,
        };

        match (other as i32 + increment) % 3 {
            0 => Move::Rock,
            1 => Move::Paper,
            _ => Move::Scissors,
        }
    }
}

impl Move {
    pub fn battle(&self, other: Move) -> Outcome {
        let a = self.score();
        let b = other.score();

        if a == b {
            Outcome::Draw
        } else if a % 3 + 1 == b {
            Outcome::Loss
        } else {
            Outcome::Win
        }
    }

    pub fn score(self) -> i32 {
        self as i32 + 1
    }

    pub fn battlescore(&self, other: Move) -> i32 {
        self.battle(other).score() + self.score()
    }
}

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let mut score_a = 0;
    let mut score_b = 0;

    let mut opponent = Move::Rock;
    let mut me = Move::Rock; // value for part A
    let mut desired_outcome = Outcome::Draw; // value for part B
    let mut line_end = false;

    for b in input.bytes() {
        match b {
            b'A' => opponent = Move::Rock,
            b'B' => opponent = Move::Paper,
            b'C' => opponent = Move::Scissors,
            b'X' => {
                me = Move::Rock;
                desired_outcome = Outcome::Loss;
                line_end = true;
            }
            b'Y' => {
                me = Move::Paper;
                desired_outcome = Outcome::Draw;
                line_end = true;
            }
            b'Z' => {
                me = Move::Scissors;
                desired_outcome = Outcome::Win;
                line_end = true;
            }
            _ => {}
        }

        if line_end {
            score_a += me.battlescore(opponent);

            let my_move_b = desired_outcome.required_move_against(opponent);
            score_b += my_move_b.battlescore(opponent);

            line_end = false;
        }
    }

    Ok((score_a, score_b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battle() {
        assert_eq!(Move::Rock.battle(Move::Scissors), Outcome::Win);
        assert_eq!(Move::Paper.battle(Move::Scissors), Outcome::Loss);
    }
}
