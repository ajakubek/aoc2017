use std::collections::HashSet;
use std::io::{self, BufRead};

fn is_valid_passphrase(passphrase: String) -> bool {
    let mut word_set: HashSet<String> = HashSet::new();

    for word in passphrase.split_whitespace() {
        if !word_set.insert(String::from(word)) {
            return false;
        }
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
