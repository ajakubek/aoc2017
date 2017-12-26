#[macro_use]
extern crate try_opt;

use std::collections::HashMap;
use std::io::{self, BufRead};

enum Operation {
    Inc(String, i32),
    Dec(String, i32),
}

enum Condition {
    Equal(String, i32),
    NotEqual(String, i32),
    LessThan(String, i32),
    LessEqualThan(String, i32),
    GreaterThan(String, i32),
    GreaterEqualThan(String, i32),
}

struct Instruction {
    operation: Operation,
    condition: Condition,
}

struct Cpu {
    registers: HashMap<String, i32>,
    max_register_value: i32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            registers: HashMap::new(),
            max_register_value: 0,
        }
    }

    fn execute(&mut self, program: &Vec<Instruction>) {
        use Operation::*;

        for instruction in program {
            if self.evaluate_condition(&instruction.condition) {
                match instruction.operation {
                    Inc(ref reg, arg) => {
                        self.load_modify_store(reg, arg, &|a, b| a + b);
                    }
                    Dec(ref reg, arg) => {
                        self.load_modify_store(reg, arg, &|a, b| a - b);
                    }
                }
            }
        }
    }

    fn evaluate_condition(&self, condition: &Condition) -> bool {
        use Condition::*;

        match *condition {
            Equal(ref reg, arg) => self.reg(reg) == arg,
            NotEqual(ref reg, arg) => self.reg(reg) != arg,
            LessThan(ref reg, arg) => self.reg(reg) < arg,
            LessEqualThan(ref reg, arg) => self.reg(reg) <= arg,
            GreaterThan(ref reg, arg) => self.reg(reg) > arg,
            GreaterEqualThan(ref reg, arg) => self.reg(reg) >= arg,
        }
    }

    fn load_modify_store(&mut self,
                         reg_name: &String,
                         argument: i32,
                         modifier: &Fn(i32, i32) -> i32) {
        let new_value = modifier(self.reg(reg_name), argument);
        self.set_reg(reg_name, new_value);
    }

    fn reg(&self, name: &String) -> i32 {
        if self.registers.contains_key(name) {
            *self.registers.get(name).unwrap()
        } else {
            0
        }
    }

    fn set_reg(&mut self, name: &String, value: i32) {
        self.registers.insert(name.clone(), value);
        self.max_register_value = std::cmp::max(self.max_register_value, value);
    }
}

fn load_program(reader: &mut BufRead) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();

    for line in reader.lines() {
        let tokens: Vec<String> = line.unwrap()
            .split_whitespace()
            .map(|s| String::from(s))
            .collect();

        if tokens.len() != 7 {
            continue;
        }

        if let (Some(operation), Some(condition))
            = (parse_operation(&tokens), parse_condition(&tokens)) {
            program.push(Instruction { operation, condition });
        }
    }

    program
}

fn parse_operation(tokens: &Vec<String>) -> Option<Operation> {
    use Operation::*;

    let register = try_opt!(tokens.get(0)).clone();
    let arg = try_opt!(parse_integer(&tokens, 2));
    let operation = match try_opt!(tokens.get(1)).as_str() {
        "inc" => Inc(register, arg),
        "dec" => Dec(register, arg),
        _ => {
            return None;
        }
    };

    Some(operation)
}

fn parse_condition(tokens: &Vec<String>) -> Option<Condition> {
    use Condition::*;

    if try_opt!(tokens.get(3)) != "if" {
        return None;
    }

    let register = try_opt!(tokens.get(4)).clone();
    let arg = try_opt!(parse_integer(&tokens, 6));
    let condition = match try_opt!(tokens.get(5)).as_str() {
        "==" => Equal(register, arg),
        "!=" => NotEqual(register, arg),
        "<" => LessThan(register, arg),
        "<=" => LessEqualThan(register, arg),
        ">" => GreaterThan(register, arg),
        ">=" => GreaterEqualThan(register, arg),
        _ => {
            return None;
        }
    };

    Some(condition)
}

fn parse_integer(tokens: &Vec<String>, index: usize) -> Option<i32> {
    tokens.get(index).and_then(|t| t.parse::<i32>().ok())
}

fn main() {
    let stdin = io::stdin();
    let program = load_program(&mut stdin.lock());

    let mut cpu = Cpu::new();
    cpu.execute(&program);

    println!("{}", cpu.max_register_value);
}
