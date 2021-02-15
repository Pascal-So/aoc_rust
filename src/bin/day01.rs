fn fuel(mass: &i32) -> i32 {
	mass / 3 - 2
}

fn clamped_fuel(mass: &i32) -> i32 {
	std::cmp::max(mass / 3 - 2, 0)
}

fn iterated_fuel(mass: &i32) -> i32 {
	let mut rest = *mass;
	let mut sum = 0;
	while rest > 0 {
		rest = clamped_fuel(&rest);
		sum += rest;
	}
	sum
}

fn solve_a(nums: &Vec<i32>) -> i32 {
	nums.iter().map(fuel).sum()
}

fn solve_b(nums: &Vec<i32>) -> i32 {
	nums.iter().map(iterated_fuel).sum()
}

fn main() {
	println!("Day 01");

	let nums: Vec<i32> = advent::parse_lines_from_file("data/01.txt").unwrap();
	println!("    Simple fuel: {}", solve_a(&nums));
	println!("    Iterated fuel: {}\n", solve_b(&nums));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_day_01() {
		let nums: Vec<i32> = advent::parse_lines_from_file("data/01.txt").unwrap();

		assert_eq!(solve_a(&nums), 3305115);
		assert_eq!(solve_b(&nums), 4954799);
	}
}
