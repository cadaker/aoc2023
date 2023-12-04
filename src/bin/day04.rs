use regex::Regex;
use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;

struct Card {
    winning: Vec<i64>,
    ticket: Vec<i64>,
}

fn score(card: &Card) -> i64 {
    let winning: HashSet<i64> = HashSet::from_iter(card.winning.iter().cloned());
    let ticket: HashSet<i64> = HashSet::from_iter(card.ticket.iter().cloned());
    let matches = winning.intersection(&ticket).count();
    if matches == 0 {
        return 0
    } else {
        return i64::pow(2, (matches - 1) as u32)
    }
}

fn parse_card(line: &str) -> Card {
    let line_regex = Regex::new("Card\\s+\\d+: ([0-9 ]+) \\| ([0-9 ]+)").unwrap();
    let captures = line_regex.captures(line).unwrap();
    let winning_str = captures.get(1).unwrap().as_str();
    let ticket_str = captures.get(2).unwrap().as_str();
    let mut winning: Vec<i64> = winning_str.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut ticket: Vec<i64> = ticket_str.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
    winning.sort();
    ticket.sort();
    return Card{ winning, ticket };
}

fn main() {
    let cards: Vec<Card> = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| parse_card(s.as_str()))
        .collect();

    println!("{}", cards.iter().map(score).sum::<i64>())
}
