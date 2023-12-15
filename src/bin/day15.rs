use aoc2023::utils::stdio_lines;

fn parse_input() -> Vec<Vec<String>> {
    let mut ret = Vec::new();
    for line in stdio_lines() {
        ret.push(line.split(",").into_iter().map(String::from).collect());
    }
    ret
}

fn hash(s: &str) -> i64 {
    let mut h = 0;
    for ch in s.chars() {
        h = ((h + ch as i64) * 17) % 256
    }
    h
}

fn main() {
    let input = parse_input();
    println!("{}",
        input.iter()
            .map(|vec|
                vec.iter()
                    .map(|s| hash(s.as_str()))
                    .sum::<i64>())
            .sum::<i64>()
    )
}
