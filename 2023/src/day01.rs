use std::ops::Index;

fn part1(input: &str) {
    let result: u32 = input.lines().map(|line| {
        let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        digits.first().unwrap() * 10 + digits.last().unwrap()
    }).sum();
    println!("Part1: {:?}", result);
}

fn part2(input: &str) {
    let spelling = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    
    let pat_to_number = |p: &str| {
        spelling.iter().position(|p1| *p1 == p)
          .or(digits.iter().position(|p1| *p1 == p))
          .unwrap() as u32 + 1
    };
    
    let result: u32 = input.lines().map(|line| {
        let (_, pat_first) = spelling.iter().chain(digits.iter()).filter_map(|p| {
            line.find(p).map(|offset| (offset, p))
        }).min_by_key(|(offset, _)| *offset).unwrap();
        let (_, pat_last) = spelling.iter().chain(digits.iter()).filter_map(|p| {
            line.rfind(p).map(|offset| (offset, p))
        }).max_by_key(|(offset, _)| *offset).unwrap();
        pat_to_number(pat_first) * 10 + pat_to_number(pat_last)
    }).sum();
    println!("Part2: {:?}", result);
}

fn main() {
    let input = include_str!("../data/day01.txt");
    part1(input);
    part2(input);
}