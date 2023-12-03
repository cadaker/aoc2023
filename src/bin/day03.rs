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

fn is_part_number(grid: &Grid<char>, tag: &Tag) -> bool {
    let check = |row: i32, col: i32| -> bool {
        return grid.contains(row, col) && is_symbol(*grid.get(row, col));
    };
    for c in (tag.start.col-1)..=(tag.start.col+tag.len) {
        if check(tag.start.row-1, c) {
            return true;
        }
        if check(tag.start.row+1, c) {
            return true;
        }
    }
    if check(tag.start.row, tag.start.col-1) {
        return true;
    }
    if check(tag.start.row, tag.start.col + tag.len) {
        return true;
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
}
