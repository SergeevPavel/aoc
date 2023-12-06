mod utils;

use nom::{IResult, multi::separated_list0, character::complete::{space1, newline, multispace1}, sequence::{separated_pair, preceded, terminated}, Parser, bytes::{complete::tag, complete::take_while1}, combinator::recognize};

use crate::utils::parse_i64;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64
}

impl Race {
    fn ways_to_win(&self) -> i64 {
        (0..=self.time).map(|button_time| {
            let speed = button_time;
            let remaining_time = self.time - button_time;
            speed * remaining_time
        }).filter(|distance| *distance > self.distance)
          .count()
          .try_into().unwrap()
    }
}

fn parse_input1(input: &str) -> IResult<&str, Vec<Race>> {
    let times_parser = preceded(terminated(tag("Time:"), multispace1), separated_list0(space1, parse_i64));
    let distances_parser = preceded(terminated(tag("Distance:"), multispace1), separated_list0(space1, parse_i64));
    separated_pair(times_parser, newline, distances_parser).map(|(times, dists)| {
        times.into_iter().zip(dists.into_iter()).map(|(time, distance)| {
            Race {
                time,
                distance
            }
        }).collect()
    }).parse(input)
}

fn parse_broken_number(input: &str) -> IResult<&str, i64> {
    take_while1(|ch: char| {
        ch.is_digit(10) || ch == ' '
    }).map(|s: &str| i64::from_str_radix(&s.replace(" ", ""), 10).unwrap()).parse(input)
}

fn parse_input2(input: &str) -> IResult<&str, Race> {
    let time_parser = preceded(terminated(tag("Time:"), multispace1), parse_broken_number);
    let distance_parser = preceded(terminated(tag("Distance:"), multispace1), parse_broken_number);
    separated_pair(time_parser, newline, distance_parser).map(|(time, distance)| {
        Race {
            time,
            distance
        }
    }).parse(input)
}

fn part1(input: &str) {
    let input = parse_input1(input).unwrap().1;
    let result = input.iter().map(|r| r.ways_to_win()).reduce(|a, b| a * b);
    println!("Part1: {:?}", result);
}

fn part2(input: &str) {
    let input = parse_input2(input).unwrap().1;
    let result = input.ways_to_win();
    println!("Part2: {:?}", result);
}

fn main() {
    let input = include_str!("../data/day06.txt");
    part1(input);
    part2(input);
}