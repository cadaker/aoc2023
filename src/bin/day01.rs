use std::io;

fn find_substrings<'a>(haystack: &'a str, needles: &Vec<&str>) -> Vec<&'a str> {
    let mut ret = Vec::<&'a str>::new();
    for i in 0..haystack.len() {
        for needle in needles {
            if haystack[i..].starts_with(needle) {
                ret.push(&haystack[i..i + needle.len()]);
                break;
            }
        }
    }
    return ret;
}

fn parse_digit(s: &str) -> i64 {
    return match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => s.parse::<i64>().unwrap()
    };
}

fn main() {
    let mut sum_part1 = 0i64;
    let mut sum_part2 = 0i64;
    let num_pattern: Vec<&str> = "1|2|3|4|5|6|7|8|9".split("|").collect();
    let extended_num_pattern: Vec<&str> = "1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine".split("|").collect();

    loop {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_ok() && line.len() > 0 {
            let nums = find_substrings(&line, &num_pattern);
            let ext_nums = find_substrings(&line, &extended_num_pattern);

            if nums.len() > 0 {
                sum_part1 += parse_digit(nums[0]) * 10 + parse_digit(nums[nums.len() - 1]);
            }
            if ext_nums.len() > 0 {
                sum_part2 += parse_digit(ext_nums[0]) * 10 + parse_digit(ext_nums[ext_nums.len() - 1]);
            }
        } else {
            break;
        }
    }
    println!("{}", sum_part1);
    println!("{}", sum_part2);
}
