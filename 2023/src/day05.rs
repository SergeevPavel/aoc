use std::{str::FromStr, ops::Range};

use nom::{IResult, Finish, bytes::complete::{tag, take_while}, sequence::{preceded, separated_pair, terminated, tuple}, character::{complete::{multispace0, digit1, multispace1, newline, space1, space0}, is_alphabetic}, multi::{separated_list0, many1}, combinator::map_res, Parser};

#[derive(Debug)]
struct Interval {
    src_start: i64,
    dst_start: i64,
    len: i64
}

#[derive(Debug)]
struct Mapping {
    name: String,
    intervals: Vec<Interval>
}

impl Mapping {
    fn apply(self: &Self, i: i64) -> i64 {
        for int in &self.intervals {
            let offset = i - int.src_start;
            if offset > 0 && offset < int.len {
                return int.dst_start + offset;
            }
        }
        return i;
    }
    
    fn apply_range(self: &Self, r: Range<i64>) -> Vec<Range<i64>> {
        todo!()
    }
}

#[derive(Debug)]
struct Input {
    seeds: Vec<i64>,
    mappings: Vec<Mapping>
}

impl Input {
    fn seeds_ranges(self: &Self) -> Vec<Range<i64>> {
        self.seeds.windows(2).map(|s| s[0]..(s[0] + s[1])).collect()
    }
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |out: &str| i64::from_str_radix(out, 10)).parse(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let mapping_header = terminated(take_while(|ch: char| is_alphabetic(ch as u8) || ch == '-'), tag(" map:"));
    let interval = tuple((terminated(parse_i64, space1),
                          terminated(parse_i64, space1),
                          terminated(parse_i64, space0))).map(|(dst_start, src_start,  len)| Interval {
        src_start,
        dst_start,
        len
    });
    let mapping = separated_pair(mapping_header, newline, separated_list0(newline, interval)).map(|(name, intervals)| {
        Mapping {
            name: name.to_string(),
            intervals
        }
    });
    let seeds = preceded(tag("seeds:"),
                         preceded(space1,
                                  separated_list0(space1, parse_i64)));
    tuple((seeds, many1(newline), separated_list0(many1(newline), mapping))).map(|(seeds, _, mappings)| {
        Input {
            seeds,
            mappings
        }
    }).parse(input)
}

impl FromStr for Input {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_input(s).finish() {
            Ok((_remaining, game)) => Ok(game),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn part1(input: &str) {
    let input = input.parse::<Input>().unwrap();
    let result = input.seeds.iter().map(|seed| {
        input.mappings.iter().fold(*seed, |seed, m| m.apply(seed))
    }).min();
    println!("Part1: {:?}", result);
}

fn part2(input: &str) {
    let input = input.parse::<Input>().unwrap();
    let init = input.seeds_ranges();
    let location_ranges = input.mappings.iter().fold(init, |ranges, m| {
        ranges.into_iter().flat_map(|r| m.apply_range(r)).collect()
    });
    let result = location_ranges.iter().min_by_key(|r| r.start);
    println!("Part1: {:?}", result);
}

fn main() {
    let input = include_str!("../data/day05.txt");
    part1(input);
    part2(input);
}