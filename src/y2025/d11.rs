use std::collections::HashMap;

use anyhow::Result;

type Node = [u8; 3];
const YOU: Node = [b'y', b'o', b'u'];
const SVR: Node = [b's', b'v', b'r'];
const END: Node = [b'o', b'u', b't'];
const DAC: Node = [b'd', b'a', b'c'];
const FFT: Node = [b'f', b'f', b't'];

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let graph = parse(input);

    Ok((solve_part_1(&graph), solve_part_2(&graph)))
}

fn solve_part_1(graph: &HashMap<Node, Vec<Node>>) -> i64 {
    let mut counts = HashMap::new();
    counts.insert(YOU, 1);

    let mut nr_paths = 0;

    for _ in 0..graph.len() {
        let mut new_counts = HashMap::new();

        for (src, dsts) in graph.iter() {
            if let Some(count) = counts.get(src) {
                for dst in dsts {
                    if *dst == END {
                        nr_paths += count;
                    } else {
                        *new_counts.entry(*dst).or_default() += count;
                    }
                }
            }
        }

        counts = new_counts;
    }

    nr_paths
}
fn solve_part_2(graph: &HashMap<Node, Vec<Node>>) -> i64 {
    let mut counts = HashMap::new();
    counts.insert((SVR, false, false), 1);

    let mut nr_paths = 0;

    for _ in 0..graph.len() {
        let mut new_counts = HashMap::new();

        for (src, dsts) in graph.iter() {
            for has_dac in [false, true] {
                for has_fft in [false, true] {
                    if let Some(count) = counts.get(&(*src, has_dac, has_fft)) {
                        for dst in dsts {
                            if *dst == END {
                                if has_fft && has_dac {
                                    nr_paths += count;
                                }
                            } else {
                                let has_dac = has_dac || *dst == DAC;
                                let has_fft = has_fft || *dst == FFT;
                                *new_counts.entry((*dst, has_dac, has_fft)).or_default() += count;
                            }
                        }
                    }
                }
            }
        }

        counts = new_counts;
    }

    nr_paths
}

fn parse(input: &str) -> HashMap<Node, Vec<Node>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        let (left, right) = line.split_once(":").expect("line is missing : char");

        let node = Node::try_from(left.as_bytes()).unwrap();

        let destinations = right
            .split_whitespace()
            .map(|dst| Node::try_from(dst.as_bytes()).unwrap())
            .collect();

        graph.insert(node, destinations);
    }

    graph
}
