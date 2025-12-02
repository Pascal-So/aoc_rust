use anyhow::Result;
use std::ops::RangeInclusive;

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let mut a = 0;
    let mut b = 0;
    for range in parse(input) {
        for i in range {
            match repeatnum(i) {
                2 => {
                    a += i;
                    b += i;
                }
                3.. => {
                    b += i;
                }
                _ => {}
            }
        }
    }

    Ok((a, b))
}

fn repeatnum(num: i64) -> usize {
    let s = format!("{num}");

    let len = s.len();
    'outer: for i in 2..=len {
        if len % i != 0 {
            continue;
        }

        let mut iter = s.as_bytes().chunks(len / i).into_iter();
        let first = iter.next().unwrap();
        for other in iter {
            if other != first {
                continue 'outer;
            }
        }

        return i;
    }

    0
}

fn parse(input: &str) -> impl Iterator<Item = RangeInclusive<i64>> + use<'_> {
    input.split(",").map(|range| {
        let (a, b) = range.split_once("-").unwrap();
        let a = a.parse().unwrap();
        let b = b.trim().parse().unwrap();
        a..=b
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat() {
        assert_eq!(repeatnum(12341234), 2);
        assert_eq!(repeatnum(123123123), 3);
        assert_eq!(repeatnum(1212121212), 5);
        assert_eq!(repeatnum(12121212), 2);
    }
}
