#[macro_use]
extern crate try_opt;

use std::collections::HashMap;
use std::io::{self, BufRead};

struct Register(String);

enum Operand {
    Register(String),
    Value(i64),
}

enum Instruction {
    Snd(Operand),
    Set(Register, Operand),
    Add(Register, Operand),
    Mul(Register, Operand),
    Mod(Register, Operand),
    Rcv(Operand),
    Jgz(Operand, Operand),
}

struct Cpu {
    registers: HashMap<String, i64>,
    pc: usize,
    played_sound: i64,
    first_recovered_sound: Option<i64>,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            pc: 0,
            registers: HashMap::new(),
            played_sound: -1,
            first_recovered_sound: None,
        }
    }

    fn execute(&mut self, program: &Vec<Instruction>) {
        use Instruction::*;

        loop {
            if self.pc >= program.len() {
                break;
            }

            match program[self.pc] {
                Snd(ref operand) => {
                    self.played_sound = self.load_operand(&operand);
                }
                Set(ref reg, ref operand) => {
                    let op_value = self.load_operand(operand);
                    self.registers.insert(reg.0.clone(), op_value);
                }
                Add(ref reg, ref operand) => {
                    self.load_modify_store(reg, operand, &|a, b| a + b);
                }
                Mul(ref reg, ref operand) => {
                    self.load_modify_store(reg, operand, &|a, b| a * b);
                }
                Mod(ref reg, ref operand) => {
                    self.load_modify_store(reg, operand, &|a, b| a % b);
                }
                Rcv(ref operand) => {
                    if self.load_operand(&operand) != 0 {
                        if self.first_recovered_sound.is_none() {
                            self.first_recovered_sound = Some(self.played_sound);
                            return;
                        }
                    }
                }
                Jgz(ref operand1, ref operand2) => {
                    if self.load_operand(&operand1) > 0 {
                        self.pc = (self.pc as i64 + self.load_operand(&operand2)) as usize;
                        continue;
                    }
                }
            }

            self.pc += 1;
        }
    }

    fn load_modify_store(&mut self,
                         register: &Register,
                         operand: &Operand,
                         modifier: &Fn(i64, i64) -> i64) {
        let reg_value = *self.registers.get(&register.0).unwrap_or(&0);
        let op_value = self.load_operand(operand);
        self.registers.insert(register.0.clone(), modifier(reg_value, op_value));
    }

    fn load_operand(&self, operand: &Operand) -> i64 {
        use Operand::*;

        match *operand {
            Register(ref reg) => *self.registers.get(reg).unwrap_or(&0),
            Value(x) => x,
        }
    }
}

fn load_program(reader: &mut BufRead) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();

    for line in reader.lines() {
        let tokens: Vec<String> = line.unwrap()
            .split_whitespace()
            .map(|s| String::from(s))
            .collect();

        if tokens.len() < 2 || tokens.len() > 3 {
            continue;
        }

        if let Some(instruction) = parse_instruction(&tokens) {
            program.push(instruction);
        }
    }

    program
}

fn parse_instruction(tokens: &Vec<String>) -> Option<Instruction> {
    use Instruction::*;

    let operation = match try_opt!(tokens.get(0)).as_str() {
        "snd" => Snd(parse_operand(&tokens, 1)),
        "set" => Set(parse_register(&tokens, 1), parse_operand(&tokens, 2)),
        "add" => Add(parse_register(&tokens, 1), parse_operand(&tokens, 2)),
        "mul" => Mul(parse_register(&tokens, 1), parse_operand(&tokens, 2)),
        "mod" => Mod(parse_register(&tokens, 1), parse_operand(&tokens, 2)),
        "rcv" => Rcv(parse_operand(&tokens, 1)),
        "jgz" => Jgz(parse_operand(&tokens, 1), parse_operand(&tokens, 2)),
        _ => {
            return None;
        }
    };

    Some(operation)
}

fn parse_register(tokens: &Vec<String>, index: usize) -> Register {
    Register(tokens.get(index).expect("missing register").clone())
}

fn parse_operand(tokens: &Vec<String>, index: usize) -> Operand {
    let token = tokens.get(index).expect("missing operand");
    if let Ok(value) = token.parse::<i64>() {
        Operand::Value(value)
    } else {
        Operand::Register(token.clone())
    }
}

fn main() {
    let stdin = io::stdin();
    let program = load_program(&mut stdin.lock());

    let mut cpu = Cpu::new();
    cpu.execute(&program);

    println!("{:?}", cpu.first_recovered_sound);
}
