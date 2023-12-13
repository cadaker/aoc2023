use std::cmp::min;
use aoc2023::utils::stdio_lines;
use aoc2023::grid::{Grid, GridBuilder};

type Pattern = Grid<char>;

fn parse_input() -> Vec<Pattern> {
    let mut builders = Vec::new();
    let lines = stdio_lines();

    builders.push(GridBuilder::new());
    for line in lines {
        let builder = builders.last_mut().unwrap();
        if line.len() > 0 {
            for ch in line.chars() {
                builder.add(ch);
            }
            builder.eol();
        } else {
            builders.push(GridBuilder::new());
        }
    }

    builders.into_iter().map(|b| b.finish()).collect()
}

fn encode_rows(pattern: &Pattern) -> Vec<i64> {
    let mut ret = Vec::new();

    for r in 0..pattern.height() {
        let mut enc = 0;
        for c in 0..pattern.width() {
            enc = enc * 2 + if *pattern.get(r, c) == '#' { 1 } else { 0 };
        }
        ret.push(enc);
    }

    ret
}

fn encode_cols(pattern: &Pattern) -> Vec<i64> {
    let mut ret = Vec::new();

    for c in 0..pattern.width() {
        let mut enc = 0;
        for r in 0..pattern.height() {
            enc = enc * 2 + if *pattern.get(r, c) == '#' { 1 } else { 0 };
        }
        ret.push(enc);
    }

    ret
}

// Check if the reflection line goes just after pos.
fn is_reflection(encoded_pattern: &[i64], pos: usize) -> bool {
    if pos + 1 == encoded_pattern.len() {
        return false;
    }
    let n = min(pos + 1, encoded_pattern.len() - pos - 1);
    for delta in 1..=n {
        if encoded_pattern[pos + 1 - delta] != encoded_pattern[pos + delta] {
            return false;
        }
    }
    true
}

fn find_reflection(encoded_pattern: &[i64]) -> Option<usize> {
    for i in 1..encoded_pattern.len() {
        if is_reflection(encoded_pattern, i-1) {
            return Some(i-1);
        }
    }
    None
}

#[derive(PartialEq)]
enum Axis {
    Vertical,
    Horizontal,
}

fn find_reflection_line(pattern: &Pattern) -> Option<(Axis, usize)> {
    let row_match = find_reflection(&encode_rows(pattern));
    let col_match = find_reflection(&encode_cols(pattern));
    if row_match.is_some() {
        Some((Axis::Horizontal, row_match.unwrap()))
    } else if col_match.is_some() {
        Some((Axis::Vertical, col_match.unwrap()))
    } else {
        None
    }
}

fn main() {
    let input = parse_input();

    println!("{}", input.iter()
        .map(find_reflection_line)
        .map(|x| x.unwrap())
        .map(|(axis, n)| (n + 1) * if axis == Axis::Horizontal { 100 } else { 1 })
        .sum::<usize>());
}
