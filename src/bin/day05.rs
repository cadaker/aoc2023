use std::cmp::min;
use std::io;
use aoc2023::utils::grab_numbers;

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

#[derive(Clone, PartialEq, Debug)]
struct Range {
    begin: i64,
    end: i64, // one-past-the-end
}

struct Step {
    mappings: Vec<Mapping>,
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

    for s in &mut mappings {
        s.mappings.sort_by_key(|m| m.src_start);
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

fn map_value_range(range: &Range, mappings: &[Mapping]) -> Vec<Range> {
    // Mappings must be sorted by src_start
    let mut ret = Vec::new();
    let mut pos = range.begin;
    for m in mappings {
        if pos < m.src_start && pos < range.end {
            let end = min(range.end, m.src_start);
            ret.push(Range { begin: pos, end });
            pos = end;
        }

        if m.src_start <= pos && pos < m.src_start + m.len && pos < range.end {
            let end = min(range.end, m.src_start + m.len);
            ret.push(Range { begin: m.dst_start + (pos - m.src_start), end: m.dst_start + (end - m.src_start)});
            pos = end;
        }
    }
    if pos < range.end {
        ret.push(Range { begin: pos, end: range.end });
    }
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_value_range() {
        fn mapping(dst_start: i64, src_start: i64, len: i64) -> Mapping { Mapping { dst_start, src_start, len }}
        assert_eq!(map_value_range(&Range{ begin: 10, end: 20 }, &[]),
                   vec![Range{ begin: 10, end: 20}]);
        assert_eq!(map_value_range(&Range{ begin: 10, end: 20 }, &[mapping(0, 3, 4)]),
                   vec![Range{ begin: 10, end: 20}]);
        assert_eq!(map_value_range(&Range{ begin: 10, end: 20 }, &[mapping(1, 8, 4)]),
                   vec![Range{ begin: 3, end: 5}, Range{ begin: 12, end: 20}]);
        assert_eq!(map_value_range(&Range{ begin: 10, end: 20 }, &[mapping(1, 12, 4)]),
                   vec![Range{ begin: 10, end: 12}, Range{ begin: 1, end: 5 }, Range{ begin: 16, end: 20}]);
        assert_eq!(map_value_range(&Range{ begin: 10, end: 20 }, &[mapping(1, 18, 4)]),
                   vec![Range{ begin: 10, end: 18}, Range{ begin: 1, end: 3 }]);
        assert_eq!(map_value_range(&Range{ begin: 10, end: 20 }, &[mapping(1, 20, 4)]),
                   vec![Range{ begin: 10, end: 20}]);
        assert_eq!(map_value_range(&Range{ begin: 10, end: 20 },
                                   &[
                                       mapping(111, 11, 2),
                                       mapping(115, 15, 2),
                                       mapping(119, 19, 3)]),
                   vec![Range{ begin: 10, end: 11}, Range{ begin: 111, end: 113}, Range{ begin: 13, end: 15},
                        Range{ begin: 115, end: 117}, Range{ begin: 17, end: 19}, Range{ begin: 119, end: 120}]);
    }
}

fn seed_ranges_to_locations(seeds: &[Range], steps: &[Step]) -> Vec<Range> {
    let mut ranges = Vec::from(seeds);

    for s in steps {
        let mut new_ranges = Vec::new();
        for r in ranges {
            new_ranges.append(map_value_range(&r, &s.mappings).as_mut());
        }
        ranges = new_ranges;
    }
    return ranges;
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let (seeds, steps) = parse_input(&lines);

    let locations: Vec<i64> = seeds.iter().map(|s| seed_to_location(*s, &steps)).collect();
    println!("{}", locations.iter().min().unwrap());

    let seed_ranges = {
        let mut r: Vec<Range> = Vec::new();
        let mut i = 0;
        while i < seeds.len() {
            r.push(Range { begin: seeds[i], end: seeds[i] + seeds[i+1]});
            i += 2;
        }
        r
    };

    let location_ranges = seed_ranges_to_locations(&seed_ranges, &steps);
    println!("{}", location_ranges.iter().min_by_key(|r| r.begin).unwrap().begin);
}
