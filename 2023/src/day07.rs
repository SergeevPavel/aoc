use std::cmp::{Reverse, min};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card {
    rank: u32,
}

impl Card {
    fn from_char(c: char, with_joker: bool) -> Self {
        match c {
            '2'..='9' => Card {
                rank: c.to_digit(10).unwrap(),
            },
            'T' => Card { rank: 10 },
            'J' if with_joker => Card { rank: 1 },
            'J' => Card { rank: 11 },
            'Q' => Card { rank: 12 },
            'K' => Card { rank: 13 },
            'A' => Card { rank: 14 },
            _ => panic!("It's not a card: {:?}", c),
        }
    }
    
    fn is_joker(&self) -> bool {
        self.rank == 1
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let counts_map = self.cards.iter().filter(|c| !c.is_joker()).counts();
        let jokers_count = self.cards.iter().filter(|c| c.is_joker()).count();
        let mut counts: Vec<_> = counts_map.values().collect();
        if counts.is_empty() {
            return HandType::FiveOfAKind
        }
        counts.sort_by_key(|w| Reverse(*w));
        match min(counts[0] + jokers_count, 5) {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                let jokers_count = jokers_count - (3 - counts[0]);
                if counts[1] + jokers_count >= 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            },
            2 => {
                let jokers_count = jokers_count - (2 - counts[0]);
                if counts[1] + jokers_count >= 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            },
            1 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str, with_joker: bool) -> Vec<(Hand, i64)> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            let hand = Hand {
                cards: cards
                    .chars()
                    .map(|c| Card::from_char(c, with_joker))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            };
            let bid = bid.trim().parse().unwrap();
            (hand, bid)
        })
        .collect()
}

fn score(mut hands: Vec<(Hand, i64)>) -> i64 {
    hands.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(&hand2));
    hands
        .iter()
        .map(|(_hand, bid)| bid)
        .enumerate()
        .map(|(rank, bid)| (rank + 1) as i64 * *bid)
        .sum()
}

fn part1(input: &str) {
    let hands = parse(input, false);
    println!("Part1: {:?}", score(hands));
}

fn part2(input: &str) {
    let hands = parse(input, true);
    println!("Part2: {:?}", score(hands));
}

fn main() {
    let input = include_str!("../data/day07.txt");
    part1(input);
    part2(input);
}
