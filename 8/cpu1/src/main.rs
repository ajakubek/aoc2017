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
}

impl Cpu {
    fn new() -> Cpu {
        Cpu { registers: HashMap::new() }
    }

    fn execute(&mut self, program: &Vec<Instruction>) {
        use Operation::*;

        for instruction in program {
            if self.evaluate_condition(&instruction.condition) {
                match instruction.operation {
                    Inc(ref reg, arg) => { *self.reg_mut(reg) += arg; }
                    Dec(ref reg, arg) => { *self.reg_mut(reg) -= arg; }
                }
            }
        }
    }

    fn evaluate_condition(&self, condition: &Condition) -> bool {
        use Condition::*;

        match *condition {
            Equal(ref reg, arg) => { self.reg(reg) == arg }
            NotEqual(ref reg, arg) => { self.reg(reg) != arg }
            LessThan(ref reg, arg) => { self.reg(reg) < arg }
            LessEqualThan(ref reg, arg) => { self.reg(reg) <= arg }
            GreaterThan(ref reg, arg) => { self.reg(reg) > arg }
            GreaterEqualThan(ref reg, arg) => { self.reg(reg) >= arg }
        }
    }

    fn reg(&self, name: &String) -> i32 {
        if self.registers.contains_key(name) {
            *self.registers.get(name).unwrap()
        } else {
            0
        }
    }

    fn reg_mut(&mut self, name: &String) -> &mut i32 {
        if !self.registers.contains_key(name) {
            self.registers.insert(name.clone(), 0);
        }

        self.registers.get_mut(name).unwrap()
    }
}

fn load_program(reader: &mut BufRead) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();

    for line in reader.lines() {
        let tokens: Vec<String> = line
            .unwrap()
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
    let register = try_opt!(tokens.get(0)).clone();
    let arg = try_opt!(parse_integer(&tokens, 2));
    let operation = match try_opt!(tokens.get(1)).as_str() {
        "inc" => Operation::Inc(register, arg),
        "dec" => Operation::Dec(register, arg),
        _ => { return None; }
    };

    Some(operation)
}

fn parse_condition(tokens: &Vec<String>) -> Option<Condition> {
    if try_opt!(tokens.get(3)) != "if" {
        return None;
    }

    let register = try_opt!(tokens.get(4)).clone();
    let arg = try_opt!(parse_integer(&tokens, 6));
    let condition = match try_opt!(tokens.get(5)).as_str() {
        "==" => Condition::Equal(register, arg),
        "!=" => Condition::NotEqual(register, arg),
        "<" => Condition::LessThan(register, arg),
        "<=" => Condition::LessEqualThan(register, arg),
        ">" => Condition::GreaterThan(register, arg),
        ">=" => Condition::GreaterEqualThan(register, arg),
        _ => { return None; }
    };

    Some(condition)
}

fn parse_integer(tokens: &Vec<String>, index: usize) -> Option<i32> {
    tokens.get(index).and_then(|t| t.parse::<i32>().ok())
}

fn get_max_register_value(cpu: &Cpu) -> i32 {
    *cpu.registers.values().max().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let program = load_program(&mut stdin.lock());

    let mut cpu = Cpu::new();
    cpu.execute(&program);

    println!("{}", get_max_register_value(&cpu));
}
