use aoc2023::grid::Point;
use aoc2023::utils::stdio_lines;

fn parse_input() -> Vec<Point> {
    let mut ret = Vec::new();
    for (row, line) in stdio_lines().iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                ret.push(Point{ row: row as i32, col: col as i32 });
            }
        }
    }
    ret
}

fn expand_space(stars: &[Point], amount: i32) -> Vec<Point> {
    let mut star_list = Vec::from(stars);

    star_list.sort_by_key(|s| s.col);
    let mut dc = 0;
    let mut col = star_list[0].col;
    for s in &mut star_list {
        if s.col > col + 1 {
            dc += (s.col - col - 1) * (amount - 1);
        }
        col = s.col;
        s.col += dc;
    }

    star_list.sort_by_key(|s| s.row);
    let mut dr = 0;
    let mut row = star_list[0].row;
    for s in &mut star_list {
        if s.row > row + 1 {
            dr += (s.row - row - 1) * (amount - 1);
        }
        row = s.row;
        s.row += dr;
    }

    star_list
}

fn abs(x: i32) -> i32 {
    if x >= 0 {
        x
    } else {
        -x
    }
}

fn dist(p1: &Point, p2: &Point) -> i64 {
    abs(p1.row - p2.row) as i64 + abs(p1.col - p2.col) as i64
}

fn pairwise_distance_sum(stars: &[Point]) -> i64 {
    let mut sum = 0;
    for (i, s1) in stars.iter().enumerate() {
        for s2 in stars[i+1..].iter() {
            sum += dist(s1, s2)
        }
    }
    sum
}

fn main() {
    let input_stars = parse_input();

    let stars = expand_space(&input_stars, 2);

    println!("{}", pairwise_distance_sum(&stars));

    let sparse_stars = expand_space(&input_stars, 1_000_000);

    println!("{}", pairwise_distance_sum(&sparse_stars));
}

