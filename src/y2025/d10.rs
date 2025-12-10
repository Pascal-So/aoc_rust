use std::collections::VecDeque;

use anyhow::Result;

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
    for machine in machines {}

    Ok((total_config_presses, total_joltage_presses))
}

fn bfs(start: u16, target: u16, buttons: &[u16]) -> Option<i64> {
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

// we make the assumption that there's never more than 10 lights/counters per machine
type CounterGroup = [u16; 10];
struct Machine {
    indicator_target: u16,
    buttons_indices: Vec<Vec<usize>>,
    buttons_masks: Vec<u16>,
    joltages: CounterGroup,
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

            let joltages_str = joltages
                .strip_suffix("}")
                .expect("joltages list does not end with }");
            let mut joltages = [0; 10];
            for (i, joltage) in joltages_str.split(",").enumerate() {
                joltages[i] = joltage.parse().unwrap();
            }

            Machine {
                indicator_target,
                joltages,
                buttons_indices,
                buttons_masks,
            }
        })
        .collect()
}
