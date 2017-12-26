use std::io::{self, BufRead};

fn run_until_crash(jumps: &mut Vec<i32>) -> u32 {
    let mut steps: u32 = 1;
    let mut position: i32 = 0;

    loop {
        let offset = jumps[position as usize];
        jumps[position as usize] += if offset < 3 { 1 } else { -1 };
        position += offset;
        if position < 0 || position as usize >= jumps.len() {
            return steps;
        }
        steps += 1;
    }
}

fn main() {
    let stdin = io::stdin();

    let mut jumps = stdin.lock()
        .lines()
        .filter_map(|line| line.unwrap().parse::<i32>().ok())
        .collect::<Vec<_>>();

    println!("{}", run_until_crash(&mut jumps));
}
