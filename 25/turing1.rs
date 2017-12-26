use std::collections::HashSet;

struct Tape {
    tape: HashSet<isize>,
    pos: isize,
}

impl Tape {
    fn new() -> Tape {
        Tape {
            tape: HashSet::new(),
            pos: 0,
        }
    }

    fn move_left(&mut self) {
        self.pos -= 1;
    }

    fn move_right(&mut self) {
        self.pos += 1;
    }

    fn get(&self) -> bool {
        self.tape.contains(&self.pos)
    }

    fn set(&mut self, value: bool) {
        if value {
            self.tape.insert(self.pos);
        } else {
            self.tape.remove(&self.pos);
        }
    }
}

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

fn run_program(tape: &mut Tape, num_steps: usize) {
    use State::*;

    let mut state = A;

    for _ in 0..num_steps {
        state = match state {
            A => handle_state_a(tape),
            B => handle_state_b(tape),
            C => handle_state_c(tape),
            D => handle_state_d(tape),
            E => handle_state_e(tape),
            F => handle_state_f(tape),
        }
    }
}

fn handle_state_a(tape: &mut Tape) -> State {
    if !tape.get() {
        tape.set(true);
        tape.move_right();
        State::B
    } else {
        tape.set(false);
        tape.move_left();
        State::D
    }
}

fn handle_state_b(tape: &mut Tape) -> State {
    if !tape.get() {
        tape.set(true);
        tape.move_right();
        State::C
    } else {
        tape.set(false);
        tape.move_right();
        State::F
    }
}

fn handle_state_c(tape: &mut Tape) -> State {
    if !tape.get() {
        tape.set(true);
        tape.move_left();
        State::C
    } else {
        tape.set(true);
        tape.move_left();
        State::A
    }
}

fn handle_state_d(tape: &mut Tape) -> State {
    if !tape.get() {
        tape.set(false);
        tape.move_left();
        State::E
    } else {
        tape.set(true);
        tape.move_right();
        State::A
    }
}

fn handle_state_e(tape: &mut Tape) -> State {
    if !tape.get() {
        tape.set(true);
        tape.move_left();
        State::A
    } else {
        tape.set(false);
        tape.move_right();
        State::B
    }
}

fn handle_state_f(tape: &mut Tape) -> State {
    if !tape.get() {
        tape.set(false);
        tape.move_right();
        State::C
    } else {
        tape.set(false);
        tape.move_right();
        State::E
    }
}

fn main() {
    let mut tape = Tape::new();

    run_program(&mut tape, 12302209);

    println!("{}", tape.tape.len());
}
