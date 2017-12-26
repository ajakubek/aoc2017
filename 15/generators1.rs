#![feature(slice_patterns)]

use std::io::BufRead;

struct Generator {
    value: u64,
    factor: u64,
}

impl Generator {
    fn new(seed: u64, factor: u64) -> Generator {
        Generator {
            value: seed,
            factor: factor,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * self.factor) % 2147483647;
        Some(self.value)
    }
}

fn initialize_generators(reader: &mut BufRead) -> Vec<Generator> {
    let factors = [16807, 48271];

    reader.lines()
        .map(|line| {
            line.expect("io error")
                .split_whitespace()
                .last()
                .expect("no seed")
                .parse::<u64>()
                .expect("invalid seed")
        })
        .zip(factors.iter())
        .map(|(value, factor)| Generator::new(value, *factor))
        .collect::<Vec<_>>()
}

fn count_mismatches(num_inputs: usize, gen1: &mut Generator, gen2: &mut Generator) -> usize {
    fn values_match(x: u64, y: u64) -> bool {
        x & 0xffff == y & 0xffff
    }

    gen1.zip(gen2).take(num_inputs).filter(|values| values_match(values.0, values.1)).count()
}

fn main() {
    let stdin = std::io::stdin();
    let mut generators = initialize_generators(&mut stdin.lock());
    if let [ref mut gen_a, ref mut gen_b] = generators[..] {
        println!("{}", count_mismatches(40000000, gen_a, gen_b));
    }
}
