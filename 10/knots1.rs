fn apply_knots(reversal_lengths: &Vec<usize>) -> Vec<u8> {
    let mut position: usize = 0;
    let mut skip_size: usize = 0;
    let mut items = (0..256).map(|x| x as u8).collect::<Vec<_>>();
    let num_items = items.len();

    for length in reversal_lengths {
        for offset in 0..length / 2 {
            items.swap((position + offset) % num_items,
                       (position + length - offset - 1) % num_items);
        }
        position = (position + length + skip_size) % items.len();
        skip_size += 1;
    }

    items
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("io error");
    let reversal_lengths =
        line.split(',').filter_map(|token| token.trim().parse::<usize>().ok()).collect::<Vec<_>>();
    let items = apply_knots(&reversal_lengths);
    println!("{}", items[0] as u32 * items[1] as u32);
}
