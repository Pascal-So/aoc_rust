use advent::bivariate_polynomial::BP;
use std::cmp::Ordering;

fn access(program: &[BP], instruction_pointer: usize) -> Result<usize, String> {
    let val = &program[instruction_pointer];
    val.get_constant()
        .ok_or(format!("Value not constant: {}", val))
}

fn run(program: &mut [BP]) -> Result<(), String> {
    let mut instruction_pointer = 0;

    while instruction_pointer < program.len() {
        let instruction = access(program, instruction_pointer)?;

        match instruction {
            1 | 2 => {
                let v1 = &program[access(program, instruction_pointer + 1).unwrap_or(0)];
                let v2 = &program[access(program, instruction_pointer + 2).unwrap_or(0)];
                let dest = access(program, instruction_pointer + 3)?;

                program[dest] = if instruction == 1 { v1 + v2 } else { v1 * v2 };
            }
            99 => return Ok(()),
            c => return Err(format!("Unknown opcode {}", c)),
        }

        instruction_pointer += 4;
    }

    Err("Reached end of program".to_owned())
}

fn requires_brute_force(program: &[usize]) -> bool {
    // Iterate only over the output addresses.
    for (i, f) in program.iter().enumerate().skip(3).step_by(4) {
        if *f > i {
            // Program will write to an address containing code
            // that hasn't yet been read.
            return true;
        }
    }

    // For the 2 inputs to this puzzle i've seen so far, we always have
    // the property that the first output is written to address 3, after
    // which we never use the output before overwriting it. This, together
    // with the previously observed property, means that after the first
    // instruction we never again have a noun or verb dependent address to
    // read or write from. The end result is thus a multivariate polynomial
    // in the noun and verb.
    program[3] != 3 || program[5] == 3 || program[6] == 3 || program[7] != 3
}

fn load_program(file: impl AsRef<std::path::Path>) -> Vec<BP> {
    let program: Vec<usize> = advent::io::parse_vector_from_file(file, b',').unwrap();

    if requires_brute_force(&program) {
        panic!("This program has address-dependent reads and the task thus has to be solved by brute force.")
    }

    program.into_iter().map(BP::constant).collect()
}

fn day02() -> (usize, i64) {
    println!("Day 02");

    let mut program = load_program("data/02.txt");

    program[1] = BP::x();
    program[2] = BP::y();

    run(&mut program).unwrap();

    println!("    Value at position 0: {}", program[0]);

    let sol_a = program[0].evaluate(12, 2);
    println!("    Evaluated with x = 12, y = 2: {}\n", sol_a);

    if !program[0].is_linear() {
        panic!("Nonlinear Diophantine equation, requires brute force.");
    }

    // We're left with a linear Diophantine equation in two variables, i.e.
    // ax + by = c
    // where all variables are integer.
    let a = program[0].get_coeff(1, 0) as i64;
    let b = program[0].get_coeff(0, 1) as i64;
    let c = 19690720 - program[0].get_coeff(0, 0) as i64;

    println!("    Solving diophantine equation {}x + {}y = {}", a, b, c);

    let dsol = advent::diophantine::linear_equation(a, b, c).unwrap();

    println!(
        "    Solution is in ({}, {}) + k({}, {}) with k in Z",
        dsol.offset.x, dsol.offset.y, dsol.step.x, dsol.step.y
    );

    let k_min = -dsol.offset.x / dsol.step.x;
    let k_max = -dsol.offset.y / dsol.step.y;

    println!("    k is in [{}, {}]", k_min, k_max);

    let xy = match (-dsol.step.y).cmp(&dsol.step.x) {
        Ordering::Greater => dsol.offset + dsol.step * k_max,
        Ordering::Equal => dsol.offset + dsol.step * (k_max + k_min) / 2,
        Ordering::Less => dsol.offset + dsol.step * k_min,
    };

    let sol_b = xy.x * 100 + xy.y;
    println!("    x = {}, y = {}. Solution = {}", xy.x, xy.y, sol_b);

    (sol_a, sol_b)
}

fn main() {
    day02();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02() {
        assert_eq!(day02(), (3562624, 8298));
    }

    #[test]
    fn test_example() {
        let mut program: Vec<BP> = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
            .iter()
            .copied()
            .map(BP::constant)
            .collect();

        run(&mut program).unwrap();
        let endstate = program
            .iter()
            .map(|c| c.get_constant().unwrap())
            .collect::<Vec<usize>>();
        assert_eq!(endstate, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }
}
