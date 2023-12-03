use std::io;
use regex::Regex;
use std::collections::HashMap;

fn parse_set(set_line: &str) -> HashMap<String, i64> {
    let clause_regex: Regex = Regex::new("(\\d+) (blue|red|green)").unwrap();
    let mut ret = HashMap::new();
    for cap in clause_regex.captures_iter(set_line) {
        let (_, [count, color]) = cap.extract();
        ret.insert(String::from(color), count.parse::<i64>().unwrap());
    };
    return ret;
}

fn parse_line(line: &str) -> (i64, Vec<HashMap<String, i64>>) {
    let line_regex: Regex = Regex::new("Game (\\d+): (.*)").unwrap();

    let captures = line_regex.captures(line).unwrap();
    let game_id = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let clauses: Vec<&str> = captures.get(2).unwrap().as_str().split(";").collect();

    return (game_id,
            clauses.iter().map(|clause| parse_set(*clause)).collect())
}

const RED_LIMIT: i64 = 12;
const GREEN_LIMIT: i64 = 13;
const BLUE_LIMIT: i64 = 14;

fn is_possible(sets: &[HashMap<String, i64>]) -> bool {
    for set in sets {
        if *set.get("red").unwrap_or(&0) > RED_LIMIT {
            return false
        } else if *set.get("green").unwrap_or(&0) > GREEN_LIMIT {
            return false
        } else if *set.get("blue").unwrap_or(&0) > BLUE_LIMIT {
            return false
        }
    }
    return true
}

fn minimal_counts(sets: &[HashMap<String, i64>]) -> (i64, i64, i64) {
    let reds = *sets.iter().map(|s| s.get("red").unwrap_or(&0)).max().unwrap_or(&0);
    let greens = *sets.iter().map(|s| s.get("green").unwrap_or(&0)).max().unwrap_or(&0);
    let blues = *sets.iter().map(|s| s.get("blue").unwrap_or(&0)).max().unwrap_or(&0);
    return (reds, greens, blues)
}

fn main () {
    let mut possible_games_sum = 0i64;
    let mut minimal_games_sum = 0i64;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        let (game_id, sets) = parse_line(&line);
        if is_possible(&sets) {
            possible_games_sum += game_id
        }
        let (reds, greens, blues) = minimal_counts(&sets);
        minimal_games_sum += reds * greens * blues;
    }
    println!("{}", possible_games_sum);
    println!("{}", minimal_games_sum);
}
