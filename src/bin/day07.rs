use std::cmp::{Ordering,Ord};
use regex::Regex;
use std::io;
use crate::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

struct Hand {
    cards: String,
    bid: i64,
}

type Count = Vec<(i64, char)>;

struct Counter {
    counts: Count,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Counter {
    fn new() -> Counter { Counter{ counts: Vec::new() } }
    fn inc(&mut self, ch: char) {
        for entry in &mut self.counts {
            if entry.1 == ch {
                entry.0 += 1;
                return;
            }
        }
        self.counts.push((1, ch))
    }
}

fn parse_input() -> Vec<Hand> {
    let pattern = Regex::new("([AKQJT2-9]{5}) ([0-9]+)").unwrap();
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect();

    lines.into_iter().map(|line| {
        let c = pattern.captures(&line).unwrap();
        Hand { cards: String::from(c.get(1).unwrap().as_str()), bid: c.get(2).unwrap().as_str().parse().unwrap() }
    })
        .collect()
}

fn count_hand(hand: &Hand, jokers: bool) -> (Count, i64) {
    let mut n_jokers = 0;
    let mut counter = Counter::new();
    for ch in hand.cards.chars() {
        if jokers && ch == 'J' {
            n_jokers += 1;
        } else {
            counter.inc(ch);
        }
    }
    counter.counts.sort_by(|p1, p2| {
        p1.0.cmp(&p2.0).reverse()
            .then_with(|| rank_card(p1.1).cmp(&rank_card(p2.1)).reverse())
    });
    (counter.counts, n_jokers)
}

fn rank_card(card: char) -> i64 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 99999
    }
}

fn rank_card_jokers(card: char) -> i64 {
    if card == 'J' {
        1
    } else {
        rank_card(card)
    }
}

fn rank_hand(hand: &Hand, jokers: bool) -> HandType {
    let (counts, n_jokers) = count_hand(hand, jokers);
    if n_jokers == 5 {
        return FiveOfAKind;
    }

    let (max_n, _) = counts[0];
    if max_n + n_jokers == 5 {
        FiveOfAKind
    } else if max_n + n_jokers == 4 {
        FourOfAKind
    } else if max_n + n_jokers == 3 {
        let (second_n, _) = counts[1];
        return if second_n == 2 {
            FullHouse
        } else {
            ThreeOfAKind
        }
    } else if max_n + n_jokers == 2 {
        let (second_n, _) = counts[1];
        return if second_n == 2 {
            TwoPair
        } else {
            OnePair
        }
    } else {
        HighCard
    }
}


fn compare_hands(h1: &Hand, h2: &Hand) -> Ordering {
    let type1 = rank_hand(h1, false);
    let type2 = rank_hand(h2, false);
    type1.cmp(&type2)
        .then_with(|| {
            let v1: Vec<i64> = h1.cards.chars().map(rank_card).collect();
            let v2: Vec<i64> = h2.cards.chars().map(rank_card).collect();
            v1.cmp(&v2)
        })
}

fn compare_hands_jokers(h1: &Hand, h2: &Hand) -> Ordering {
    let type1 = rank_hand(h1, true);
    let type2 = rank_hand(h2, true);
    type1.cmp(&type2)
        .then_with(|| {
            let v1: Vec<i64> = h1.cards.chars().map(rank_card_jokers).collect();
            let v2: Vec<i64> = h2.cards.chars().map(rank_card_jokers).collect();
            v1.cmp(&v2)
        })
}

fn winnings(hands: &[Hand]) -> i64 {
    hands.iter()
        .enumerate()
        .map(|(i, hand)| ((i+1) as i64) * hand.bid)
        .sum()
}

fn main() {
    let mut hands = parse_input();

    hands.sort_by(|h1, h2| compare_hands(h1, h2));
    println!("{}", winnings(&hands));

    hands.sort_by(|h1, h2| compare_hands_jokers(h1, h2));
    println!("{}", winnings(&hands));
}
