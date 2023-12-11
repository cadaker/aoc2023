use aoc2023::grid::{Grid, GridBuilder, Point};
use aoc2023::utils::stdio_lines;

fn parse_input() -> Grid<char> {
    let lines = stdio_lines();
    let mut builder = GridBuilder::new();

    for line in lines {
        for ch in line.chars() {
            builder.add(ch);
        }
        builder.eol();
    }

    builder.finish()
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn delta(dir: Dir) -> (i32, i32) {
    match dir {
        Dir::Up => (-1, 0),
        Dir::Right => (0, 1),
        Dir::Down => (1, 0),
        Dir::Left => (0, -1),
    }
}

fn step(p: &Point, d: Dir) -> Point {
    let (dr, dc) = delta(d);
    Point{row: p.row + dr, col: p.col + dc}
}

fn enterable_via(pipe: char, dir: Dir) -> bool {
    match dir {
        Dir::Up => pipe == '|' || pipe == 'F' || pipe == '7',
        Dir::Right => pipe == '-' || pipe == 'J' || pipe == '7',
        Dir::Down => pipe == '|' || pipe == 'J' || pipe == 'L',
        Dir::Left => pipe == '-' || pipe == 'F' || pipe == 'L',
    }
}

fn exit_of(pipe: char, in_dir: Dir) -> Dir {
    match (pipe, in_dir) {
        ('|', Dir::Up) => Dir::Up,
        ('|', Dir::Down) => Dir::Down,
        ('-', Dir::Left) => Dir::Left,
        ('-', Dir::Right) => Dir::Right,
        ('J', Dir::Down) => Dir::Left,
        ('J', Dir::Right) => Dir::Up,
        ('L', Dir::Left) => Dir::Up,
        ('L', Dir::Down) => Dir::Right,
        ('7', Dir::Right) => Dir::Down,
        ('7', Dir::Up) => Dir::Left,
        ('F', Dir::Left) => Dir::Down,
        ('F', Dir::Up) => Dir::Right,
        _ => panic!("Mismatch: {} {:?}", pipe, in_dir)
    }
}

fn find_start(grid: &Grid<char>) -> Option<Point> {
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            if *grid.get(row, col) == 'S' {
                return Some(Point{row, col});
            }
        }
    }
    None
}

fn trace_loop(grid: &Grid<char>) -> Vec<Point> {
    let start = find_start(grid).unwrap();

    let matches = |dir: Dir| {
        let next = step(&start, dir);
        grid.containsp(&next) && enterable_via(*grid.getp(&next), dir)
    };

    let mut dir = Dir::Up;
    let mut next: Point = start;
    for d in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
        if matches(d) {
            dir = d;
            next = step(&start, d);
        }
    }

    let mut path = Vec::new();
    path.push(start);

    while next != start {
        path.push(next.clone());
        let out_dir = exit_of(*grid.getp(&next), dir);
        dir = out_dir;
        next = step(&next, dir);
    }

    path
}

fn main() {
    let grid = parse_input();

    let path = trace_loop(&grid);

    println!("{}", path.len() / 2);
}
