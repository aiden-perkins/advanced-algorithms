use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

pub fn vertex_cover(file_path: &str) -> i32 {
    let graph = parse_input(file_path);
    (graph.minimum_vertex_cover() / 2) as i32
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
    
    fn hopcroft_karp(&self) -> Vec<(usize, usize)> {
        let mut matching: HashMap<usize, usize> = HashMap::new();
        let mut dist = vec![0; self.left + 1];
        
        while self.bfs(&matching, &mut dist) {
            for u in 0..self.left {
                if !matching.contains_key(&u) {
                    self.dfs(u, &mut matching, &mut dist);
                }
            }
        }
        
        matching.iter().map(|(&u, &v)| (u, v)).collect()
    }

    fn bfs(&self, matching: &HashMap<usize, usize>, dist: &mut Vec<usize>) -> bool {
        let mut queue = VecDeque::new();
        for u in 0..self.left {
            if !matching.contains_key(&u) {
                dist[u] = 0;
                queue.push_back(u);
            } else {
                dist[u] = usize::MAX;
            }
        }
        dist[self.left] = usize::MAX;

        while let Some(u) = queue.pop_front() {
            if dist[u] < dist[self.left] {
                if let Some(edges) = self.edges.get(&u) {
                    for &v in edges {
                        let u_next = matching.get(&v).copied().unwrap_or(self.left);
                        if dist[u_next] == usize::MAX {
                            dist[u_next] = dist[u] + 1;
                            queue.push_back(u_next);
                        }
                    }
                }
            }
        }

        dist[self.left] != usize::MAX
    }

    fn dfs(&self, u: usize, matching: &mut HashMap<usize, usize>, dist: &mut Vec<usize>) -> bool {
        if u != self.left {
            if let Some(edges) = self.edges.get(&u) {
                for &v in edges {
                    let u_next = matching.get(&v).copied().unwrap_or(self.left);
                    if dist[u_next] == dist[u] + 1 && self.dfs(u_next, matching, dist) {
                        matching.insert(u, v);
                        matching.insert(v, u);
                        return true;
                    }
                }
            }
            dist[u] = usize::MAX;
            false
        } else {
            true
        }
    }

    fn minimum_vertex_cover(&self) -> usize {
        let maximum_matching = self.hopcroft_karp();
        maximum_matching.len()
    }
}

fn parse_input(file_path: &str) -> Graph {
    let input_file = File::open(file_path).expect("Failed to open the file: {file_path}");
    let reader = BufReader::new(input_file);
    let mut graph = Graph::new(16000, 16000);
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
