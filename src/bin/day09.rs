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

fn integrate_pre(diffs: &[Sequence]) -> i64 {
    let mut ret = 0;
    // next + prev = first ==> next = first - prev
    for s in diffs.iter().rev() {
        ret = s.first().unwrap() - ret;
    }
    ret
}

fn predict(s: &Sequence) -> (i64, i64) {
    let mut stack: Vec<Sequence> = Vec::new();
    stack.push(s.clone());
    while !all_zero(&stack.last().unwrap()) {
        stack.push(diff(&stack.last().unwrap()))
    }
    (integrate_pre(&stack), integrate(&stack))
}

fn main () {
    let input = parse_input();

    let pre_post_predictions: Vec<(i64, i64)> = input.iter().map(predict).collect();
    println!("{}", pre_post_predictions.iter().map(|(_,post)| post).sum::<i64>());
    println!("{}", pre_post_predictions.iter().map(|(pre,_)| pre).sum::<i64>());
}
