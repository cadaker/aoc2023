use regex::Regex;
use aoc2023::utils::stdio_lines;
use aoc2023::grid::Point;
use aoc2023::dir::{Dir, step, cw, ccw};

fn parse_input1(lines: &[String]) -> Vec<(Dir, i64)> {
    let pattern = Regex::new("([RULD]) (\\d+) \\(#([0-9a-f]{6})\\)").unwrap();
    lines.iter()
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

fn parse_hex(s: &str) -> i64 {
    let mut ret = 0;
    for ch in s.chars() {
        ret = ret * 16 + match ch {
            'a' | 'A' => 10,
            'b' | 'B' => 11,
            'c' | 'C' => 12,
            'd' | 'D' => 13,
            'e' | 'E' => 14,
            'f' | 'F' => 15,
            digit => digit.to_string().parse().unwrap(),
        }
    }
    ret
}

fn parse_input2(lines: &[String]) -> Vec<(Dir, i64)> {
    let pattern = Regex::new("([RULD]) (\\d+) \\(#([0-9a-f]{6})\\)").unwrap();
    lines.iter()
        .map(|line| pattern.captures(line).unwrap())
        .map(|c| {
            let (_, [_dir, _steps, colour]) = c.extract();
            let d = match colour.chars().last().unwrap() {
                '0' => Dir::Right,
                '1' => Dir::Down,
                '2' => Dir::Left,
                '3' => Dir::Up,
                _ => panic!("bad direction"),
            };
            let steps = parse_hex(&colour[..colour.len() - 1]);
            (d, steps)
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
    let input_lines = stdio_lines();

    let input1 = parse_input1(&input_lines);
    let path1 = trace_path(&input1);
    println!("{}", area(&path1));

    let input2 = parse_input2(&input_lines);
    let path2 = trace_path(&input2);
    println!("{}", area(&path2));
}
