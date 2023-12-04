use std::{io::BufRead, collections::{HashMap, BTreeSet}};

type Indicator = BTreeSet<char>;

#[derive(Debug)]
struct Task {
    patterns: Vec<Indicator>,
    state: Vec<Indicator>,
}

fn read_input() -> Vec<Task> {
    let f = std::fs::File::open("data/input08.txt").expect("file not found");
    std::io::BufReader::new(f).lines().map(|line| {
        let line = line.unwrap();
        let (patterns, state) = line.split_once("|").unwrap();
        Task {
            patterns: patterns.trim().split(" ").map(|s| s.chars().collect::<BTreeSet<_>>()).collect(),
            state: state.trim().split(" ").to_owned().map(|s| s.chars().collect::<BTreeSet<_>>()).collect()
        }
    }).collect()
}

fn task1() {
    let input = read_input();
    let result: usize = input.iter().map(|task| {
        task.state.iter().filter(|s| {
            let l = s.len();
            l == 2 || l == 4 || l == 3 || l == 7
        }).count()
    }).sum();
    println!("result_1: {:?}", result)
}

fn by_segments_count(patterns: &Vec<Indicator>, segments_count: usize) -> impl Iterator<Item = Indicator> + '_ {
    patterns.iter().filter(move |p| p.len() == segments_count).cloned()
}

fn filter_by147_intersection<I>(patterns: &Vec<Indicator>,
                                iter: I,
                                d1_int: usize,
                                d4_int: usize,
                                d7_int: usize) -> impl Iterator<Item = Indicator>
  where I: Iterator<Item = Indicator> {
    let d1 = find_digit(patterns, 1);
    let d4 = find_digit(patterns, 4);
    let d7 = find_digit(patterns, 7);
    iter.filter(move |ind| ind.intersection(&d1).count() == d1_int &&
                           ind.intersection(&d4).count() == d4_int &&
                           ind.intersection(&d7).count() == d7_int)
}

fn find_digit(patterns: &Vec<Indicator>, digit: u8) -> Indicator {
    match digit {
        1 => {
            by_segments_count(patterns, 2).next().unwrap()
        }
        4 => {
            by_segments_count(patterns, 4).next().unwrap()
        }
        7 => {
            by_segments_count(patterns, 3).next().unwrap()
        }
        8 => {
            by_segments_count(patterns, 7).next().unwrap()
        }
        2 => {
            let segments5 = by_segments_count(patterns, 5);
            filter_by147_intersection(patterns, segments5, 1, 2, 2).next().unwrap()
        }
        3 => {
            let segments5 = by_segments_count(patterns, 5);
            filter_by147_intersection(patterns, segments5, 2, 3, 3).next().unwrap()
        }
        5 => {
            let segments5 = by_segments_count(patterns, 5);
            filter_by147_intersection(patterns, segments5, 1, 3, 2).next().unwrap()
        }
        6 => {
            let segments6 = by_segments_count(patterns, 6);
            filter_by147_intersection(patterns, segments6, 1, 3, 2).next().unwrap()
        }
        9 => {
            let segments6 = by_segments_count(patterns, 6);
            filter_by147_intersection(patterns, segments6, 2, 4, 3).next().unwrap()
        }
        0 => {
            let segments6 = by_segments_count(patterns, 6);
            filter_by147_intersection(patterns, segments6, 2, 3, 3).next().unwrap()
        }
        _ => unreachable!("digit should be between 0 and 9")
    }
}

fn task2() {
    let input = read_input();
    let result: i32 = input.iter().map(|task| {
        let mut mapping: HashMap<Indicator, u8> = Default::default();
        for digit in 0..=9 {
            let indicator = find_digit(&task.patterns, digit);
            mapping.insert(indicator, digit);
        }
        let mut code: i32 = 0;
        for position in &task.state {
            code *= 10;
            code += mapping[position] as i32;
        }
        code
    }).sum();
    println!("result2: {}", result)
}

fn main() {
//    task1()
    task2()
}