use aoc2023::utils::stdio_lines;
use aoc2023::grid::{Grid, GridBuilder};

fn parse_input() -> Grid<char> {
    let mut builder = GridBuilder::new();
    for line in stdio_lines() {
        for ch in line.chars() {
            builder.add(ch);
        }
        builder.eol()
    }
    builder.finish()
}

fn copy_fixed(grid: &Grid<char>) -> Grid<char> {
    let mut builder = GridBuilder::new();
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            if *grid.get(row, col) == '#' {
                builder.add('#');
            } else {
                builder.add('.');
            }
        }
        builder.eol();
    }
    builder.finish()
}

fn roll_north(grid: &Grid<char>) -> Grid<char> {
    let mut new_grid = copy_fixed(grid);
    for col in 0..grid.width() {
        let mut rock_count = 0;
        for row in (-1..grid.height()).rev() {
            if row == -1 || *grid.get(row, col) == '#' {
                for r in row+1..row+1+rock_count {
                    *new_grid.mutget(r, col) = 'O';
                }
                rock_count = 0;
            } else if *grid.get(row, col) == 'O' {
                rock_count += 1;
            }
        }
    }

    new_grid
}

fn total_load_north(grid: &Grid<char>) -> i64 {
    let mut load = 0;
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            if *grid.get(row, col) == 'O' {
                load += (grid.height() - row) as i64;
            }
        }
    }

    load
}

fn main() {
    let input = parse_input();

    println!("{}", total_load_north(&roll_north(&input)));
}
