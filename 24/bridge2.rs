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

struct PathFinder<'a> {
    node_index: &'a NodeIndex<'a>,
    max_length: usize,
    max_weight: usize,
}

impl<'a> PathFinder<'a> {
    fn new(node_index: &'a NodeIndex) -> PathFinder<'a> {
        PathFinder {
            node_index: node_index,
            max_length: 0,
            max_weight: 0,
        }
    }

    fn find_strongest_longest_path(&mut self) -> usize {
        self.visit_nodes(0, 0, 0);
        self.max_weight
    }

    fn visit_nodes(&mut self,
                   path_length: usize,
                   path_weight: usize,
                   node_weight: usize) {
        let nodes = self.node_index.get(&node_weight).expect("node not found");

        for node in nodes.iter() {
            if !node.visited.replace(true) {
                let new_length = path_length + 1;
                let new_weight = path_weight + node.weight1 + node.weight2;
                let dst_node_weight = if node_weight == node.weight1 {
                    node.weight2
                } else {
                    node.weight1
                };
                self.add_path(new_length, new_weight);
                self.visit_nodes(new_length, new_weight, dst_node_weight);
                node.visited.replace(false);
            }
        }
    }

    fn add_path(&mut self, length: usize, weight: usize) {
        if length > self.max_length {
            self.max_length = length;
            self.max_weight = weight;
        } else if length == self.max_length {
            self.max_weight = self.max_weight.max(weight);
        }
    }

}

fn main() {
    let stdin = std::io::stdin();

    let nodes = load_nodes(&mut stdin.lock());
    let index = build_index(&nodes);

    let mut path_finder = PathFinder::new(&index);

    println!("{}", path_finder.find_strongest_longest_path());
}
