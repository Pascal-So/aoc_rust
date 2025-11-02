use anyhow::{bail, Result};
use itertools::Itertools;

use crate::io;

fn parse_input(input: &str) -> Result<(i64, i64)> {
    if let [l, u] = io::parse_entries(input, '-')?.as_slice() {
        Ok((*l, *u))
    } else {
        bail!("Incorrect number of segments in input!");
    }
}

fn is_pwd<const N: usize>(digits: &[u8; N], allow_groups: bool) -> bool {
    let mut has_double = false;
    let mut group_len = 1;
    for (last, current) in digits.iter().tuple_windows() {
        match last.cmp(current) {
            std::cmp::Ordering::Less => {
                if !allow_groups {
                    if group_len == 2 {
                        has_double = true;
                    }
                    group_len = 1;
                }
            }
            std::cmp::Ordering::Equal => {
                if allow_groups {
                    has_double = true;
                } else {
                    group_len += 1;
                }
            }
            std::cmp::Ordering::Greater => return false,
        }
    }
    if group_len == 2 {
        has_double = true;
    }

    has_double
}

pub fn solve(input: &str) -> Result<(i32, i32)> {
    let (lower, upper) = parse_input(input)?;

    let iter = NonDecreasing::<6>::new(lower)?;
    let limit = NonDecreasing::<6>::new(upper)?.next().unwrap();

    let mut sol_a = 0;
    let mut sol_b = 0;

    for digits in iter {
        if digits > limit {
            break;
        }

        sol_a += is_pwd(&digits, true) as i32;
        sol_b += is_pwd(&digits, false) as i32;
    }

    Ok((sol_a, sol_b))
}

pub fn next_nondecreasing<const N: usize>(mut num: [u8; N]) -> Option<[u8; N]> {
    for i in (0..N).rev() {
        if num[i] < 9 {
            let new = num[i] + 1;
            for d in &mut num[i..] {
                *d = new;
            }
            return Some(num);
        }
    }

    None
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
struct NonDecreasing<const N: usize> {
    digits: Option<[u8; N]>,
}

impl<const N: usize> NonDecreasing<N> {
    pub fn new(mut i: i64) -> Result<Self> {
        let mut digits = [0; N];
        for d in digits.iter_mut().rev() {
            *d = (i % 10) as u8;
            i /= 10;
        }
        if i > 0 {
            bail!("Number too big!");
        }

        Ok(NonDecreasing {
            digits: Some(digits),
        })
    }
}

impl<const N: usize> Iterator for NonDecreasing<N> {
    type Item = [u8; N];

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.digits;
        self.digits = next_nondecreasing(self.digits?);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_nondecreasing() {
        assert_eq!(next_nondecreasing([1, 2]), Some([1, 3]));
        assert_eq!(next_nondecreasing([1, 9]), Some([2, 2]));
        assert_eq!(next_nondecreasing([1, 3, 8]), Some([1, 3, 9]));
        assert_eq!(next_nondecreasing([1, 8, 8]), Some([1, 8, 9]));
        assert_eq!(next_nondecreasing([1, 8, 9]), Some([1, 9, 9]));
        assert_eq!(next_nondecreasing([1, 9, 9]), Some([2, 2, 2]));
        assert_eq!(next_nondecreasing([9, 9, 9]), None);
    }
}
