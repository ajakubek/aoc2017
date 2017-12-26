fn get_zero_successor(num_steps: usize, step_size: usize) -> usize {
    let mut item_count: usize = 1;
    let mut position: usize = 0;
    let mut zero_successor: usize = 0;

    assert!(num_steps > 0);

    for step in 0..num_steps {
        position += step_size;
        if position >= item_count {
            if item_count > step_size {
                position -= item_count;
            } else {
                position %= item_count;
            }
            if position == 0 {
                zero_successor = step + 1;
            }
        }
        item_count += 1;
        position += 1;
    }

    zero_successor
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("io error");
    let step_size = input.trim().parse::<usize>().expect("invalid step size");
    println!("{}", get_zero_successor(50000000, step_size));
}
