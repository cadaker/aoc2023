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

fn roll_west(grid: &Grid<char>) -> Grid<char> {
    let mut new_grid = copy_fixed(grid);
    for row in 0..grid.height() {
        let mut rock_count = 0;
        for col in (-1..grid.width()).rev() {
            if col == -1 || *grid.get(row, col) == '#' {
                for c in col+1..col+1+rock_count {
                    *new_grid.mutget(row, c) = 'O';
                }
                rock_count = 0;
            } else if *grid.get(row, col) == 'O' {
                rock_count += 1;
            }
        }
    }

    new_grid
}

fn roll_east(grid: &Grid<char>) -> Grid<char> {
    let mut new_grid = copy_fixed(grid);
    for row in 0..grid.height() {
        let mut rock_count = 0;
        for col in 0..=grid.width() {
            if col == grid.width() || *grid.get(row, col) == '#' {
                for c in col-rock_count..col {
                    *new_grid.mutget(row, c) = 'O';
                }
                rock_count = 0;
            } else if *grid.get(row, col) == 'O' {
                rock_count += 1;
            }
        }
    }

    new_grid
}

fn roll_south(grid: &Grid<char>) -> Grid<char> {
    let mut new_grid = copy_fixed(grid);
    for col in 0..grid.width() {
        let mut rock_count = 0;
        for row in 0..=grid.height() {
            if row == grid.height() || *grid.get(row, col) == '#' {
                for r in row-rock_count..row {
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

fn cycle(grid: &Grid<char>) -> Grid<char> {
    roll_east(&roll_south(&roll_west(&roll_north(grid))))
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

fn find_cycle<T, F>(val: T, func: F) -> (usize, usize)
    where F: Fn(T) -> T, T: Clone + PartialEq
{
    let mut x1 = func(val.clone());
    let mut x2 = func(x1.clone());
    while x1 != x2 {
        x1 = func(x1);
        x2 = func(func(x2));
    }

    let mut prefix_len = 0usize;
    x1 = val;
    while x1 != x2 {
        x1 = func(x1);
        x2 = func(x2);
        prefix_len += 1;
    }

    let mut cycle_len = 1usize;
    x1 = func(x1);
    while x1 != x2 {
        x1 = func(x1);
        cycle_len += 1;
    }

    (prefix_len, cycle_len)
}

fn main() {
    let mut grid = parse_input();

    println!("{}", total_load_north(&roll_north(&grid)));

    let (prefix_len, cycle_len) = find_cycle(grid.clone(), |g| cycle(&g));
    let iterations = prefix_len + ((1_000_000_000 - prefix_len) % cycle_len);

    for _i in 0..iterations {
        grid = cycle(&grid);
    }
    println!("{}", total_load_north(&grid));
}
