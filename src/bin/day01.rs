use std::iter;

fn fuel(mass: &i32) -> i32 {
    mass / 3 - 2
}

fn solve_a(nums: &[i32]) -> i32 {
    nums.iter().map(fuel).sum()
}

fn solve_b(nums: &[i32]) -> i32 {
    nums.iter()
        .map(|&m| {
            iter::successors(Some(m), |m| Some(fuel(m)))
                .take_while(|&m| m > 0)
                .skip(1)
        })
        .flatten()
        .sum()
}

fn main() {
    println!("Day 01");

    let nums: Vec<i32> = advent::io::parse_lines_from_file("data/01.txt").unwrap();
    println!("    Simple fuel: {}", solve_a(&nums));
    println!("    Iterated fuel: {}\n", solve_b(&nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01() {
        let nums: Vec<i32> = advent::io::parse_lines_from_file("data/01.txt").unwrap();

        assert_eq!(solve_a(&nums), 3305115);
        assert_eq!(solve_b(&nums), 4954799);
    }
}
