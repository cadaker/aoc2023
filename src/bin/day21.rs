use aoc2023::utils::stdio_lines;
use aoc2023::grid::{Grid, GridBuilder, Point};
use aoc2023::dir::cart_neighbours;
use std::collections::{VecDeque, HashSet};

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

struct WrappedGrid {
    grid: Grid<char>
}

fn safe_mod(x: i32, n: i32) -> i32 {
    ((x % n) + n) % n
}
impl WrappedGrid {
    fn new(grid: Grid<char>) -> WrappedGrid {
        WrappedGrid{ grid }
    }

    fn getp(&self, p: &Point) -> &char {
        self.grid.get(safe_mod(p.row, self.grid.height()), safe_mod(p.col, self.grid.width()))
    }
}

fn search_reachable(grid: &WrappedGrid, start: &Point, max_steps: usize) -> Vec<usize> {
    let mut ret = Vec::new();
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((*start, 0usize));
    visited.insert(*start);

    while !queue.is_empty() {
        let (pos, steps) = queue.pop_front().unwrap();
        if (steps % 2) == (max_steps % 2) {
            if steps / 2 == ret.len() {
                ret.push(1);
            } else {
                ret[steps / 2] += 1;
            }
        }
        if steps < max_steps {
            for n in cart_neighbours(&pos) {
                if !visited.contains(&n) && *grid.getp(&n) == '.' {
                    queue.push_back((n, steps + 1));
                    visited.insert(n);
                }
            }
        }
    }

    ret
}

fn predict_output(grid: &WrappedGrid, start: &Point, max_steps: usize) -> usize {
    let n = (max_steps + 1) / 2;
    const PERIOD: usize = 131; // Actually half of the "real" period.

    let loops = n / PERIOD - 1;
    let offset = (n % PERIOD) + PERIOD;

    let x = search_reachable(&grid, &start, 1200 + (max_steps % 2));

    // The data in x grows as x_i+P = x_i + delta_i, or x_i+rP = x_i + r*delta_i
    // We want the sum of the first n terms of x.
    x[0..offset].iter().sum::<usize>() +
        loops * x[offset..offset + PERIOD].iter().sum::<usize>() +
        (loops - 1) * loops / 2 * ((offset..offset + PERIOD).map(|i| x[i + 2*PERIOD] - x[i + PERIOD]).sum::<usize>())
}

fn main() {
    let (grid, start) = parse_input();
    let wrapped_grid = WrappedGrid::new(grid);

    println!("{}", search_reachable(&wrapped_grid, &start, 64).iter().sum::<usize>());
    const N: usize = 26501365;
    println!("{}", predict_output(&wrapped_grid, &start, N));
}
