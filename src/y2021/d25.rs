use anyhow::Result;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Field {
    Down,
    Right,
    Empty,
}

type Board = Vec<Vec<Field>>;

fn parse_input(input: &str) -> Board {
    let mut out = vec![];
    for line in input.lines() {
        let mut outline = vec![];
        for c in line.bytes() {
            let field = match c {
                b'v' => Field::Down,
                b'>' => Field::Right,
                b'.' => Field::Empty,
                _ => continue,
            };

            outline.push(field);
        }
        if !outline.is_empty() {
            out.push(outline);
        }
    }

    for i in 1..out.len() {
        if out[i].len() != out[i - 1].len() {
            panic!(
                "length of line {i} ({}) does not match length of line {} ({})",
                out[i].len(),
                i - 1,
                out[i - 1].len()
            );
        }
    }
    out
}

fn step(board: &mut Board) -> bool {
    let mut has_moved = false;

    let last_x = board[0].len() - 1;
    let last_y = board.len() - 1;

    // horizontal motion
    for line in board.iter_mut() {
        let mut just_moved = false;
        let mut wrap_moved = false;
        for i in 0..line.len() {
            let prev_idx = if i == 0 { last_x } else { i - 1 };
            if !just_moved
                && (i != last_x || !wrap_moved)
                && line[i] == Field::Empty
                && line[prev_idx] == Field::Right
            {
                has_moved = true;
                just_moved = true;
                if i == 0 {
                    wrap_moved = true;
                }
                line.swap(prev_idx, i);
            } else {
                just_moved = false;
            }
        }
    }

    // vertical motion
    for j in 0..board[0].len() {
        let mut just_moved = false;
        let mut wrap_moved = false;
        for i in 0..board.len() {
            let prev_idx = if i == 0 { last_y } else { i - 1 };
            if !just_moved
                && (i != last_y || !wrap_moved)
                && board[i][j] == Field::Empty
                && board[prev_idx][j] == Field::Down
            {
                has_moved = true;
                just_moved = true;
                if i == 0 {
                    wrap_moved = true;
                }
                board[i][j] = Field::Down;
                board[prev_idx][j] = Field::Empty;
            } else {
                just_moved = false;
            }
        }
    }

    has_moved
}

#[allow(unused)]
fn print(board: &Board) {
    for line in board {
        for field in line {
            print!(
                "{}",
                match field {
                    Field::Down => "v",
                    Field::Right => ">",
                    Field::Empty => ".",
                }
            );
        }
        println!();
    }
}

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let mut board = parse_input(input);

    let mut steps = 0;
    while step(&mut board) {
        steps += 1;
    }

    return Ok((steps + 1, 0));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let input = "v.\n>>\n";
        use Field::*;
        let expected = vec![vec![Down, Empty], vec![Right, Right]];

        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_motion() {
        use Field::*;
        let mut board = vec![vec![Right, Empty]];
        assert!(step(&mut board));
        assert_eq!(board, vec![vec![Empty, Right]]);
        assert!(step(&mut board));
        assert_eq!(board, vec![vec![Right, Empty]]);
    }

    #[test]
    fn test_single_step_small_example() {
        let input = r#"
            ..........
            .>v....v..
            .......>..
            ..........
        "#;
        let output = r#"
            ..........
            .>........
            ..v....v>.
            ..........
        "#;
        let mut board_in = parse_input(input);
        let board_out = parse_input(output);

        step(&mut board_in);
        assert_eq!(board_in, board_out);
    }
}
