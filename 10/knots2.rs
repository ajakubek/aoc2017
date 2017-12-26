fn knot_hash(input: &Vec<u8>) -> String {
    let mut position: usize = 0;
    let mut skip_size: usize = 0;
    let mut items = (0..256).map(|x| x as u8).collect::<Vec<_>>();
    let num_items = items.len();
    let suffix: Vec<u8> = vec![17, 31, 73, 47, 23];
    let reversal_lengths =
        input.iter().chain(suffix.iter()).map(|l| *l as usize).collect::<Vec<_>>();

    for _ in 0..64 {
        for length in &reversal_lengths {
            for offset in 0..length / 2 {
                items.swap((position + offset) % num_items,
                           (position + length - offset - 1) % num_items);
            }
            position = (position + length + skip_size) % num_items;
            skip_size += 1;
        }
    }

    let mut hash: String = String::new();

    hash.reserve(32);
    for group in 0..16 {
        let offset = group * 16;
        hash += &format!("{:02x}",
                         items[offset..offset + 16].iter().fold(0, |r, x| r ^ x));
    }

    hash
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("io error");
    let input = line.trim().bytes().collect::<Vec<_>>();
    let hash = knot_hash(&input);
    println!("{}", hash);
}
