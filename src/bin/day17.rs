use std::collections::HashSet;
use aoc2023::grid::{Grid, GridBuilder, Point};
use aoc2023::utils::stdio_lines;
use priority_queue::PriorityQueue;

fn parse_input() -> Grid<i64> {
    let mut builder = GridBuilder::new();
    for line in stdio_lines() {
        for ch in line.chars() {
            builder.add(ch.to_string().parse().unwrap())
        }
        builder.eol()
    }
    builder.finish()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn possible_dirs(in_dir: Dir, steps: usize, min: usize, max: usize) -> Vec<Dir> {
    if steps < min {
        return vec![in_dir];
    }
    use Dir::*;
    match (in_dir, steps >= max) {
        (Left, false) => vec![Left, Up, Down],
        (Left, true) => vec![Up, Down],
        (Right, false) => vec![Right, Up, Down],
        (Right, true) => vec![Up, Down],
        (Up, false) => vec![Left, Up, Right],
        (Up, true) => vec![Left, Right],
        (Down, false) => vec![Down, Left, Right],
        (Down, true) => vec![Left, Right],
    }
}

fn step(p: Point, d: Dir) -> Point {
    use Dir::*;
    match d {
        Left => Point{row: p.row, col: p.col - 1},
        Right => Point{row: p.row, col: p.col + 1},
        Up => Point{row: p.row - 1, col: p.col},
        Down => Point{row: p.row + 1, col: p.col},
    }
}

fn search(grid: &Grid<i64>, min_steps: usize, max_steps: usize) -> i64 {
    let mut pq = PriorityQueue::new();
    pq.push((Point{ row: 0, col: 0 }, Dir::Right, 0usize), 0i64);

    let mut visited = HashSet::new();
    visited.insert((Point{row: 0, col: 0}, Dir::Right, 0usize));

    // Heat loss values are counted as negative, because that way the prio queue works out
    while !pq.is_empty() {
        let ((pos, dir, steps), heat_loss) = pq.pop().unwrap();
        if pos.row == grid.height() - 1 && pos.col == grid.width() - 1 {
            return -heat_loss;
        }

        for d in possible_dirs(dir, steps, min_steps, max_steps) {
            let next_p = step(pos, d);
            if grid.containsp(&next_p) {
                let next_node = (next_p, d, if d != dir { 1 } else { steps + 1 });
                let next_heat_loss = heat_loss - grid.getp(&next_p);
                let n = pq.get(&next_node);
                if visited.get(&next_node).is_none() {
                    if n.is_none() {
                        pq.push(next_node, next_heat_loss);
                    } else if next_heat_loss > *n.unwrap().1 {
                        pq.change_priority(&next_node, next_heat_loss);
                    }
                    visited.insert(next_node);
                }
            }
        }
    }
    0
}

fn main() {
    let grid = parse_input();

    println!("{}", search(&grid, 0, 3));
    println!("{}", search(&grid, 4, 10));
}
