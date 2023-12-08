use std::io;
use std::collections::HashMap;
use regex::Regex;

type Map = HashMap<String, (String, String)>;

fn read_map(map_lines: &[String]) -> Map {
    let mut ret = HashMap::new();
    let pattern = Regex::new("([A-Z]{3}) = \\(([A-Z]{3}), ([A-Z]{3})\\)").unwrap();
    for line in map_lines {
        let cap = pattern.captures(line).unwrap();
        let (_, [first, left, right]) = cap.extract();
        ret.insert(String::from(first),(String::from(left), String::from(right)));
    }
    ret
}

fn follow(instructions: &str, map: &Map) -> usize {
    let mut count = 0;
    let mut pos = &String::from("AAA");

    while pos != "ZZZ" {
        if instructions.chars().nth(count % instructions.len()).unwrap() == 'L' {
            pos = &map.get(pos).unwrap().0;
        } else {
            pos = &map.get(pos).unwrap().1;
        }
        count += 1;
    }

    count
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let instructions: String = lines[0].clone();
    let map = read_map(&lines[2..]);

    println!("{}", follow(&instructions, &map));
}
