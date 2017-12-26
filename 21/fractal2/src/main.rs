extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io::{Read, Write};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Bitmap {
    data: Vec<bool>,
    dimension: usize,
}

impl Bitmap {
    fn new(dimension: usize) -> Bitmap {
        Bitmap {
            data: vec![false; dimension * dimension],
            dimension: dimension,
        }
    }

    fn flip_x(&self) -> Bitmap {
        let mut flipped = Bitmap::new(self.dimension);
        for y in 0..self.dimension {
            for x in 0..self.dimension {
                flipped.set(x, y, self.get(self.dimension - x - 1, y));
            }
        }
        flipped
    }

    fn flip_y(&self) -> Bitmap {
        let mut flipped = Bitmap::new(self.dimension);
        for y in 0..self.dimension {
            for x in 0..self.dimension {
                flipped.set(x, y, self.get(x, self.dimension - y - 1));
            }
        }
        flipped
    }

    fn rotate(&self) -> Bitmap {
        let mut rotated = Bitmap::new(self.dimension);
        for y in 0..self.dimension {
            for x in 0..self.dimension {
                rotated.set(self.dimension - y - 1, x, self.get(x, y));
            }
        }
        rotated
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.data[y * self.dimension + x]
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        self.data[y * self.dimension + x] = value;
    }

    fn extend(&self, rule_set: &HashMap<Bitmap, Bitmap>) -> Bitmap {
        if self.dimension % 2 == 0 {
            let slices = self.slice_vec(2);
            Bitmap::join(&slices.iter().map(|s| s.apply_rule(rule_set)).collect::<Vec<_>>())
        } else {
            let slices = self.slice_vec(3);
            Bitmap::join(&slices.iter().map(|s| s.apply_rule(rule_set)).collect::<Vec<_>>())
        }
    }

    fn slice_vec(&self, slice_dim: usize) -> Vec<Bitmap> {
        let mut slices = Vec::new();

        assert!(self.dimension % slice_dim == 0);

        let num_slices = self.dimension / slice_dim;
        for sy in 0..num_slices {
            for sx in 0..num_slices {
                slices.push(self.slice(sx * slice_dim, sy * slice_dim, slice_dim));
            }
        }

        slices
    }

    fn slice(&self, slice_x: usize, slice_y: usize, slice_dim: usize) -> Bitmap {
        let mut slice = Bitmap::new(slice_dim);

        for y in 0..slice_dim {
            for x in 0..slice_dim {
                slice.set(x, y, self.get(slice_x + x, slice_y + y));
            }
        }

        slice
    }

    fn apply_rule(&self, rule_set: &HashMap<Bitmap, Bitmap>) -> Bitmap {
        rule_set.get(self).expect("unmatched rule").clone()
    }

    fn join(slices: &Vec<Bitmap>) -> Bitmap {
        assert!(slices.len() > 0);

        let slice_dim = slices[0].dimension;
        let slice_cnt = (slices.len() as f64).sqrt() as usize;
        assert!(slice_cnt * slice_cnt == slices.len());

        let mut joined = Bitmap::new(slice_dim * slice_cnt);

        for sy in 0..slice_cnt {
            for sx in 0..slice_cnt {
                let slice = &slices[sx + sy * slice_cnt];
                for y in 0..slice_dim {
                    for x in 0..slice_dim {
                        let pixel = slice.get(x, y);
                        joined.set(sx * slice_dim + x, sy * slice_dim + y, pixel);
                    }
                }
            }
        }

        joined
    }

    fn count_pixels(&self) -> usize {
        self.data.iter().filter(|p| **p).count()
    }

    fn print(&self) {
        let mut stdout = std::io::stdout();
        for y in 0..self.dimension {
            for x in 0..self.dimension {
                stdout.write(if self.get(x, y) { b"#" } else { b"." }).expect("io error");
            }
            stdout.write(b"\n").expect("io error");
        }
    }
}

#[derive(Debug)]
struct Rule(Bitmap, Bitmap);

fn parse_rules(input: &str, dimension: usize, re: &Regex) -> Vec<Rule> {
    let mut rules = Vec::new();

    for cap in re.captures_iter(input) {
        let data = cap.iter().skip(1).map(|m| m.expect("no match").as_str()).collect::<Vec<_>>();
        rules.push(Rule(parse_bitmap(&data[0..dimension]),
                        parse_bitmap(&data[dimension..])));
    }

    rules
}

fn parse_bitmap(rows: &[&str]) -> Bitmap {
    let mut bitmap = Bitmap::new(rows.len());


    for (y, row) in rows.iter().enumerate() {
        for (x, pixel) in row.chars().enumerate() {
            match pixel {
                '.' => {
                    bitmap.set(x, y, false);
                }
                '#' => {
                    bitmap.set(x, y, true);
                }
                _ => {
                    panic!("Invalid pixel value ({})", pixel);
                }
            }
        }
    }

    bitmap
}

fn initial_map() -> Bitmap {
    Bitmap {
        data: vec![false, true, false, false, false, true, true, true, true],
        dimension: 3,
    }
}

fn add_to_rule_set(rule_set: &mut HashMap<Bitmap, Bitmap>, rules: Vec<Rule>) {
    for &Rule(ref src_bitmap, ref dst_bitmap) in rules.iter() {
        rule_set.insert(src_bitmap.clone(), dst_bitmap.clone());
        rule_set.insert(src_bitmap.flip_x(), dst_bitmap.clone());
        rule_set.insert(src_bitmap.flip_y(), dst_bitmap.clone());
        rule_set.insert(src_bitmap.flip_x().flip_y(), dst_bitmap.clone());

        let mut rotated = src_bitmap.clone();
        for _ in 0..3 {
            rotated = rotated.rotate();
            rule_set.insert(rotated.clone(), dst_bitmap.clone());
        }

        let mut rotated = src_bitmap.flip_x();
        for _ in 0..3 {
            rotated = rotated.rotate();
            rule_set.insert(rotated.clone(), dst_bitmap.clone());
        }

        let mut rotated = src_bitmap.flip_y();
        for _ in 0..3 {
            rotated = rotated.rotate();
            rule_set.insert(rotated.clone(), dst_bitmap.clone());
        }
    }
}

fn main() {
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input).expect("io error");

    let rules2 = parse_rules(&input,
                             2,
                             &Regex::new(r"(..)/(..) => (...)/(...)/(...)").expect("bad regex"));
    let rules3 = parse_rules(&input,
                             3,
                             &Regex::new(r"(...)/(...)/(...) => (....)/(....)/(....)/(....)")
                                 .expect("bad regex"));

    let mut map = initial_map();

    let mut rule_set: HashMap<Bitmap, Bitmap> = HashMap::new();
    add_to_rule_set(&mut rule_set, rules2);
    add_to_rule_set(&mut rule_set, rules3);

    for _ in 0..18 {
        map = map.extend(&rule_set);
    }

    println!("{}", map.count_pixels());
}
