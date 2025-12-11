use std::{collections::VecDeque, i64};

use anyhow::Result;
use microlp::{ComparisonOp, OptimizationDirection, Problem};

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let machines = parse(input);

    // solve part 1
    let mut total_config_presses = 0;
    for machine in &machines {
        let best =
            bfs(0, machine.indicator_target, &machine.buttons_masks).expect("unsolvable machine");
        total_config_presses += best;
    }

    // solve part 2
    let mut total_joltage_presses = 0;
    for machine in machines {
        let mut problem = Problem::new(OptimizationDirection::Minimize);

        let vars: Vec<_> = (0..machine.buttons_indices.len())
            .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
            .collect();

        let mut matrix = Vec::with_capacity(machine.joltages.len());
        for _ in 0..machine.joltages.len() {
            matrix.push(vec![0; machine.buttons_indices.len()]);
        }

        for (j, button) in machine.buttons_indices.iter().enumerate() {
            for &i in button {
                matrix[i][j] = 1;
            }
        }

        for (i, joltage) in machine.joltages.iter().enumerate() {
            let terms: Vec<_> = matrix[i]
                .iter()
                .enumerate()
                .map(|(j, coeff)| (vars[j], f64::from(*coeff)))
                .collect();
            problem.add_constraint(&terms, ComparisonOp::Eq, f64::from(*joltage));
        }

        let solution = problem.solve().unwrap();
        let objective = solution.objective();
        total_joltage_presses += objective.round() as i64;
    }

    Ok((total_config_presses, total_joltage_presses))
}

fn bfs(start: u16, target: u16, buttons: &[u16]) -> Option<i64> {
    // we make the assumption that there's never more than 10 lights/counters per machine
    let mut visited = vec![false; 1 << 10];
    let mut queue = VecDeque::new();

    visited[0] = true;
    queue.push_back((start, 0));

    while let Some((current, dist)) = queue.pop_front() {
        if current == target {
            return Some(dist);
        }

        for button in buttons {
            let next = current ^ button;

            if !visited[next as usize] {
                visited[next as usize] = true;
                queue.push_back((next, dist + 1));
            }
        }
    }

    None
}

struct Machine {
    indicator_target: u16,
    buttons_indices: Vec<Vec<usize>>,
    buttons_masks: Vec<u16>,
    joltages: Vec<i32>,
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let (indicators, rest) = line.split_once("]").expect("no ] char in line");
            let mut indicator_target = 0;

            for (i, b) in indicators.as_bytes()[1..].iter().enumerate() {
                assert!(i < 16);
                if *b == b'#' {
                    indicator_target |= 1 << i;
                }
            }

            let (buttons, joltages) = rest.split_once("{").expect("no { char in line");

            let buttons = buttons.split_whitespace();
            let buttons = buttons
                .into_iter()
                .map(|btn| {
                    let btn = btn.strip_prefix("(").expect("button does not start with (");
                    let btn = btn.strip_suffix(")").expect("button does not end with )");
                    btn
                })
                .collect::<Vec<_>>();

            let buttons_masks = buttons
                .iter()
                .map(|btn| {
                    let mut bitmap = 0;
                    for idx in btn.split(",") {
                        let idx: usize = idx.parse().expect("cannot parse button index");
                        bitmap |= 1 << idx;
                    }
                    bitmap
                })
                .collect();
            let buttons_indices = buttons
                .iter()
                .map(|btn| {
                    btn.split(",")
                        .map(|idx| idx.parse::<usize>().expect("cannot parse button index"))
                        .collect()
                })
                .collect();

            let joltages = joltages
                .strip_suffix("}")
                .expect("joltages list does not end with }")
                .split(",")
                .into_iter()
                .map(|j| j.parse().unwrap())
                .collect();
            Machine {
                indicator_target,
                joltages,
                buttons_indices,
                buttons_masks,
            }
        })
        .collect()
}
