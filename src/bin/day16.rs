use std::collections::HashSet;
use aoc2023::grid::{Grid, GridBuilder, Point};
use aoc2023::utils::stdio_lines;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Square {
    Empty,
    MirrorULDR,
    MirrorURDL,
    SplitterHoriz,
    SplitterVert,
}

fn parse_input() -> Grid<Square> {
    let mut builder = GridBuilder::new();

    use Square::*;
    for line in stdio_lines() {
        for ch in line.chars() {
            match ch {
                '.' => builder.add(Empty),
                '/' => builder.add(MirrorURDL),
                '\\' => builder.add(MirrorULDR),
                '-' => builder.add(SplitterHoriz),
                '|' => builder.add(SplitterVert),
                _ => panic!("Invalid input token"),
            }
        }
        builder.eol();
    }

    builder.finish()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn interact(in_dir: Dir, square: Square) -> Vec<Dir> {
    use Square::*;
    use Dir::*;
    match (square, in_dir) {
        (Empty, _) => vec![in_dir],
        (SplitterHoriz, Up | Down) => vec![Left, Right],
        (SplitterHoriz, _) => vec![in_dir],
        (SplitterVert, Left | Right) => vec![Up, Down],
        (SplitterVert, _) => vec![in_dir],
        (MirrorURDL, Left) => vec![Down],
        (MirrorURDL, Right) => vec![Up],
        (MirrorURDL, Down) => vec![Left],
        (MirrorURDL, Up) => vec![Right],
        (MirrorULDR, Left) => vec![Up],
        (MirrorULDR, Right) => vec![Down],
        (MirrorULDR, Down) => vec![Right],
        (MirrorULDR, Up) => vec![Left],
    }
}

fn step(p: Point, dir: Dir) -> Point {
    use Dir::*;
    match dir {
        Up => Point{ row: p.row - 1, col: p.col},
        Down => Point{ row: p.row + 1, col: p.col},
        Right => Point{ row: p.row, col: p.col + 1},
        Left => Point{ row: p.row, col: p.col - 1},
    }
}

fn search(grid: &Grid<Square>, start_point: Point, start_dir: Dir) -> usize {
    let mut stack = Vec::new();
    stack.push((start_point, start_dir));

    let mut visited: HashSet<(Point, Dir)> = HashSet::new();
    visited.insert((start_point, start_dir));

    while !stack.is_empty() {
        let (p, dir) = stack.pop().unwrap();

        let square = grid.getp(&p);
        for new_dir in interact(dir, *square) {
            let new_p = step(p, new_dir);
            if grid.containsp(&new_p) && !visited.contains(&(new_p, new_dir)) {
                stack.push((new_p, new_dir));
                visited.insert((new_p, new_dir));
            }
        }
    }

    let visited_point = |p: Point| {
        visited.contains(&(p, Dir::Left)) ||
            visited.contains(&(p, Dir::Right)) ||
            visited.contains(&(p, Dir::Up)) ||
            visited.contains(&(p, Dir::Down))
    };

    let mut count = 0;
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            if visited_point(Point{row, col}) {
                count += 1;
            }
        }
    }
    count
}

fn maximize_count(grid: &Grid<Square>) -> usize {
    let mut counts = Vec::new();

    for row in 0..grid.height() {
        counts.push(search(grid, Point{ row, col: 0 }, Dir::Right));
        counts.push(search(grid, Point{ row, col: grid.width() - 1 }, Dir::Left));
    }
    for col in 0..grid.width() {
        counts.push(search(grid, Point{ row: 0, col }, Dir::Down));
        counts.push(search(grid, Point{ row: grid.height() - 1, col }, Dir::Up));
    }

    *counts.iter().max().unwrap()
}

fn main() {
    let grid = parse_input();

    println!("{}", search(&grid, Point{ row: 0, col: 0 }, Dir::Right));
    println!("{}", maximize_count(&grid));
}
