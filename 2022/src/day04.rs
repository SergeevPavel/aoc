use std::{ops::{RangeInclusive}};


static INPUT: &str = include_str!("../data/day04.txt");

fn intervals() -> impl Iterator<Item = (RangeInclusive<i32>, RangeInclusive<i32>)> {
    INPUT.lines().map(|line| {
        let (a, b) = line.split_once(",").unwrap();
        let parse_interval = |s: &str| {
            let (l, r) = s.split_once("-").unwrap();
            let l = l.parse::<i32>().unwrap();
            let r = r.parse::<i32>().unwrap();
            l..=r
        };
        let a = parse_interval(a);
        let b = parse_interval(b);
        (a, b)
    })
}

fn taks1() {
    let contains = |a: &RangeInclusive<i32>, b: &RangeInclusive<i32>| {
        a.contains(b.start()) && a.contains(b.end())
    };
    let result = intervals().filter(|(a, b)| {
        contains(a, b) || contains(b, a)
    }).count();
    println!("Result1: {}", result);
}

fn task2() {
    let overlaps = |a: &RangeInclusive<i32>, b: &RangeInclusive<i32>| {
        !(a.end() < b.start() || b.end() < a.start())
    };
    let result = intervals().filter(|(a, b)| {
        overlaps(&a, &b)
    }).count();
    println!("Result: {}", result);
}

fn main() {
    taks1();
    task2();
}