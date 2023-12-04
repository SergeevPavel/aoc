use std::{str::FromStr, collections::{HashSet, VecDeque}};

use nom::{IResult, Finish, sequence::{terminated, preceded, delimited, tuple}, character::complete::{multispace0, multispace1, digit1}, bytes::complete::tag, combinator::map_res, multi::separated_list0, Parser};

struct Card {
    id: u32,
    win_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn win_numbers_count(self: &Card) -> u32 {
        let numbers_set: HashSet<_> = self.numbers.iter().collect();
        let win_numbers_set: HashSet<_> = self.win_numbers.iter().collect();
        return win_numbers_set.intersection(&numbers_set).count().try_into().unwrap();
    }
}

impl FromStr for Card {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match card_parser(s).finish() {
            Ok((_remaining, game)) => Ok(game),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn card_parser(input: &str) -> IResult<&str, Card> {
    let card_id = preceded(terminated(tag("Card"), multispace1), map_res(digit1, str::parse));
    let card_id_terminator = terminated(tag(":"), multispace1);
    let groups_terminator = delimited(multispace1, tag("|"), multispace1);
    let win_numbers = separated_list0(multispace1, map_res(digit1, str::parse));
    let numbers = separated_list0(multispace1, map_res(digit1, str::parse));
    tuple((card_id, card_id_terminator, win_numbers, groups_terminator, numbers)).map(|(card_id, _, win_numbers, _, numbers)| {
        Card {
            id: card_id,
            win_numbers: win_numbers,
            numbers: numbers
        }
    }).parse(input)
}

fn part1(input: &str) {
    let result: u64 = input.lines().map(|line| {
        let card: Card = line.parse().unwrap();
        let we_have = card.win_numbers_count();
        if we_have > 0 {
            2u64.pow(we_have - 1)
        } else {
            0
        }
    }).sum();
    println!("Part1: {:?}", result);
}

fn part2(input: &str) {
    let mut cart_bring = VecDeque::<u64>::new();
    input.lines().map(|line| line.parse().unwrap()).rev().for_each(|card: Card| {
        let win_count: u64 = card.win_numbers_count().into();
        cart_bring.push_front(cart_bring.iter().take(win_count.try_into().unwrap()).sum::<u64>() + 1);
    });
    let result: u64 = cart_bring.iter().sum();
    println!("Part2: {:?}", result);
}

fn main() {
    let input = include_str!("../data/day04.txt");
    part1(&input);
    part2(&input);
}