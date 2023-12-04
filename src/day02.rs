use std::{str::FromStr, cmp::max};

use nom::{IResult, bytes::complete::tag, sequence::{terminated, tuple, delimited}, character::complete::{multispace1, digit1, alpha1}, combinator::{map_res}, multi::{separated_list0, separated_list1}, Parser, Finish, error::Error};

#[derive(Debug)]
struct Turn {
    red: u32,
    green: u32,
    blue: u32
}

#[derive(Debug)]
struct Game {
    id: u32,
    turns: Vec<Turn>
}

impl Game {
    fn is_possible(&self) -> bool {
        self.turns.iter().all(|Turn { red, green, blue }| {
            *red <= 12 && *green <= 13 && *blue <= 14
        })
    }
    
    fn min_power(&self) -> u32 {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        self.turns.iter().for_each(|t| {
            min_red = max(min_red, t.red);
            min_green = max(min_green, t.green);
            min_blue = max(min_blue, t.blue);
        });
        min_red * min_green * min_blue
    }
}

fn turn_parser(input: &str) -> IResult<&str, Turn> {
    let num = map_res(digit1, str::parse);
    let color_num = tuple((terminated(num, multispace1), alpha1));
    let (input, colors) = separated_list1(terminated(tag(","), multispace1), color_num).parse(input)?;
    let mut turn = Turn {
        red: 0,
        green: 0,
        blue: 0,
    };
    colors.into_iter().for_each(|(count, color): (u32, &str)| {
        match color {
            "blue" => { turn.blue = count },
            "green" => { turn.green = count },
            "red" => { turn.red = count }
            _ => panic!()
        }
    });
    return Ok((input, turn));
}

fn game_parser(input: &str) -> IResult<&str, Game> {
    let game_id = map_res(digit1, str::parse);
    let game_id_prefix = delimited(terminated(tag("Game"), multispace1),
                                   game_id,
                                   terminated(tag(":"), multispace1));
    let turns = separated_list0(terminated(tag(";"), multispace1), turn_parser);
    tuple((game_id_prefix, turns)).map(|(id, turns)| {
        Game {
            id,
            turns
        }
    }).parse(input)
}

impl FromStr for Game {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match game_parser(s).finish() {
            Ok((_remaining, game)) => Ok(game),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn part1(input: &str) -> u32 {
    input.lines().map(|line| { line.parse::<Game>().unwrap() })
                 .filter(|g| g.is_possible())
                 .map(|g| g.id)
                 .sum()
}

fn part2(input: &str) -> u32 {
    input.lines().map(|line| { line.parse::<Game>().unwrap() })
                 .map(|g| g.min_power())
                 .sum()
}

fn main() {
    let input = include_str!("../data/day02.txt");
    println!("Part1: {:?}", part1(input));
    println!("Part2: {:?}", part2(input));
}