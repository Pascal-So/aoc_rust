use anyhow::{bail, Result};

use crate::io;

const MOD: i64 = 20201227;

fn mod_mul(a: i64, b: i64) -> i64 {
    (a * b) % MOD
}

fn mod_pow(mut a: i64, mut b: i64) -> i64 {
    let mut res = 1;
    while b > 0 {
        if b & 1 > 0 {
            res = mod_mul(res, a);
        }
        a = mod_mul(a, a);
        b /= 2;
    }
    res
}

pub fn solve(input: &str) -> Result<i64> {
    let (a, b) = match io::parse_entries(input, '\n')?[..] {
        [a, b] => (a, b),
        _ => bail!("Expected two numbers as input"),
    };

    let mut val = 1;
    let mut log = 1;
    for i in 0.. {
        if val == a {
            log = i;
            break;
        }
        val = mod_mul(val, 7);
    }

    Ok(mod_pow(b, log))
}
