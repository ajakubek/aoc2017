use std::collections::{HashMap, HashSet};

use std::io::BufRead;
use std::rc::Rc;

#[derive(Clone)]
struct Register(String);

#[derive(Clone)]
enum Operand {
    Register(String),
    Value(i64),
}

#[derive(Clone)]
enum Instruction {
    Set(Register, Operand),
    Sub(Register, Operand),
    Mul(Register, Operand),
    Jnz(Operand, Operand),
}

struct Cpu {
    program: Rc<Vec<Instruction>>,
    registers: HashMap<String, i64>,
    pc: usize,
    finished: bool,
    mul_count: usize,
    tracepoints: HashSet<usize>,
}

impl Cpu {
    fn new(program: Rc<Vec<Instruction>>) -> Cpu {
        let mut cpu = Cpu {
            program: program,
            registers: HashMap::new(),
            pc: 0,
            finished: false,
            mul_count: 0,
            tracepoints: HashSet::new(),
        };
        cpu.set_register(&Register(String::from("a")), &Operand::Value(1));
        cpu
    }

    fn run_until_finished(&mut self) {
        while !self.finished {
            self.run_cycle();
        }
    }
    fn run_cycle(&mut self) {
        use Instruction::*;

        if self.tracepoints.contains(&self.pc) {
            self.dump_registers();
        }

        if self.pc >= self.program.len() {
            self.finished = true;
            return;
        }

        let instruction = self.program[self.pc].clone();

        match instruction {
            Set(ref reg, ref operand) => {
                self.set_register(reg, operand);
            }
            Sub(ref reg, ref operand) => {
                self.load_modify_store(reg, operand, &|a, b| a - b);
            }
            Mul(ref reg, ref operand) => {
                self.load_modify_store(reg, operand, &|a, b| a * b);
                self.mul_count += 1;
            }
            Jnz(ref operand1, ref operand2) => {
                if self.load_operand(&operand1) != 0 {
                    self.pc = (self.pc as i64 + self.load_operand(&operand2)) as usize;
                    return;
                }
            }
        }

        self.pc += 1;
    }

    fn set_register(&mut self, register: &Register, operand: &Operand) {
        let op_value = self.load_operand(operand);
        self.registers.insert(register.0.clone(), op_value);
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

    fn set_tracepoint(&mut self, pc: usize) {
        self.tracepoints.insert(pc);
    }

    fn dump_registers(&self) {
        for (register, value) in &self.registers {
            println!("{}: {}", register, value);
        }
        println!("");
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

    if tokens.get(0).is_none() {
        return None;
    }

    let operation = match tokens.get(0).unwrap().as_str() {
        "set" => Set(parse_register(&tokens, 1), parse_operand(&tokens, 2)),
        "sub" => Sub(parse_register(&tokens, 1), parse_operand(&tokens, 2)),
        "mul" => Mul(parse_register(&tokens, 1), parse_operand(&tokens, 2)),
        "jnz" => Jnz(parse_operand(&tokens, 1), parse_operand(&tokens, 2)),
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
    let stdin = std::io::stdin();
    let program = Rc::new(load_program(&mut stdin.lock()));

    let mut cpu = Cpu::new(Rc::clone(&program));
    cpu.set_tracepoint(15);
    cpu.run_until_finished();

    println!("{}", cpu.mul_count);
}
