use std::collections::HashSet;

use itertools::Itertools;

static INPUT: &str = include_str!("../data/day03.txt");

fn priority(ch: char) -> u32 {
    return if ch.is_ascii_lowercase() {
        (ch as u32) - ('a' as u32) + 1
    } else if ch.is_ascii_uppercase() {
        (ch as u32) - ('A' as u32) + 27
    } else {
        unreachable!()
    }
}

fn solve1() {
    let result = INPUT.lines().map(|line| {
        let line = line.trim();
        let len = line.len();
        let (part1, part2) = line.split_at(len / 2);
        let items1 = HashSet::<char>::from_iter(part1.chars());
        let items2 = HashSet::<char>::from_iter(part2.chars());
        items1.intersection(&items2).map(|ch| {
            priority(*ch)
        }).sum::<u32>()
    }).sum::<u32>();
    println!("Result1: {:?}", result);
}

fn solve2() {
    let result = INPUT.lines().chunks(3).into_iter().flat_map(|chunk| {
        chunk.map(|line| {
            line.chars().collect::<HashSet<_>>()
        }).reduce(|s1, s2| s1.intersection(&s2).cloned().collect::<HashSet<_>>()).unwrap()
    }).map(|ch| priority(ch)).sum::<u32>();
    println!("Result2: {:?}", result);
}

fn main() {
    solve1();
    solve2();
}