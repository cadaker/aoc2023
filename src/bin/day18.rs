use regex::Regex;
use aoc2023::utils::stdio_lines;
use aoc2023::grid::Point;
use aoc2023::dir::{Dir, step, cw, ccw};

fn parse_input() -> Vec<(Dir, i64)> {
    let pattern = Regex::new("([RULD]) (\\d+) \\(#([0-9a-f]{6})\\)").unwrap();
    stdio_lines().iter()
        .map(|line| pattern.captures(line).unwrap())
        .map(|c| {
            let (_, [dir, steps, _colour]) = c.extract();
            let d = match dir {
                "R" => Dir::Right,
                "U" => Dir::Up,
                "L" => Dir::Left,
                "D" => Dir::Down,
                _ => panic!("bad direction"),
            };
            (d, steps.parse().unwrap())
        })
        .collect()
}

fn trace_path(turns: &[(Dir, i64)]) -> Vec<(Point, Dir)> {
    let mut p = Point{ row: 0, col: 0 };
    let mut ret = Vec::new();
    for (dir, steps) in turns {
        for _ in 0..*steps {
            ret.push((p, *dir));
            p = step(p, *dir);
        }
    }
    ret
}

fn fix_offsets(points: &[(Point, Dir)]) -> Vec<(Point, Dir)> {
    let min_row = points.iter().map(|(p, _d)| p.row).min().unwrap();
    let min_col = points.iter().map(|(p, _d)| p.col).min().unwrap();
    points.iter().map(|(p, d)| (Point{ row: p.row - min_row, col: p.col - min_col }, *d)).collect()
}


fn area(path: &[(Point, Dir)]) -> i64 {
    // Green's theorem: dA = sum y * dx

    // This counts the area on the right side of the curve
    let mut inner_area = 0i64;
    let mut path_area_quarters = 0i64;
    for i in 0..path.len() {
        let j = (i + 1) % path.len();
        let (p, dir) = &path[i];
        let (_, dir2)  = &path[j];

        let y = p.row as i64;
        let dx = match *dir {
            Dir::Left => 1,
            Dir::Right => -1,
            _ => 0,
        };
        inner_area += y * dx;
        if *dir2 == cw(*dir) {
            path_area_quarters += 1;
        } else if *dir2 == ccw(*dir) {
            path_area_quarters += 3;
        } else {
            path_area_quarters += 2;
        }
    }
    let inner_squares = inner_area - path_area_quarters / 4;
    if inner_squares < 0 {
        -inner_squares + path.len() as i64
    } else {
        inner_squares + path.len() as i64
    }
}

fn main() {
    let input = parse_input();

    let path = fix_offsets(&trace_path(&input));

    println!("{}", area(&path));
}
