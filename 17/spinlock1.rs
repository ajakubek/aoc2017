fn get_successor(num_steps: u32, step_size: usize) -> u32 {
    let mut items: Vec<u32> = Vec::new();
    let mut position: usize = 0;

    items.push(0);
    for step in 0..num_steps {
        position = (position + step_size) % items.len();
        if position == items.len() {
            items.push(step + 1);
        } else {
            items.insert(position + 1, step + 1);
        }
        position += 1;
    }

    items[(position + 1) % items.len()]
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("io error");
    let step_size = input.trim().parse::<usize>().expect("invalid step size");
    println!("{}", get_successor(2017, step_size));
}
