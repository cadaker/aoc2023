use aoc2023::utils::{stdio_lines, grab_numbers};

type Sequence = Vec<i64>;

fn parse_input() -> Vec<Sequence> {
    stdio_lines().iter()
        .map(|s| s.as_str())
        .map(grab_numbers)
        .collect()
}

fn diff(s: &Sequence) -> Sequence {
    let mut ret = Sequence::new();
    for i in 1..s.len() {
        ret.push(s[i] - s[i-1]);
    }
    ret
}

fn all_zero(s: &Sequence) -> bool {
    return s.iter().all(|n| *n == 0)
}

fn integrate(diffs: &[Sequence]) -> i64 {
    let mut ret = 0;
    for s in diffs {
        ret = ret + s.last().unwrap();
    }
    ret
}

fn predict(s: &Sequence) -> i64 {
    let mut stack: Vec<Sequence> = Vec::new();
    stack.push(s.clone());
    while !all_zero(&stack.last().unwrap()) {
        stack.push(diff(&stack.last().unwrap()))
    }
    integrate(&stack)
}

fn main () {
    let input = parse_input();

    println!("{}", input.iter().map(predict).sum::<i64>());
}
