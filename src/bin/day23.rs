use std::cmp::max;
use aoc2023::grid::{Point, Grid, GridBuilder};
use aoc2023::dir::{step, Dir};
use aoc2023::utils::stdio_lines;
use std::collections::HashMap;
use std::ops::DerefMut;

fn parse_input() -> Grid<char> {
    let mut builder = GridBuilder::new();
    for line in stdio_lines() {
        for ch in line.chars() {
            builder.add(ch);
        }
        builder.eol();
    }
    builder.finish()
}

struct Node {
    exits: Vec<usize>,
    length: usize,
}

struct Graph {
    nodes: HashMap<usize, Node>,
}

struct GraphBuilder {
    next_node_id: usize,
    nodes: HashMap<usize, Node>,
}

impl GraphBuilder {
    fn new() -> GraphBuilder {
        GraphBuilder{ next_node_id: 0, nodes: HashMap::new() }
    }

    fn new_node(&mut self) -> usize {
        let id = self.next_node_id;
        self.next_node_id += 1;
        self.nodes.insert(id, Node{ exits: Vec::new(), length: 0 });
        id
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.nodes.get_mut(&from).unwrap().exits.push(to)
    }

    fn inc_length(&mut self, id: usize, n: usize) {
        self.nodes.get_mut(&id).unwrap().length += n
    }

    fn finish(self) -> Graph {
        Graph{ nodes: self.nodes }
    }
}

fn build_graph(map: &Grid<char>) -> Graph {
    use Dir::*;
    let start = Point{ row: 0, col: 1};
    let end = Point{ row: map.height() - 1, col: map.width() - 2 };

    let mut builder = GraphBuilder::new();
    let outside_node = builder.new_node();

    let mut visited = Grid::from_dim(map.height(), map.width(), 0usize);

    // node 0 means "outside", so 0 -> n means we enter into n, and n -> 0 means we exit through n.
    let mut stack = vec![(start, outside_node)];

    while !stack.is_empty() {
        let (entrance, prev_node) = stack.pop().unwrap();

        let tag = *visited.getp(&entrance);
        if tag > 0 {
            // This section has already been tagged and visited, but we need to add the graph edge
            builder.add_edge(prev_node, tag);
            continue;
        }

        let node = builder.new_node();
        builder.add_edge(prev_node, node);
        // Scan forward, counting up the length. Also, remember to mark any other entrances.
        let mut pos = entrance;
        loop {
            builder.inc_length(node, 1);
            *visited.mutgetp(&pos) = node;

            if pos == end {
                // Found the exit.
                builder.add_edge(node, outside_node);
                break;
            }

            let mut new_pos = None;
            for dir in [Up, Down, Left, Right] {
                let next = step(pos, dir);
                if map.containsp(&next) && *map.getp(&next) != '#' {
                    match (dir, map.getp(&next)) {
                        (Up, '^') | (Down, 'v') | (Left, '<') | (Right, '>') => {
                            // This is an exit to another node.
                            stack.push((next, node));
                        },
                        (Up, 'v') | (Down, '^') | (Left, '>') | (Right, '<') => {
                            // This is another entrance to the node
                            *visited.mutgetp(&next) = node;
                        },
                        (_, '.') => {
                            // Just take a step.
                            if *visited.getp(&next) == 0 {
                                assert!(new_pos.is_none());
                                new_pos = Some(next);
                            }
                        },
                        _ => panic!("Failed to take step"),
                    }
                }
            }
            if let Some(p) = new_pos {
                pos = p;
            } else {
                // Nowhere else to go
                break;
            }
        }
    }

    builder.finish()
}

fn longest_path(graph: &Graph) -> usize {
    let mut stack = Vec::new();
    stack.push((0usize, 0usize));

    let mut max_len = 0;

    while !stack.is_empty() {
        let (node_id, len) = stack.pop().unwrap();
        let node = graph.nodes.get(&node_id).unwrap();
        for e in &node.exits {
            if *e == 0 {
                max_len = max(max_len, len + node.length);
            } else {
                stack.push((*e, len + node.length));
            }
        }
    }

    max_len
}

fn bidirect(graph: &Graph) -> Graph {
    let mut builder = GraphBuilder::new();
    for _ in &graph.nodes {
        builder.new_node();
    }
    for (id, node) in &graph.nodes {
        for e in &node.exits {
            builder.add_edge(*id, *e);
            if *id != 0 && *e != 0 {
                builder.add_edge(*e, *id);
            }
        }
        builder.inc_length(*id, node.length);
    }
    builder.finish()
}

struct State {
    node_id: usize,
    next_child: usize,
    length: usize,
}

fn longest_path2(graph: &Graph) -> usize {
    let mut longest_path = 0usize;

    let mut stack = Vec::new();
    stack.push(State{ node_id: 0, next_child: 0, length: 0});

    let mut visited = Vec::new();
    visited.resize(graph.nodes.len(), false);
    visited[0] = true;

    while !stack.is_empty() {
        let State{node_id, next_child, length} = *stack.last().unwrap();
        let node = graph.nodes.get(&node_id).unwrap();
        if next_child == node.exits.len() {
            visited[node_id] = false;
            stack.pop();
        } else {
            let edge = node.exits[next_child];
            stack.last_mut().unwrap().deref_mut().next_child += 1;
            if edge == 0 {
                longest_path = max(longest_path, length + node.length);
            } else if !visited[edge] {
                visited[edge] = true;
                stack.push(State{ node_id: edge, next_child: 0, length: length + node.length});
            }
        }
    }

    longest_path
}

fn main() {
    let map = parse_input();

    let graph = build_graph(&map);

    println!("{}", longest_path(&graph) - 1);
    println!("{}", longest_path2(&bidirect(&graph)) - 1);
}
