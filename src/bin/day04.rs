use regex::Regex;
use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;

struct Card {
    winning: Vec<i64>,
    ticket: Vec<i64>,
}

fn card_matches(card: &Card) -> usize {
    let winning: HashSet<i64> = HashSet::from_iter(card.winning.iter().cloned());
    let ticket: HashSet<i64> = HashSet::from_iter(card.ticket.iter().cloned());
    return winning.intersection(&ticket).count();
}

fn score(card: &Card) -> i64 {
    let matches = card_matches(card);
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

    println!("{}", cards.iter().map(score).sum::<i64>());

    let mut card_counts: Vec<usize> = Vec::new();
    for _ in &cards {
        card_counts.push(1);
    }
    for i in 0..cards.len() {
        let matches = card_matches(&cards[i]);
        for j in i + 1 ..= i + matches {
            if j < cards.len() {
                card_counts[j] += card_counts[i];
            }
        }
    }
    println!("{}", card_counts.iter().sum::<usize>());
}
