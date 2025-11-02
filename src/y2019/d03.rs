#[macro_use]
extern crate anyhow;
use std::{
    cmp,
    io::{self, BufRead},
};

fn main() {
    let input: Vec<[Vec<Line>; 2]> = io::stdin()
        .lock()
        .lines()
        .take(2)
        .map(|line| {
            let parsed = parse(&line.unwrap()).unwrap();
            let mut vh_edges = moves_to_edges(parsed);
            for edges in &mut vh_edges {
                edges.sort_unstable();
            }

            vh_edges.into()
        })
        .collect();

    let [va, ha] = &input[0];
    let [vb, hb] = &input[1];

    let mut closest = None;

    find_parallel_intersections(va, vb, &mut closest);
    find_parallel_intersections(ha, hb, &mut closest);

    println!("{:?}", closest);
}

fn update_closest(closest: &mut Option<i32>, dist: i32) {
    match closest {
        Some(v) => {
            *closest = Some(cmp::min(*v, dist));
        }
        None => {
            *closest = Some(dist);
        }
    };
}

fn find_orthogonal_intersections(a: &Vec<Line>, b: &Vec<Line>, closest: &mut Option<i32>) {}

fn find_parallel_intersections(a: &Vec<Line>, b: &Vec<Line>, closest: &mut Option<i32>) {
    let mut i_a = 0;
    let mut i_b = 0;

    while i_a < a.len() && i_b < b.len() {
        if a[i_a].offset < b[i_b].offset {
            i_a += 1;
            continue;
        }
        if a[i_a].offset > b[i_b].offset {
            i_b += 1;
            continue;
        }

        let offset = a[i_a].offset;
        if let Some(cl) = *closest {
            if cl <= offset.abs() {
                if offset > 0 {
                    return;
                } else {
                    while i_a < a.len() && a[i_a].offset == offset {
                        i_a += 1;
                    }
                    continue;
                }
            }
        }

        while i_a < a.len() && a[i_a].offset == offset {
            if let Some(cl) = *closest {
                if cl <= a[i_a].start || cl <= -a[i_a].end {
                    continue;
                }
            }

            let start_b = i_b;

            while i_b < b.len() {
                let start = cmp::max(a[i_a].start, b[i_b].start);
                let end = cmp::min(a[i_a].end, b[i_b].end);

                if start >= end {
                    if start * end <= 0 {
                        update_closest(closest, a[i_a].offset);
                    } else {
                        // let dist = distance(cmp::min(start.abs(), end.abs()), a[i_a].offset);
                        // update_closest(closest, dist);
                    }
                }

                i_b += 1;
            }

            i_b = start_b;
            i_a += 1;
        }
    }
}

// fn find_parallel_intersections(a: &Vec<Line>, b: &Vec<Line>, closest: &mut Option<i32>) {
//     let mut it_a = a.iter().peekable();
//     let mut it_b = b.iter().peekable();

//     loop {
//         let mut peek_a = it_a.peek();
//         let mut peek_b = it_b.peek();

//         while let (Some(line_a), Some(line_b)) = &(peek_a, peek_b) {
//             use std::cmp::Ordering::*;
//             match line_a.offset.cmp(&line_b.offset) {
//                 Less => {
//                     it_a.next();
//                     peek_a = it_a.peek();
//                 },
//                 Equal => {
//                     break;
//                 },
//                 Greater => {
//                     it_b.next();
//                     peek_b = it_b.peek();
//                 },
//             }
//         }

//         if peek_a.is_none() || peek_b.is_none() {
//             break;
//         }

//         let start_b = it_b.clone();
//         loop {
//             let line_a = it_a.next();

//             if let Some(cl) = *closest {
//                 if cl <= line_a.offset.abs() {
//                     break;
//                 }
//                 if cl <= line_a.start ||
//                    cl <= -line_a.end {
//                     continue;
//                 }
//             }

//             it_b = start_b.clone();
//         }
//     }

// for (first, second) of a.iter().merge(b.iter()).tuple_windows() {

// }

// fn next(&mut self) -> Option<Self::Item> {
//     let less_than = match self.fused {
//         Some(lt) => lt,
//         None => match (self.a.peek(), self.b.peek()) {
//             (Some(a), Some(b)) => self.cmp.merge_pred(a, b),
//             (Some(_), None) => {
//                 self.fused = Some(true);
//                 true
//             }
//             (None, Some(_)) => {
//                 self.fused = Some(false);
//                 false
//             }
//             (None, None) => return None,
//         }
//     };
//     if less_than {
//         self.a.next()
//     } else {
//         self.b.next()
//     }
// }

// }

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Line {
    pub offset: i32,
    pub start: i32,
    pub end: i32,
}

fn parse(input: &str) -> anyhow::Result<Vec<(char, i32)>> {
    use nom::{
        character::complete::{char, digit1, one_of},
        combinator::{all_consuming, map_res},
        multi::separated_list1,
        sequence::tuple,
    };

    let direction = one_of("UDLR");
    let number = map_res(digit1, |n: &str| n.parse::<i32>());

    let parser = separated_list1(char(','), tuple((direction, number)));

    let res: nom::IResult<_, _> = all_consuming(parser)(input);
    return match res {
        Ok((_, val)) => Ok(val),
        Err(err) => Err(anyhow!("{}", err)),
    };
}

fn moves_to_edges(moves: Vec<(char, i32)>) -> [Vec<Line>; 2] {
    let mut current_x = 0;
    let mut current_y = 0;

    let mut edges_vertical = vec![];
    let mut edges_horizontal = vec![];

    for (c, dist) in moves {
        match c {
            'U' => {
                edges_vertical.push(Line {
                    offset: current_x,
                    start: current_y,
                    end: current_y + dist,
                });
                current_y += dist;
            }
            'D' => {
                edges_vertical.push(Line {
                    offset: current_x,
                    start: current_y - dist,
                    end: current_y,
                });
                current_y -= dist;
            }
            'L' => {
                edges_horizontal.push(Line {
                    offset: current_y,
                    start: current_x - dist,
                    end: current_x,
                });
                current_x -= dist;
            }
            'R' => {
                edges_horizontal.push(Line {
                    offset: current_y,
                    start: current_x,
                    end: current_x + dist,
                });
                current_x += dist;
            }
            _ => panic!("impossible"),
        };
    }

    [edges_vertical, edges_horizontal]
}
