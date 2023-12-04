use aoc2023::grid::{Point, Grid, GridBuilder};
use std::io;

struct Tag {
    start: Point,
    len: i32,
}

fn find_tags(grid: &Grid<char>) -> Vec<Tag> {
    let mut ret = Vec::new();

    for row in 0..grid.height() {
        let mut col = 0i32;
        while col < grid.width() {
            while col < grid.width() && !grid.get(row, col).is_ascii_digit() {
                col += 1;
            }
            if col < grid.width() {
                let start = Point{row, col};
                while col < grid.width() && grid.get(row, col).is_ascii_digit() {
                    col += 1;
                }
                let len = col - start.col;
                ret.push(Tag{start, len})
            }
        }
    }

    return ret;
}

fn is_symbol(ch: char) -> bool {
    return !ch.is_ascii_digit() && ch != '.';
}

fn neighbours(tag: &Tag, grid: &Grid<char>) -> Vec<Point> {
    let mut ret = Vec::new();
    for c in (tag.start.col - 1)..=(tag.start.col + tag.len) {
        if grid.contains(tag.start.row - 1, c) {
            ret.push(Point{row: tag.start.row - 1, col: c});
        }
        if grid.contains(tag.start.row + 1, c) {
            ret.push(Point{row: tag.start.row + 1, col: c});
        }
    }
    if grid.contains(tag.start.row, tag.start.col - 1) {
        ret.push(Point{row: tag.start.row, col: tag.start.col - 1});
    }
    if grid.contains(tag.start.row, tag.start.col + tag.len) {
        ret.push(Point{row: tag.start.row, col: tag.start.col + tag.len});
    }
    return ret;
}

fn is_part_number(grid: &Grid<char>, tag: &Tag) -> bool {
    for p in neighbours(tag, grid) {
        if is_symbol(*grid.getp(&p)) {
            return true;
        }
    }
    return false;
}

fn to_number(grid: &Grid<char>, tag: &Tag) -> i64 {
    let mut ret = 0;
    for c in tag.start.col..tag.start.col+tag.len {
        ret = ret * 10 + (*grid.get(tag.start.row, c) as i64 - '0' as i64)
    }
    return ret;
}

fn make_gear_grid(grid: &Grid<char>) -> Grid<Vec<i64>> {
    let mut gear_grid_builder = GridBuilder::<Vec<i64>>::new();
    for _row in 0..grid.height() {
        for _col in 0..grid.width() {
            gear_grid_builder.add(vec![]);
        }
        gear_grid_builder.eol();
    }
    return gear_grid_builder.finish();
}

fn mark_gears(grid: &Grid<char>, gear_grid: &mut Grid<Vec<i64>>, tags: &[Tag]) {
    for tag in tags {
        for p in neighbours(tag, grid) {
            if *grid.getp(&p) == '*' {
                gear_grid.mutgetp(&p).push(to_number(grid, tag));
            }
        }
    }
}

fn sum_gear_ratios(gear_grid: &Grid<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for row in 0..gear_grid.height() {
        for col in 0..gear_grid.width() {
            let entry = gear_grid.get(row, col);
            if entry.len() == 2 {
                sum += entry[0] * entry[1];
            }
        }
    }
    return sum;
}

fn main() {
    let mut grid_builder = GridBuilder::<char>::new();
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        for ch in line.chars() {
            grid_builder.add(ch);
        }
        grid_builder.eol();
    }
    let grid = grid_builder.finish();

    let tags = find_tags(&grid);

    let parts_sum: i64 = tags.iter().filter(|t| is_part_number(&grid, *t)).map(|t| to_number(&grid, t)).sum();
    println!("{}", parts_sum);

    let mut gear_grid = make_gear_grid(&grid);
    mark_gears(&grid, &mut gear_grid, &tags);
    println!("{}", sum_gear_ratios(&gear_grid));
}
