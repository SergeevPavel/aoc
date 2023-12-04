#![feature(exclusive_range_pattern)]

use std::collections::{HashMap, VecDeque, HashSet};

static INPUT: &str = include_str!("../data/day12.txt");

type Point = (i32, i32);
type Field = HashMap::<Point, i32>;

struct Input {
    start: Point,
    end: Point,
    field: Field
}

fn char_lvl(ch: char) -> i32 {
    ch as i32 - 'a' as i32
}

fn parse_input() -> Input {
    let mut field = Field::new();
    let mut start = None;
    let mut end = None;
    INPUT.lines().enumerate().for_each(|(i, line)| {
        line.trim().chars().enumerate().for_each(|(j, ch)| {
            let i: i32 = i.try_into().unwrap();
            let j: i32 = j.try_into().unwrap();
            match ch {
                'S' => {
                    start = Some((i, j));
                    field.insert((i, j), char_lvl('a'));
                }
                'E' => {
                    end = Some((i, j));
                    field.insert((i, j), char_lvl('z'));
                }
                'a'..='z' => {
                    field.insert((i, j), char_lvl(ch));
                }
                _ => panic!("Unexpected char: {}", ch)
            };
        });
    });
    Input { start: start.unwrap(), end: end.unwrap(), field }
}

fn neigbours(field: &Field, point: Point) -> impl Iterator<Item = Point> + '_ {
    [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().filter_map(move |(di, dj)| {
        let neigh = (point.0 + di, point.1 + dj);
        if field.contains_key(&neigh) && field[&neigh] - field[&point] <= 1 {
            Some(neigh)
        } else {
            None
        }
    })
}

fn bfs(field: &Field, start: Point, end: Point) -> Option<usize> {
    let mut visited = HashSet::<Point>::new();
    let mut queue = VecDeque::<(Point, usize)>::new();
    queue.push_back((start, 0));
    while let Some((next, dist)) = queue.pop_front() {
        if next == end {
            return Some(dist);
        }
        for neigh in neigbours(field, next) {
            if visited.insert(neigh) {
                queue.push_back((neigh, dist + 1));
            }
        }
    }
    return None;
}

fn solve1() {
    let Input { start, end, field } = parse_input();
    println!("Result1: {:?}", bfs(&field, start, end));
}

fn solve2() {
    let Input { start: _start, end, field } = parse_input();
    let result = field.iter().filter(|(_, v)| **v == 0).flat_map(|(start, _)| {
        bfs(&field, *start, end)
    }).min();
    println!("Result2: {:?}", result);
}

fn main() {
    solve1();
    solve2();
}