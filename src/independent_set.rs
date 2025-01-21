
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashSet;

pub fn independent_set(file_path: &str) -> i32 {
    let graph = parse_input(file_path);
    let max_set = find_maximum_independent_set(&graph, file_path);
    max_set.len() as i32
}

struct Graph {
    n: usize,
    edges: Vec<HashSet<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            n,
            edges: vec![HashSet::new(); n],
        }
    }

    fn add_edge(&mut self, a: usize, b: usize) {
        self.edges[a].insert(b);
        self.edges[b].insert(a);
    }
}

fn parse_input(file_path: &str) -> Graph {
    let input_file = File::open(file_path).expect("Failed to open the file");
    let reader = BufReader::new(input_file);
    let re = Regex::new(r"UndirectedEdge\[(\d+),\s*(\d+)]").unwrap();

    let mut max_node = 0;
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line in the file");
        for cap in re.captures_iter(&line) {
            let a: usize = cap[1].parse().unwrap();
            let b: usize = cap[2].parse().unwrap();
            max_node = max_node.max(a).max(b);
            edges.push((a, b));
        }
    }

    let mut graph = Graph::new(max_node + 1);
    for (a, b) in edges {
        graph.add_edge(a, b);
    }

    graph
}

fn find_maximum_independent_set(graph: &Graph, file_path: &str) -> HashSet<usize> {
    let mut max_set = HashSet::new();
    let mut current_set = HashSet::new();

    backtrack(graph, &mut current_set, 0, &mut max_set, file_path);

    max_set
}

fn backtrack(graph: &Graph, current_set: &mut HashSet<usize>, start: usize, max_set: &mut HashSet<usize>, file_path: &str) {
    if current_set.len() > max_set.len() {
        *max_set = current_set.clone();
        println!("{}: {}", file_path, max_set.len());
    }

    for v in start..graph.n {
        if !current_set.iter().any(|&u| graph.edges[u].contains(&v)) {
            current_set.insert(v);
            backtrack(graph, current_set, v + 1, max_set, file_path);
            current_set.remove(&v);
        }
    }
}
