#![feature(slice_patterns)]

use std::io::BufRead;

struct Generator {
    value: u64,
    factor: u64,
    mask: u64,
}

impl Generator {
    fn new(seed: u64, factor: u64, mask: u64) -> Generator {
        Generator {
            value: seed,
            factor: factor,
            mask: mask,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.value = (self.value * self.factor) % 2147483647;
            if self.value & self.mask == 0 {
                return Some(self.value);
            }
        }
    }
}

fn initialize_generators(reader: &mut BufRead) -> Vec<Generator> {
    let factors = [16807, 48271];
    let masks = [4 - 1, 8 - 1];

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
        .zip(masks.iter())
        .map(|((seed, factor), mask)| Generator::new(seed, *factor, *mask))
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
        println!("{}", count_mismatches(5000000, gen_a, gen_b));
    }
}
