#![feature(i128_type)]

struct Bitmap {
    data: Vec<i32>
}

impl Bitmap {
    fn new(prefix: &str) -> Bitmap {
        let mut bitmap = Bitmap {
            data: vec![0; 128 * 128],
        };

        for row in 0..128 {
            let generator = format!("{}-{}", prefix, row);
            let mut hash = knot_hash(&generator.bytes().collect::<Vec<_>>());
            for col in 0..128 {
                bitmap.set_field(row, 127 - col, if hash & 1 != 0 { -1 } else { 0 });
                hash >>= 1;
            }
        }

        bitmap
    }

    fn find_regions(&mut self) -> i32 {
        let mut region_count: i32 = 0;

        for row in 0..128 {
            for col in 0..128 {
                if self.field(row, col) >= 0 {
                    continue;
                }

                self.propagate_region(row, col, region_count + 1);
                region_count += 1;
            }
        }

        region_count
    }

    fn propagate_region(&mut self, row: i32, col: i32, region_index: i32) {
        if self.field(row, col) >= 0 {
            return;
        }

        self.set_field(row, col, region_index);
        if row > 0 {
            self.propagate_region(row - 1, col, region_index);
        }
        if row < 127 {
            self.propagate_region(row + 1, col, region_index);
        }
        if col > 0 {
            self.propagate_region(row, col - 1, region_index);
        }
        if col < 127 {
            self.propagate_region(row, col + 1, region_index);
        }
    }

    fn field(&self, row: i32, col: i32) -> i32 {
        *self.data.get((row * 128 + col) as usize).unwrap()
    }

    fn set_field(&mut self, row: i32, col: i32, value: i32) {
        *self.data.get_mut((row * 128 + col) as usize).unwrap() = value;
    }
}

fn knot_hash(input: &Vec<u8>) -> u128 {
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

    let mut hash: u128 = 0;

    for group in 0..16 {
        let offset = group * 16;
        hash <<= 8;
        hash ^= items[offset..offset + 16].iter().fold(0, |r, x| r ^ x) as u128;
    }

    hash
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("io error");
    let input = line.trim();
    let mut bitmap = Bitmap::new(input);
    println!("{}", &mut bitmap.find_regions());
}
