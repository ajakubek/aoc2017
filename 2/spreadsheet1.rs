use std::io::{self, BufRead};

fn main() {
    let mut sum = 0;
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let mut row_min = i32::max_value();
        let mut row_max = i32::min_value();

        for token in line.unwrap().split_whitespace() {
            let number = token.parse::<i32>().unwrap();
            row_min = std::cmp::min(number, row_min);
            row_max = std::cmp::max(number, row_max);
        }

        sum += row_max - row_min;
    }

    println!("{}", sum);
}
