use std::io::{self, Read};
use std::iter::FromIterator;

fn main() {
    let digits = Vec::from_iter(io::stdin()
        .bytes()
        .map(|x| x.unwrap() as i32 - '0' as i32)
        .filter(|&x| x >= 0));

    let sum = digits.iter()
        .zip(digits.iter().cycle().skip(1))
        .filter(|p| p.0 == p.1)
        .map(|p| p.0)
        .fold(0, |sum, x| sum + x);

    println!("{}", sum)
}
