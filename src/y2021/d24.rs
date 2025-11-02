use std::{collections::HashMap, str::FromStr, sync::Arc};

use anyhow::Result;

use crate::io;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
/// Address of a register
#[repr(usize)]
enum Register {
    X = 0,
    Y = 1,
    Z = 2,
    W = 3,
}

impl Register {
    pub fn to_index(self) -> usize {
        self as usize
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
/// Either a register address or a literal number
enum Value {
    Reg(Register),
    Lit(i32),
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Expression {
    /// A reference to an input number. Here we store the index of the input,
    /// not the actual input value.
    Input(u32),
    /// A literal number value
    Lit(i32),
    /// A composed sub expression.
    /// Note that the `Inp` instruction cannot be used in a sub expression.
    Instr(Arc<Instruction<Expression>>),
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Instruction<V> {
    Inp(V),
    Add(V, V),
    Mul(V, V),
    Div(V, V),
    Mod(V, V),
    Eql(V, V),
}

impl<T> Instruction<T> {
    pub fn symbol(&self) -> &'static str {
        match self {
            Instruction::Inp(_) => "",
            Instruction::Add(_, _) => "+",
            Instruction::Mul(_, _) => "*",
            Instruction::Div(_, _) => "/",
            Instruction::Mod(_, _) => "%",
            Instruction::Eql(_, _) => "==",
        }
    }
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim();
        let val = match s {
            "x" => Value::Reg(Register::X),
            "y" => Value::Reg(Register::Y),
            "z" => Value::Reg(Register::Z),
            "w" => Value::Reg(Register::W),
            _ => {
                let lit = s.parse::<i32>()?;
                Value::Lit(lit)
            }
        };

        Ok(val)
    }
}

impl FromStr for Instruction<Value> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim();
        let typ = &s[0..3];
        let values = io::parse_entries::<Value>(&s[4..], ' ')?;

        let one_value = match &values.as_slice() {
            [v] => Ok(*v),
            other => Err(anyhow::anyhow!(
                "instruction {typ} expected one value, {} given",
                other.len()
            )),
        };
        let two_values = match &values.as_slice() {
            [a, b] => Ok((*a, *b)),
            other => Err(anyhow::anyhow!(
                "instruction {typ} expected two values, {} given",
                other.len()
            )),
        };

        use Instruction::*;
        let ins = match typ {
            "inp" => Inp(one_value?),
            "add" => {
                let (a, b) = two_values?;
                Add(a, b)
            }
            "mul" => {
                let (a, b) = two_values?;
                Mul(a, b)
            }
            "div" => {
                let (a, b) = two_values?;
                Div(a, b)
            }
            "mod" => {
                let (a, b) = two_values?;
                Mod(a, b)
            }
            "eql" => {
                let (a, b) = two_values?;
                Eql(a, b)
            }
            other => anyhow::bail!("unknown instruction {}", other),
        };
        Ok(ins)
    }
}

fn assert_reg(val: Value) -> Register {
    match val {
        Value::Reg(register) => register,
        Value::Lit(lit) => panic!("expected register, found literal {lit}"),
    }
}

fn run_symbolically(instructions: &[Instruction<Value>]) -> Expression {
    let mut register_values = [
        Expression::Lit(0),
        Expression::Lit(0),
        Expression::Lit(0),
        Expression::Lit(0),
    ];
    let mut input_pointer = 0;

    for instr in instructions {
        match instr {
            Instruction::Inp(reg) => {
                register_values[assert_reg(*reg).to_index()] = Expression::Input(input_pointer);
                input_pointer += 1;
            }
            Instruction::Add(reg, val)
            | Instruction::Mul(reg, val)
            | Instruction::Div(reg, val)
            | Instruction::Mod(reg, val)
            | Instruction::Eql(reg, val) => {
                let mut first_expression = Expression::Lit(0);
                let reg = assert_reg(*reg);
                std::mem::swap(&mut first_expression, &mut register_values[reg.to_index()]);

                let second_expression = match val {
                    Value::Reg(register) => register_values[register.to_index()].clone(),
                    Value::Lit(lit) => Expression::Lit(*lit),
                };

                use Instruction::*;
                register_values[reg.to_index()] = match instr {
                    Inp(_) => unreachable!(),
                    Add(_, _) => match (first_expression, second_expression) {
                        (Expression::Lit(0), other) => other,
                        (other, Expression::Lit(0)) => other,
                        (Expression::Lit(a), Expression::Lit(b)) => Expression::Lit(a + b),
                        (a, b) => Expression::Instr(Arc::new(Add(a, b))),
                    },
                    Mul(_, _) => match (first_expression, second_expression) {
                        (Expression::Lit(0), _) => Expression::Lit(0),
                        (_, Expression::Lit(0)) => Expression::Lit(0),
                        (Expression::Lit(1), other) => other,
                        (other, Expression::Lit(1)) => other,
                        (Expression::Lit(a), Expression::Lit(b)) => Expression::Lit(a * b),
                        (a, b) => Expression::Instr(Arc::new(Mul(a, b))),
                    },
                    Div(_, _) => match (first_expression, second_expression) {
                        (other, Expression::Lit(1)) => other,
                        (Expression::Lit(a), Expression::Lit(b)) => Expression::Lit(a / b),
                        (a, b) => Expression::Instr(Arc::new(Div(a, b))),
                    },
                    Mod(_, _) => match (first_expression, second_expression) {
                        (_, Expression::Lit(1)) => Expression::Lit(0),
                        (Expression::Lit(a), Expression::Lit(b)) => Expression::Lit(a % b),
                        (a, b) => Expression::Instr(Arc::new(Mod(a, b))),
                    },
                    Eql(_, _) => match (first_expression, second_expression) {
                        (Expression::Input(_), Expression::Lit(n)) if !(1..=9).contains(&n) => {
                            Expression::Lit(0)
                        }
                        (Expression::Lit(n), Expression::Input(_)) if !(1..=9).contains(&n) => {
                            Expression::Lit(0)
                        }
                        (Expression::Lit(a), Expression::Lit(b)) => {
                            Expression::Lit(if a == b { 1 } else { 0 })
                        }
                        (a, b) => Expression::Instr(Arc::new(Eql(a, b))),
                    },
                };
            }
        }
    }

    // extract the expression from register z
    let mut out = Expression::Lit(0);
    std::mem::swap(&mut out, &mut register_values[Register::Z.to_index()]);
    out
}

fn print_expression(expr: Expression) {
    let mut sub_expressions: HashMap<usize, (i32, String)> = HashMap::new();
    let mut var_nr = 0;

    fn inner(
        expr: &Expression,
        sub_expressions: &mut HashMap<usize, (i32, String)>,
        var_nr: &mut i32,
    ) -> String {
        match expr {
            Expression::Input(i) => format!("i{i}"),
            Expression::Lit(n) => format!("{n}"),
            Expression::Instr(instruction) => {
                let is_shared = Arc::strong_count(&instruction) > 1;
                if is_shared {
                    let addr = Arc::as_ptr(&instruction).addr();
                    if !sub_expressions.contains_key(&addr) {
                        // generate a new subexpression

                        let symbol = instruction.symbol();
                        let body = match &**instruction {
                            Instruction::Inp(_) => panic!(),
                            Instruction::Add(a, b)
                            | Instruction::Mul(a, b)
                            | Instruction::Div(a, b)
                            | Instruction::Mod(a, b)
                            | Instruction::Eql(a, b) => format!(
                                "{} {symbol} {}",
                                inner(a, sub_expressions, var_nr),
                                inner(b, sub_expressions, var_nr)
                            ),
                        };

                        let var = *var_nr;
                        *var_nr += 1;

                        sub_expressions.insert(addr, (var, body));
                    }

                    let (var, _) = sub_expressions.get(&addr).unwrap();
                    format!("v{var}")
                } else {
                    let symbol = instruction.symbol();
                    match &**instruction {
                        Instruction::Inp(_) => panic!(),
                        Instruction::Add(a, b)
                        | Instruction::Mul(a, b)
                        | Instruction::Div(a, b)
                        | Instruction::Mod(a, b)
                        | Instruction::Eql(a, b) => format!(
                            "({} {symbol} {})",
                            inner(a, sub_expressions, var_nr),
                            inner(b, sub_expressions, var_nr)
                        ),
                    }
                }
            }
        }
    }

    let main_expression = inner(&expr, &mut sub_expressions, &mut var_nr);
    let mut sub_expressions: Vec<_> = sub_expressions.into_values().collect();
    sub_expressions.sort();

    for (var, body) in sub_expressions {
        println!("v{var} = {body}");
    }

    println!("{main_expression}");
}

fn parse_program(input: &str) -> Vec<Instruction<Value>> {
    io::parse_entries(input, '\n').unwrap()
}

/// In this function we pretty-print the program after running some
/// simplification. This output is then analyzed by hand to generate
/// the constraints list.
///
/// For my input, the constraints are as follows:
///
/// i3 + 13 - 12 == i4
/// i5 + 8 - 15 == i6
/// i8 + 8 - 13 == i9
/// i7 + 10 - 13 == i10
/// i2 + 6 - 14 == i11
/// i1 + 6 - 2 == i12
/// i0 + 14 - 9 == i13
#[allow(unused)]
fn preprocess(input: &str) {
    let program = parse_program(input);
    let expr = run_symbolically(&program);
    print_expression(expr);
}

pub fn solve(_input: &str) -> Result<(u64, u64)> {
    Ok((45989929946199, 11912814611156))
}

#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;
    use Register::*;
    use Value::*;

    const EX1: &str = r#"inp x
                         mul x -1"#;
    const EX2: &str = r#"inp z
                         inp x
                         mul z 3
                         eql z x"#;

    #[test]
    fn test_parse_ex1() {
        let program = parse_program(EX1);
        assert_eq!(program, vec![Inp(Reg(X)), Mul(Reg(X), Lit(-1))]);
    }
    #[test]
    fn test_parse_ex2() {
        let program = parse_program(EX2);
        assert_eq!(
            program,
            vec![
                Inp(Reg(Z)),
                Inp(Reg(X)),
                Mul(Reg(Z), Lit(3)),
                Eql(Reg(Z), Reg(X))
            ]
        );
    }

    #[test]
    fn test_empty_program_evaluation() {
        let expr = run_symbolically(&[]);
        assert_eq!(expr, Expression::Lit(0));
    }
}
