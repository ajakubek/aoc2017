#[macro_use]
extern crate try_opt;

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};
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
    Snd(Operand),
    Set(Register, Operand),
    Add(Register, Operand),
    Mul(Register, Operand),
    Mod(Register, Operand),
    Rcv(Register),
    Jgz(Operand, Operand),
}

struct Cpu {
    program: Rc<Vec<Instruction>>,
    registers: HashMap<String, i64>,
    pc: usize,
    tx_channel: Rc<RefCell<VecDeque<i64>>>,
    rx_channel: Rc<RefCell<VecDeque<i64>>>,
    finished: bool,
    waiting: bool,
    total_sent: usize,
}

impl Cpu {
    fn new(program_id: i64,
           tx_channel: Rc<RefCell<VecDeque<i64>>>,
           rx_channel: Rc<RefCell<VecDeque<i64>>>,
           program: Rc<Vec<Instruction>>)
           -> Cpu {
        let mut cpu = Cpu {
            program: program,
            pc: 0,
            registers: HashMap::new(),
            tx_channel: tx_channel,
            rx_channel: rx_channel,
            finished: false,
            waiting: false,
            total_sent: 0,
        };
        cpu.set_register(&Register(String::from("p")), &Operand::Value(program_id));
        cpu
    }

    fn run_cycle(&mut self) {
        use Instruction::*;

        if self.pc >= self.program.len() {
            self.finished = true;
        }

        let instruction = self.program[self.pc].clone();

        match instruction {
            Snd(ref operand) => {
                self.send(operand);
            }
            Set(ref reg, ref operand) => {
                self.set_register(reg, operand);
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
            Rcv(ref register) => {
                self.receive(register);
                if self.waiting {
                    return;
                }
            }
            Jgz(ref operand1, ref operand2) => {
                if self.load_operand(&operand1) > 0 {
                    self.pc = (self.pc as i64 + self.load_operand(&operand2)) as usize;
                    return;
                }
            }
        }

        self.pc += 1;
    }

    fn send(&mut self, operand: &Operand) {
        let op_value = self.load_operand(operand);
        self.tx_channel.borrow_mut().push_back(op_value);
        self.total_sent += 1;
    }

    fn receive(&mut self, register: &Register) {
        self.waiting = true;
        let value = self.rx_channel.borrow_mut().pop_front();
        if value.is_some() {
            self.set_register(register, &Operand::Value(value.unwrap()));
            self.waiting = false;
        } else {
            self.waiting = true;
        }
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
}

struct Scheduler {
    programs: Vec<Rc<RefCell<Cpu>>>,
}

impl Scheduler {
    fn new() -> Scheduler {
        Scheduler { programs: Vec::new() }
    }

    fn add_program(&mut self, cpu: Rc<RefCell<Cpu>>) {
        self.programs.push(cpu);
    }

    fn execute(&mut self) {
        loop {
            if self.programs.iter().all(|cpu| cpu.borrow().waiting) {
                break;
            }

            if self.programs.iter().all(|cpu| cpu.borrow().finished) {
                break;
            }

            for mut cpu in self.programs.iter() {
                cpu.borrow_mut().run_cycle();
            }
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
        "rcv" => Rcv(parse_register(&tokens, 1)),
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
    let program = Rc::new(load_program(&mut stdin.lock()));

    let queue_a_b: Rc<RefCell<VecDeque<i64>>> = Rc::new(RefCell::new(VecDeque::new()));
    let queue_b_a: Rc<RefCell<VecDeque<i64>>> = Rc::new(RefCell::new(VecDeque::new()));

    let cpu_a = Rc::new(RefCell::new(Cpu::new(0,
                                              Rc::clone(&queue_a_b),
                                              Rc::clone(&queue_b_a),
                                              Rc::clone(&program))));
    let cpu_b = Rc::new(RefCell::new(Cpu::new(1,
                                              Rc::clone(&queue_b_a),
                                              Rc::clone(&queue_a_b),
                                              Rc::clone(&program))));

    let mut scheduler = Scheduler::new();

    scheduler.add_program(Rc::clone(&cpu_a));
    scheduler.add_program(Rc::clone(&cpu_b));

    scheduler.execute();

    println!("{}", cpu_b.borrow().total_sent);
}
