use std::io;
use regex::Regex;

fn main() {
    let mut sum = 0i64;
    let num_pattern = Regex::new("[0-9]").unwrap();
    loop {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_ok() && line.len() > 0 {
            let nums: Vec<_> = num_pattern.find_iter(&line).map(|m| m.as_str()).collect();
            let val = (nums[0].to_owned() + nums[nums.len()-1]).parse::<i64>().unwrap();
            sum += val;
        } else {
            break;
        }
    }
    println!("{}", sum);
}
