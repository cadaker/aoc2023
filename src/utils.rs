use regex::Regex;

pub fn grab_numbers(line: &str) -> Vec<i64> {
    let pattern = Regex::new("(\\d+)").unwrap();
    pattern
        .captures_iter(line)
        .map(|c| c.get(1).unwrap())
        .map(|s| s.as_str().parse::<i64>().unwrap())
        .collect()
}
