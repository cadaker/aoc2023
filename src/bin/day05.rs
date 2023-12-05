use std::io;
use regex::Regex;

struct Mapping {
    dst_start: i64,
    src_start: i64,
    len: i64,
}

impl Mapping {
    fn maps(&self, val: i64) -> bool {
        self.src_start <= val && val < self.src_start + self.len
    }

    fn map(&self, val: i64) -> i64 {
        if self.maps(val) {
            self.dst_start + (val - self.src_start)
        } else {
            val
        }
    }
}

struct Step {
    mappings: Vec<Mapping>,
}

fn grab_numbers(line: &str) -> Vec<i64> {
    let pattern = Regex::new("(\\d+)").unwrap();
    pattern
        .captures_iter(line)
        .map(|c| c.get(1).unwrap())
        .map(|s| s.as_str().parse::<i64>().unwrap())
        .collect()
}

fn parse_input(lines: &[String]) -> (Vec<i64>, Vec<Step>) {
    let seeds = grab_numbers(&lines[0]);

    let mut mappings: Vec<Step> = Vec::new();
    mappings.push(Step { mappings: Vec::new() });
    let mut i = 3usize;
    while i < lines.len() {
        let nums = grab_numbers(&lines[i]);
        if nums.len() == 0 {
            i += 2;
            mappings.push(Step { mappings: Vec::new() });
        } else {
            i += 1;
            mappings.last_mut().unwrap().mappings.push(Mapping { dst_start: nums[0], src_start: nums[1], len: nums[2] });
        }
    }
    return (seeds, mappings)
}

fn map_value(value: i64, mappings: &[Mapping]) -> i64 {
    for m in mappings {
        if m.maps(value) {
            return m.map(value)
        }
    }
    return value;
}

fn seed_to_location(seed: i64, steps: &[Step]) -> i64 {
    let mut value = seed;
    for s in steps {
        value = map_value(value, &s.mappings);
    }
    return value;
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let (seeds, steps) = parse_input(&lines);

    let locations: Vec<i64> = seeds.iter().map(|s| seed_to_location(*s, &steps)).collect();
    println!("{}", locations.iter().min().unwrap());
}
