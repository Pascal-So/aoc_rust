use std::collections::{hash_map::Entry, HashMap};

use anyhow::{anyhow, bail, Context, Result};

use crate::io;

#[derive(Debug)]
struct Boards {
    /// Counting the number of drawn numbers in this row or column, indexed
    /// by the rowcol index.
    pub count: Vec<usize>,

    /// Mapping from a bingo number to the rowcol indices where this
    /// number occurs.
    pub occurrences: HashMap<i32, Vec<usize>>,

    /// Drawn bingo numbers in order.
    pub numbers: Vec<i32>,

    /// Side length of a board.
    pub length: usize,

    /// Remaining sum of each board.
    pub sums: Vec<i32>,
}

fn parse(input: &str) -> Result<Boards> {
    let mut lines = input.lines();
    let first_line = lines.next().context("no first line in input")?;
    let numbers = io::parse_entries(first_line, ',')?;

    let mut board_index = 0;
    let mut column_index = 0;
    let mut occurrences = HashMap::<i32, Vec<usize>>::new();
    let mut length = None;
    let mut sums = Vec::new();
    let mut board_sum = 0;

    for line in lines {
        if line.is_empty() {
            if length.is_some() {
                board_index += 1;
                column_index = 0;
                sums.push(board_sum);
                board_sum = 0;
            }
            continue;
        }

        let nums = io::parse_vec(line.as_bytes(), b' ', true).context("Cannot parse board line")?;
        if let Some(len) = length {
            if len != nums.len() {
                bail!(
                    "Board lines of incosistend lengths, first {} then {}!",
                    len,
                    nums.len()
                );
            }
        } else {
            length = Some(nums.len());
        }

        for (i, &n) in nums.iter().enumerate() {
            board_sum += n;
            let row = board_index * nums.len() * 2 + i + nums.len();
            let col = board_index * nums.len() * 2 + column_index;
            match occurrences.entry(n) {
                Entry::Occupied(mut e) => e.get_mut().append(&mut vec![row, col]),
                Entry::Vacant(e) => {
                    e.insert(vec![row, col]);
                }
            }
        }

        column_index += 1;
    }
    sums.push(board_sum);

    let length: usize = length.ok_or_else(|| anyhow!("No boards given!"))?;
    let total_nr_rows_and_cols: usize = (board_index + 1) * length * length;

    Ok(Boards {
        count: vec![0; total_nr_rows_and_cols],
        occurrences,
        numbers,
        length,
        sums,
    })
}

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let mut boards = parse(input)?;
    let mut finished = vec![false; boards.sums.len()];

    let mut first_finisher_score = None;
    let mut last_finisher_score = None;

    for n in boards.numbers.iter() {
        for (occurrence_id, &rowcol_id) in boards.occurrences[n].iter().enumerate() {
            let board_id = rowcol_id / (boards.length * 2);
            if finished[board_id] {
                continue;
            }

            boards.count[rowcol_id] += 1;

            // If a board contains a number then the cell belongs to both
            // a row and a column. Make sure that we don't subtract the number
            // twice.
            if occurrence_id % 2 == 0 {
                boards.sums[board_id] -= n;
            }

            if boards.count[rowcol_id] >= boards.length {
                finished[board_id] = true;
                let score = boards.sums[board_id] * n;

                if first_finisher_score.is_none() {
                    first_finisher_score = Some(score);
                } else {
                    last_finisher_score = Some(score)
                }
            }
        }
    }

    Ok((
        first_finisher_score.ok_or_else(|| anyhow!("No finisher!"))?,
        last_finisher_score.ok_or_else(|| anyhow!("No second finishers!"))?,
    ))
}
