use std::collections::{HashMap, HashSet};
use std::io::BufRead;

type Graph = HashMap<i32, Vec<i32>>;

fn load_graph(reader: &mut BufRead) -> Graph {
    let mut graph: Graph = Graph::new();

    for line in reader.lines() {
        let line = line.expect("io error");
        let tokens = line.split_whitespace().map(|token| token.trim()).collect::<Vec<_>>();

        assert!(tokens[1] == "<->");
        let from = tokens[0].parse::<i32>().expect("invalid source");
        let mut to = tokens[2..]
            .iter()
            .map(|token| token.trim_right_matches(',').parse::<i32>().expect("invalid destination"))
            .collect::<Vec<_>>();

        let mut edges = graph.entry(from).or_insert(Vec::new());
        edges.append(&mut to);
    }

    graph
}

fn count_subgraphs(graph: &Graph) -> usize {
    let mut num_subgraphs = 0;
    let mut unvisited: HashSet<i32> = get_all_vertices(graph);

    while !unvisited.is_empty() {
        num_subgraphs += 1;
        let vertex = *unvisited.iter().next().unwrap();
        unvisited = &unvisited - &get_reachable(graph, vertex);
    }

    num_subgraphs
}

fn get_all_vertices(graph: &Graph) -> HashSet<i32> {
    let mut vertices = HashSet::new();

    for (from, to) in graph {
        vertices.insert(*from);
        for v in to {
            vertices.insert(*v);
        }
    }

    vertices
}

fn get_reachable(graph: &Graph, source: i32) -> HashSet<i32> {
    let mut visited: HashSet<i32> = HashSet::new();

    fn visit_neighbors(graph: &Graph, visited: &mut HashSet<i32>, vertex: i32) {
        visited.insert(vertex);

        for neighbor in graph.get(&vertex).expect("missing vertex") {
            if !visited.contains(neighbor) {
                visit_neighbors(graph, visited, *neighbor);
            }
        }
    };

    visit_neighbors(graph, &mut visited, source);

    visited
}

fn main() {
    let stdin = std::io::stdin();
    let graph = load_graph(&mut stdin.lock());
    println!("{:?}", count_subgraphs(&graph));
}
