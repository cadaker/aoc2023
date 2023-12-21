use aoc2023::utils::stdio_lines;
use aoc2023::grid::{Grid, GridBuilder, Point};
use aoc2023::dir::cart_neighbours;
use std::collections::VecDeque;

fn parse_input() -> (Grid<char>, Point) {
    let mut builder = GridBuilder::new();
    let mut start = Point{ row: -1, col: -1 };

    for (row, line) in stdio_lines().iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = Point{ row: row as i32, col: col as i32 };
                builder.add('.');
            } else {
                builder.add(ch);
            }
        }
        builder.eol();
    }
    (builder.finish(), start)
}

fn search_reachable(grid: &Grid<char>, start: &Point, max_steps: usize) -> usize {
    let mut visited = Grid::from_dim(grid.height(), grid.width(), false);
    let mut queue = VecDeque::new();
    queue.push_back((*start, 0usize));
    *visited.mutgetp(start) = true;
    let mut count = 0usize;

    while !queue.is_empty() {
        let (pos, steps) = queue.pop_front().unwrap();
        if (steps % 2) == 0 {
            count += 1;
        }
        if steps < max_steps {
            for n in cart_neighbours(&pos) {
                if visited.containsp(&n) && !*visited.getp(&n) && *grid.getp(&n) == '.' {
                    queue.push_back((n, steps + 1));
                    *visited.mutgetp(&n) = true;
                }
            }
        }
    }

    count
}

fn main() {
    let (grid, start) = parse_input();

    println!("{}", search_reachable(&grid, &start, 64));
}
