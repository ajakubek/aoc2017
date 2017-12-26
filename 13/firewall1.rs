use std::collections::HashMap;
use std::io::BufRead;

struct Firewall {
    layers: HashMap<u32, u32>,
}

impl Firewall {
    fn new() -> Firewall {
        Firewall {
            layers: HashMap::new(),
        }
    }

    fn get_severity(&self) -> u32 {
        let mut severity = 0;

        for (depth, range) in &self.layers {
            let period = range * 2 - 2;
            if depth % period == 0 {
                severity += depth * range;
            }
        }

        severity
    }
}

fn load_firewall(reader: &mut std::io::BufRead) -> Firewall {
    let mut firewall = Firewall::new();

    for line in reader.lines() {
        let line = line.expect("io error");
        let tokens = line.split(": ")
            .map(|token| token.parse::<u32>().expect("invalid token"))
            .collect::<Vec<_>>();
        firewall.layers.insert(tokens[0], tokens[1]);
    }

    firewall
}

fn main() {
    let stdin = std::io::stdin();
    let firewall = load_firewall(&mut stdin.lock());
    println!("{}", firewall.get_severity());
}
