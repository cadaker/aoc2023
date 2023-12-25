use aoc2023::utils::stdio_lines;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone)]
struct Graph {
    edges: HashMap<String, HashMap<String, i64>>,
    vertices: HashSet<String>,
}

impl Graph {
    fn new() -> Self {
        Graph{ edges: HashMap::new(), vertices: HashSet::new() }
    }

    fn add_edge(&mut self, from: String, to: String, capacity: i64) {
        if !self.vertices.contains(&from) {
            self.vertices.insert(from.clone());
            self.edges.insert(from.clone(), HashMap::new());
        }
        if !self.vertices.contains(&to) {
            self.vertices.insert(to.clone());
            self.edges.insert(to.clone(), HashMap::new());
        }
        self.edges.get_mut(&from).unwrap().insert(to, capacity);
    }

    fn has_edge(&self, from: &str, to: &str) -> bool {
        self.edges.get(from).unwrap().get(to).is_some()
    }

    fn change_capacity(&mut self, from: &str, to: &str, diff: i64) {
        if let Some(c) = self.edges.get_mut(from).unwrap().get_mut(to) {
            *c += diff;
        } else {
            self.add_edge(String::from(from), String::from(to), diff);
        }
    }
}

fn parse_input() -> Graph {
    let mut graph = Graph::new();
    for line in stdio_lines() {
        let s: Vec<&str> = line.split(": ").collect();
        for dst in s[1].split_ascii_whitespace() {
            graph.add_edge(String::from(s[0]), String::from(dst), 1);
            graph.add_edge(String::from(dst), String::from(s[0]), 1);
        }
    }
    graph
}

fn find_path(graph: &Graph, from: &str, to: &str) -> Option<Vec<String>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut comes_from= HashMap::new();

    queue.push_back((from, 0usize));
    visited.insert(from);
    while !queue.is_empty() {
        let (node, dist) = queue.pop_front().unwrap();
        if *node == *to {
            let mut path = Vec::new();
            path.push(String::from(node));
            loop {
                let last = path.last().unwrap();
                if *last == *from {
                    path.reverse();
                    return Some(path);
                } else {
                    path.push(String::from(*comes_from.get(last.as_str()).unwrap()));
                }
            }
        }
        for (n, cap) in &graph.edges[node] {
            if !visited.contains(n.as_str()) && *cap > 0 {
                visited.insert(n);
                comes_from.insert(n.as_str(), node);
                queue.push_back((n, dist + 1));
            }
        }
    }
    None
}

fn find_component(graph: &Graph, source: &str) -> HashSet<String> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(source);
    visited.insert(String::from(source));
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        for (n, cap) in &graph.edges[node] {
            if !visited.contains(n) && *cap > 0 {
                visited.insert(n.clone());
                queue.push_back(n);
            }
        }
    }
    visited
}

fn min_cut_between(mut graph: Graph, source: &str, sink: &str) -> (i64, HashSet<String>) {
    let mut capacity = 0;
    loop {
        if let Some(path) = find_path(&graph, source, sink) {
            for i in 1..path.len() {
                let src = &path[i-1];
                let dst = &path[i];

                graph.change_capacity(src, dst, -1);
                graph.change_capacity(dst, src, 1);
            }
            capacity += 1;
        } else {
            return (capacity, find_component(&graph, source));
        }
    }
}

fn min_cut(graph: &Graph) -> (i64, HashSet<String>) {
    let mut best = ((graph.vertices.len() * graph.vertices.len()) as i64, HashSet::new());

    let vertices: Vec<String> = graph.vertices.iter().cloned().collect();
    for i in 1..vertices.len() {
        let (cap, set) = min_cut_between(graph.clone(), &vertices[0], &vertices[i]);

        if cap < best.0 {
            best = (cap, set);
        }
    }
    best
}

fn main() {
    let graph = parse_input();

    let (_best, set) = min_cut(&graph);
    println!("{}", set.len() * (graph.vertices.len() - set.len()));
}
