use std::io::{self, BufRead};

fn cartesian<T: Clone>(a: &[T], b: &[T]) -> Vec<(T, T)> {
    let mut pairs: Vec<(T, T)> = vec![];
    for (ai, av) in a.iter().enumerate() {
        for (bi, bv) in b.iter().enumerate() {
            if ai != bi {
                pairs.push((av.clone(), bv.clone()));
            }
        }
    }
    pairs
}

fn main() {
    let mut sum = 0;
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut numbers = line.split_whitespace()
            .filter_map(|token| token.parse::<i32>().ok())
            .collect::<Vec<_>>();
        numbers.sort();
        numbers.reverse();
        let pairs = cartesian(&numbers, &numbers);
        for (x, y) in pairs {
            if x % y == 0 {
                sum += x / y;
                break;
            }
        }
    }


    println!("{}", sum);
}
