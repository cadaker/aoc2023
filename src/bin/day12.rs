use std::cmp::min;
use aoc2023::utils::{stdio_lines, grab_numbers};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

struct Record {
    springs: Vec<SpringState>,
    groups: Vec<i64>,
}

fn parse_line(line: &str) -> Record {
    let parts: Vec<&str> = line.split_ascii_whitespace().into_iter().collect();
    assert_eq!(parts.len(), 2);
    let springs = parts[0].chars().map(|ch| {
        match ch {
            '.' => SpringState::Operational,
            '#' => SpringState::Damaged,
            '?' => SpringState::Unknown,
            _ => panic!("invalid char"),
        }
    }).collect();
    let groups = grab_numbers(parts[1]);
    Record{ springs, groups }
}

fn parse_input() -> Vec<Record> {
    stdio_lines().iter().map(|line| parse_line(line)).collect()
}

fn can_be_operational(springs: &[SpringState]) -> bool {
    springs.iter().all(|s| *s != SpringState::Damaged)
}

fn can_be_damaged(springs: &[SpringState]) -> bool {
    springs.iter().all(|s| *s != SpringState::Operational)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_be_operational() {
        use SpringState::*;
        assert!(can_be_operational(&[]));
        assert!(can_be_operational(&[Operational]));
        assert!(can_be_operational(&[Unknown]));
        assert!(can_be_operational(&[Operational, Unknown]));
        assert!(can_be_operational(&[Unknown, Operational]));
        assert!(!can_be_operational(&[Damaged]));
        assert!(!can_be_operational(&[Operational, Damaged]));
        assert!(!can_be_operational(&[Unknown, Damaged]));
    }
}

fn count_options(record: &Record) -> usize {
    let mut table: HashMap<(usize, usize), usize> = HashMap::new();

    for spring in 0..=record.springs.len() {
        if can_be_operational(&record.springs[spring..]) {
            table.insert((spring, record.groups.len()), 1);
        } else {
            table.insert((spring, record.groups.len()), 0);
        }
    }

    for group in 0..record.groups.len() {
        table.insert((record.springs.len(), group), 0);
    }

    for spring in (0..record.springs.len()).rev() {
        for group in (0..record.groups.len()).rev() {
            let n = record.groups[group] as usize;

            let next_spring_after_damaged = min(spring + n + 1, record.springs.len());
            let can_place_damaged =
                spring + n <= record.springs.len() &&
                    can_be_damaged(&record.springs[spring..(spring + n)]) &&
                    (spring + n == record.springs.len() || record.springs[spring + n] != SpringState::Damaged);

            if record.springs[spring] == SpringState::Operational {
                table.insert((spring, group), *table.get(&(spring + 1, group)).unwrap());
            } else if record.springs[spring] == SpringState::Damaged && can_place_damaged {
                table.insert((spring, group), *table.get(&(next_spring_after_damaged, group + 1)).unwrap());
            } else if record.springs[spring] == SpringState::Unknown {
                table.insert((spring, group),
                             if can_place_damaged { *table.get(&(next_spring_after_damaged, group + 1)).unwrap() } else { 0 }
                                 +
                                 *table.get(&(spring + 1, group)).unwrap());
            } else {
                table.insert((spring, group), 0);
            }
        }
    }

    table[&(0,0)]
}

fn main() {
    let records = parse_input();

    println!("{}", records.iter().map(count_options).sum::<usize>());
}
