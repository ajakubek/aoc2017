extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    id: String,
    weight: i32,
    subnodes: Vec<String>,
}

fn read_nodes(reader: &mut BufRead) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let re = Regex::new(r"^(\w+)\s+\((\d+)\)(?:\s+->\s+(.*))").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(caps) = re.captures(&line) {
            let mut subnodes: Vec<String> = match caps.get(3) {
                Some(subnodes_cap) => {
                    subnodes_cap.as_str()
                        .split_whitespace()
                        .map(|s| String::from(s.trim_matches(',')))
                        .collect()
                }
                None => Vec::new(),
            };

            nodes.push(Node {
                id: String::from(caps.get(1).unwrap().as_str()),
                weight: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                subnodes: subnodes,
            });
        }
    }

    nodes
}

fn find_root(nodes: &Vec<Node>) -> String {
    let mut subnodes: HashSet<String> = HashSet::new();

    for node in nodes.iter() {
        for subnode in node.subnodes.iter() {
            subnodes.insert(subnode.clone());
        }
    }

    return nodes.iter().find(|n| !subnodes.contains(&n.id)).unwrap().id.clone();
}

fn main() {
    let stdin = io::stdin();

    let nodes = read_nodes(&mut stdin.lock());
    let root = find_root(&nodes);

    println!("{}", root);
}
