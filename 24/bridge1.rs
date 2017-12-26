use std::cell::Cell;
use std::collections::HashMap;
use std::io::BufRead;

struct Node {
    weight1: usize,
    weight2: usize,
    visited: Cell<bool>,
}

impl Node {
    fn new(weight1: usize, weight2: usize) -> Node {
        Node {
            weight1: weight1,
            weight2: weight2,
            visited: Cell::new(false),
        }
    }
}

type NodeIndex<'a> = HashMap<usize, Vec<&'a Node>>;

fn load_nodes(reader: &mut BufRead) -> Vec<Node> {
    let mut nodes = Vec::new();

    for line in reader.lines() {
        let line = line.expect("io error");

        let weights = line.split('/')
            .map(|token| token.parse::<usize>().expect("invalid weight"))
            .collect::<Vec<_>>();

        if weights.len() == 2 {
            nodes.push(Node::new(weights[0], weights[1]));
        }
    }

    nodes
}

fn build_index(nodes: &Vec<Node>) -> NodeIndex {
    let mut index = NodeIndex::new();

    for node in nodes.iter() {
        index.entry(node.weight1).or_insert(Vec::new()).push(node);
        index.entry(node.weight2).or_insert(Vec::new()).push(node);
    }

    index
}

fn find_strongest_path(node_index: &NodeIndex) -> usize {
    fn visit_nodes(path_weight: usize, node_index: &NodeIndex, node_weight: usize) -> usize {
        let mut max_weight = path_weight;
        let nodes = node_index.get(&node_weight).expect("node not found");

        for node in nodes.iter() {
            if !node.visited.replace(true) {
                let new_weight = path_weight + node.weight1 + node.weight2;
                let dst_node_weight = if node_weight == node.weight1 {
                    node.weight2
                } else {
                    node.weight1
                };
                max_weight = max_weight.max(visit_nodes(new_weight, node_index, dst_node_weight));
                node.visited.replace(false);
            }
        }

        max_weight
    }

    visit_nodes(0, node_index, 0)
}

fn main() {
    let stdin = std::io::stdin();

    let nodes = load_nodes(&mut stdin.lock());
    let index = build_index(&nodes);

    println!("{}", find_strongest_path(&index));
}
