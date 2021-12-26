use std::io::BufRead;

use anyhow::{Context, Result};
use combine::{
    between, choice,
    parser::byte::{byte, digit},
    ParseError, Stream,
};
use itertools::Itertools;

use crate::parse::combine_parse;

macro_rules! number {
    [$a: tt, $b: tt] => {
        Number::Pair(Box::new((number![$a], number![$b])))
    };
    [[$a: tt, $b: tt]] => {
        Number::Pair(Box::new((number![$a], number![$b])))
    };
    [$a: tt] => {
        Number::Regular($a)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Number {
    Regular(u8),
    Pair(Box<(Number, Number)>),
}

impl Number {
    pub fn magnitude(&self) -> i64 {
        match &self {
            Number::Regular(n) => *n as i64,
            Number::Pair(pair) => 3 * pair.0.magnitude() + 2 * pair.1.magnitude(),
        }
    }

    fn add_to(&mut self, right: bool, amount: u8) {
        match self {
            Number::Regular(n) => *n += amount,
            Number::Pair(pair) => {
                if right { &mut pair.1 } else { &mut pair.0 }.add_to(right, amount);
            }
        }
    }

    fn explode(&mut self, depth: u8) -> Option<(Option<u8>, Option<u8>)> {
        match self {
            Number::Regular(_) => None,
            Number::Pair(pair) => {
                if depth == 4 {
                    let l = Some(pair.0.magnitude() as u8);
                    let r = Some(pair.1.magnitude() as u8);
                    *self = number![0];
                    Some((l, r))
                } else if let Some((left, right)) = pair.0.explode(depth + 1) {
                    if let Some(amount) = right {
                        pair.1.add_to(false, amount);
                    }
                    Some((left, None))
                } else if let Some((left, right)) = pair.1.explode(depth + 1) {
                    if let Some(amount) = left {
                        pair.0.add_to(true, amount);
                    }
                    Some((None, right))
                } else {
                    None
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Number::Regular(n) => {
                if *n >= 10 {
                    let l = *n / 2;
                    let r = (*n + 1) / 2;
                    *self = number![l, r];
                    true
                } else {
                    false
                }
            }
            Number::Pair(pair) => pair.0.split() || pair.1.split(),
        }
    }

    pub fn add(self, other: Number) -> Number {
        let mut new = Number::Pair(Box::new((self, other)));
        loop {
            let mut cont = false;
            while new.explode(0).is_some() {
                cont = true;
            }
            if new.split() {
                cont = true;
            }
            if !cont {
                break;
            }
        }

        new
    }
}

parser! {
    fn number[Input]()(Input) -> Number
    where [
        Input: Stream<Token = u8>,
        Input::Error: ParseError<u8, Input::Range, Input::Position>,
        <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError:
            From<::std::num::ParseIntError>,
    ]
    {
        choice((
            digit().map(|b| Number::Regular(b - b'0')),
            between(byte(b'['), byte(b']'), number().skip(byte(b',')).and(number())).map(|pair| Number::Pair(Box::new(pair))),
        ))
    }
}

pub fn solve(buf: impl BufRead) -> Result<(i64, i64)> {
    let numbers = buf
        .split(b'\n')
        .map(|r| combine_parse(number(), &*r?))
        .collect::<Result<Vec<Number>>>()?;

    let max = numbers
        .iter()
        .combinations(2)
        .map(|v| {
            let a = v[0].clone().add(v[1].clone()).magnitude();
            let b = v[1].clone().add(v[0].clone()).magnitude();
            a.max(b)
        })
        .max()
        .context("At least 1 number in input expected")?;

    let sum = numbers
        .into_iter()
        .fold1(Number::add)
        .context("At least 1 number in input expected")?
        .magnitude();

    Ok((sum, max))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[rustfmt::skip]
    fn test_magnitude() {
        assert_eq!(number![[1,2],[[3,4],5]].magnitude(), 143);
        assert_eq!(number![[[[0,7],4],[[7,8],[6,0]]],[8,1]].magnitude(), 1384);
        assert_eq!(number![[[[1,1],[2,2]],[3,3]],[4,4]].magnitude(), 445);
        assert_eq!(number![[[[3,0],[5,3]],[4,4]],[5,5]].magnitude(), 791);
        assert_eq!(number![[[[5,0],[7,4]],[5,5]],[6,6]].magnitude(), 1137);
        assert_eq!(number![[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]].magnitude(), 3488);
    }

    #[test]
    fn test_explode() {
        let mut n;

        n = number![[[[[9, 8], 1], 2], 3], 4];
        n.explode(0);
        assert_eq!(n, number![[[[0, 9], 2], 3], 4]);

        n = number![7, [6, [5, [4, [3, 2]]]]];
        n.explode(0);
        assert_eq!(n, number![7, [6, [5, [7, 0]]]]);

        n = number![[6, [5, [4, [3, 2]]]], 1];
        n.explode(0);
        assert_eq!(n, number![[6, [5, [7, 0]]], 3]);

        n = number![[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]];
        n.explode(0);
        assert_eq!(n, number![[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]]);

        n = number![[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]];
        n.explode(0);
        assert_eq!(n, number![[3, [2, [8, 0]]], [9, [5, [7, 0]]]]);
    }

    #[test]
    fn test_split() {
        let mut n;

        n = number![10];
        n.split();
        assert_eq!(n, number![5, 5]);

        n = number![11];
        n.split();
        assert_eq!(n, number![5, 6]);

        n = number![12];
        n.split();
        assert_eq!(n, number![6, 6]);
    }

    #[test]
    #[rustfmt::skip]
    fn test_add() {
        let sum = number![[[[4,3],4],4],[7,[[8,4],9]]].add(number![1,1]);
        assert_eq!(sum, number![[[[0,7],4],[[7,8],[6,0]]],[8,1]]);

        let sum = [
            number![1,1],
            number![2,2],
            number![3,3],
            number![4,4],
        ].into_iter().fold1(Number::add).unwrap();
        assert_eq!(sum, number![[[[1,1],[2,2]],[3,3]],[4,4]]);

        let sum = [
            number![1,1],
            number![2,2],
            number![3,3],
            number![4,4],
            number![5,5],
            number![6,6],
        ].into_iter().fold1(Number::add).unwrap();
        assert_eq!(sum, number![[[[5,0],[7,4]],[5,5]],[6,6]]);

        let sum = [
            number![[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],
            number![7,[[[3,7],[4,3]],[[6,3],[8,8]]]],
            number![[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]],
            number![[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]],
            number![7,[5,[[3,8],[1,4]]]],
            number![[2,[2,2]],[8,[8,1]]],
            number![2,9],
            number![1,[[[9,3],9],[[9,0],[0,7]]]],
            number![[[5,[7,4]],7],1],
            number![[[[4,2],2],6],[8,7]],
        ].into_iter().fold1(Number::add).unwrap();
        assert_eq!(sum, number![[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]);
    }

    #[test]
    fn test_parse() {
        let parsed = combine_parse(number(), b"1".as_ref()).unwrap();
        assert_eq!(parsed, number![1]);

        let parsed = combine_parse(number(), b"[1,4]".as_ref()).unwrap();
        assert_eq!(parsed, number![1, 4]);

        let parsed = combine_parse(number(), b"[[1,4],[[4,[2,5]],6]]".as_ref()).unwrap();
        assert_eq!(parsed, number![[1, 4], [[4, [2, 5]], 6]]);
    }
}
