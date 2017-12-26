extern crate primapalooza;

fn main() {
    let mut prime_count = 0;
    let mut x = 106500;

    while x <= 123500 {
        if !primapalooza::is_prime(x) {
            prime_count += 1;
        }

        x += 17;
    }

    println!("{}", prime_count);
}
