use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

pub fn chromatic_number(file_path: &str) -> i32 {
    let graph = parse_input(file_path);
    println!("{}", file_path);
    graph.backtracking_coloring()
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
        self.edges.entry(right).or_insert_with(Vec::new).push(left);
    }


    fn is_safe(&self, vertex: usize, color: usize, color_map: &HashMap<usize, usize>) -> bool {
        if let Some(neighbors) = self.edges.get(&vertex) {
            for &neighbor in neighbors {
                if color_map.get(&neighbor) == Some(&color) {
                    return false;
                }
            }
        }
        true
    }

    fn color_graph(&self, vertex: usize, num_colors: usize, color_map: &mut HashMap<usize, usize>) -> bool {
        if vertex == self.left + self.right {
            return true;
        }

        for color in 0..num_colors {
            if self.is_safe(vertex, color, color_map) {
                color_map.insert(vertex, color);
                if self.color_graph(vertex + 1, num_colors, color_map) {
                    return true;
                }
                color_map.remove(&vertex);
            }
        }

        false
    }

    fn backtracking_coloring(&self) -> i32 {
        let mut color_map = HashMap::new();
        let mut num_colors = 1;

        while !self.color_graph(0, num_colors, &mut color_map) {
            num_colors += 1;
            println!("{}", num_colors);
            color_map.clear();
        }

        num_colors as i32
    }
}

fn parse_input(file_path: &str) -> Graph {
    let input_file = File::open(file_path).expect("Failed to open the file: {file_path}");
    let reader = BufReader::new(input_file);
    let mut graph = Graph::new(100, 100);
    let re = Regex::new(r"UndirectedEdge\[(\d+),\s*(\d+)]").unwrap();

    for line in reader.lines() {
        let line = line.expect("Failed to read line in the file: {file_path}");
        for cap in re.captures_iter(&line) {
            let a: usize = cap[1].parse().unwrap();
            let b: usize = cap[2].parse().unwrap();
            graph.add_edge(a - 1, b - 1);
        }
    }

    graph
}

