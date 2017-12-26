use std::collections::HashSet;
use std::io;

const NUM_BANKS: usize = 16;

fn count_cycles(banks: &mut Vec<i32>) -> u32 {
    let mut cycles: u32 = 0;
    let mut layouts: HashSet<Vec<i32>> = HashSet::new();

    assert!(banks.len() == NUM_BANKS);

    loop {
        let max_bank_idx = max_index(banks).unwrap();
        let redistributed_blocks = banks[max_bank_idx];
        banks[max_bank_idx] = 0;

        let common_blocks = redistributed_blocks / NUM_BANKS as i32;
        for bank in banks.iter_mut() {
            *bank += common_blocks;
        }

        let remaining_blocks = redistributed_blocks % NUM_BANKS as i32;
        for i in 0..remaining_blocks as usize {
            let target_bank_idx = max_bank_idx + i + 1;
            banks[target_bank_idx % NUM_BANKS] += 1;
        }

        cycles += 1;

        if !layouts.insert(banks.to_vec()) {
            return cycles;
        }
    }
}

fn max_index<T: std::cmp::PartialOrd>(items: &Vec<T>) -> Option<usize> {
    if items.len() == 0 {
        return None;
    }

    let mut max_index: usize = 0;
    let mut max_item: &T = &items[0];

    for (index, item) in items.iter().enumerate() {
        if item > max_item {
            max_index = index;
            max_item = &item;
        }
    }

    Some(max_index)
}

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("Failed to read line");

    let mut banks = line.split_whitespace()
        .filter_map(|token| token.parse::<i32>().ok())
        .collect::<Vec<_>>();
    println!("{}", count_cycles(&mut banks));
}
