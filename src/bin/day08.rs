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

fn follow(instructions: &str, map: &Map, start: &str) -> usize {
    let mut count = 0;
    let mut pos = &String::from(start);

    while !pos.ends_with("Z") {
        let dir: char = instructions.chars().nth(count % instructions.len()).unwrap();
        if dir == 'L' {
            pos = &map.get(pos).unwrap().0;
        } else {
            pos = &map.get(pos).unwrap().1;
        }
        count += 1;
    }
    count
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a > 0 {
        (a, b) = (b % a, a);
    }
    b
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::gcd;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 3), 3);
        assert_eq!(gcd(3, 3), 3);
        assert_eq!(gcd(3, 3), 3);
        assert_eq!(gcd(15, 18), 3);
        assert_eq!(gcd(77, 19), 1);
    }
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let instructions: String = lines[0].clone();
    let map = read_map(&lines[2..]);

    println!("{}", follow(&instructions, &map, "AAA"));

    let starts: Vec<String> = map.keys().filter(|&s| s.ends_with("A")).cloned().collect();

    // We can make a simplifying assumption, that each start only leads to one single ending
    let lengths: Vec<usize> = starts.iter().map(|s| follow(&instructions, &map, s)).collect();

    println!("{}", lengths.iter().cloned().reduce(lcm).unwrap())
}
