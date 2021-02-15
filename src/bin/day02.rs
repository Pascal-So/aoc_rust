fn run(program: &mut Vec<usize>) -> Result<(), String> {
	let mut instruction_pointer = 0;

	while instruction_pointer < program.len() {
		match program[instruction_pointer] {
			1 | 2 => {
				let v1 = program[program[instruction_pointer + 1]];
				let v2 = program[program[instruction_pointer + 2]];
				let dest = program[instruction_pointer + 3];

				program[dest] = if program[instruction_pointer] == 1 {v1 + v2} else {v1 * v2};
			},
			99 => return Ok(()),
			c => return Err(format!("Unknown opcode {}", c))
		}

		instruction_pointer += 4;
	}

	Err("Reached end of program".to_owned())
}

fn main() {
	println!("Day 02");
	let mut program: Vec<usize> = advent::parse_vector_from_file("data/02.txt", b',').unwrap();

	program[1] = 12;
	program[2] = 2;

	run(&mut program).unwrap();

	println!("    Value at position 0: {}", program[0]);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_day_02() {
		let mut program: Vec<usize> = advent::parse_vector_from_file("data/02.txt", b',').unwrap();

		program[1] = 12;
		program[2] = 2;

		run(&mut program).unwrap();

		assert_eq!(program[0], 3562624);
	}

	#[test]
	fn test_example() {
		let mut program = vec![1,9,10,3, 2,3,11,0, 99, 30,40,50];
		run(&mut program).unwrap();
		assert_eq!(program, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
	}
}
