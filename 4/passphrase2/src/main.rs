extern crate permutohedron;

use std::collections::HashSet;
use std::io::{self, BufRead};
use std::iter::FromIterator;

fn is_valid_passphrase(passphrase: String) -> bool {
    let mut word_set: HashSet<String> = HashSet::new();

    for word in passphrase.split_whitespace() {
        if word_set.contains(&word.to_string()) {
            return false;
        }

        let mut chars = word.chars().collect::<Vec<_>>();
        permutohedron::heap_recursive(&mut chars, |permutation| {
            word_set.insert(String::from_iter(permutation.iter().cloned()));
        });
    }

    true
}

fn main() {
    let mut valid_count = 0;
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        if is_valid_passphrase(line.unwrap()) {
            valid_count += 1;
        }
    }

    println!("{}", valid_count);
}
