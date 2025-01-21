use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

pub fn matching(file_path: &str) -> usize {
    let graph = parse_input(file_path);

    let mut matching = vec![None; graph.right];
    let mut visited = vec![false; graph.left];
    let mut result = Vec::new();

    for left in 0..graph.left {
        if dfs(&graph, left, &mut matching, &mut visited) {
            visited.fill(false);
        }
    }

    for (right, &left) in matching.iter().enumerate() {
        if let Some(left) = left {
            result.push((left, right));
        }
    }

    result.len()
}

fn dfs(graph: &Graph, left: usize, matching: &mut Vec<Option<usize>>, visited: &mut Vec<bool>) -> bool {
    if visited[left] {
        return false;
    }
    visited[left] = true;

    if let Some(edges) = graph.edges.get(&left) {
        for &right in edges {
            if matching[right].is_none() || dfs(graph, matching[right].unwrap(), matching, visited) {
                matching[right] = Some(left);
                return true;
            }
        }
    }

    false
}

struct Graph {
    left: usize,
    right: usize,
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new(left: usize, right: usize) -> Self {
        Graph {
            left,
            right,
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, left: usize, right: usize) {
        self.edges.entry(left).or_insert_with(Vec::new).push(right);
    }
}

fn parse_input(file_path: &str) -> Graph {
    let input_file = File::open(file_path).expect("Failed to open the file: {file_path}");
    let reader = BufReader::new(input_file);
    let mut graph = Graph::new(20000, 20000);
    let re = Regex::new(r"UndirectedEdge\[(\d+),\s*(\d+)]").unwrap();

    for line in reader.lines() {
        let line = line.expect("Failed to read line in the file: {file_path}");
        for cap in re.captures_iter(&line) {
            let a: usize = cap[1].parse().unwrap();
            let b: usize = cap[2].parse().unwrap();
            graph.add_edge(a - 1, b - 20001);
        }
    }
    
    graph
}

