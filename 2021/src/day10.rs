use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum CheckResult {
    Ok,
    Incomplete { completion: String },
    Corrupted { first_incorrect: char },
}

fn check_line(line: String) -> CheckResult {
    let mut stack = Vec::new();
    for ch in line.chars() {
        if let '(' | '[' | '{' | '<' = ch {
            stack.push(ch);
        } else {
            match stack.last().cloned() {
                Some(m) if m == matching(ch) => {
                    stack.pop();
                }
                _ => return CheckResult::Corrupted {
                    first_incorrect: ch
                }
            }
        }
    }
    if stack.is_empty() {
        CheckResult::Ok
    } else {
        let s: String = stack.iter().rev().map(|ch| matching(*ch)).collect();
        CheckResult::Incomplete {
            completion: s
        }
    }
}

fn matching(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',

        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn completion_score(s: String) -> i64 {
    s.chars().map(|ch| {
        match ch {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0
        }
    }).fold(0, |acc, v| acc * 5 + v)
}

fn task1() {
    let f = File::open("data/input10.txt").unwrap();
    let result: i32 = BufReader::new(f)
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            match check_line(line) {
                CheckResult::Corrupted { first_incorrect } => Some(score(first_incorrect)),
                _ => None,
            }
        })
        .sum();
    println!("result1: {}", result)
}

fn task2() {
    let f = File::open("data/input10.txt").unwrap();
    let mut scores: Vec<i64> = BufReader::new(f)
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            match check_line(line) {
                CheckResult::Incomplete { completion } => Some(completion_score(completion)),
                _ => None,
            }
        }).collect();
    println!("scores: {:?}", scores);
    scores.sort();
    println!("result2: {}", scores[scores.len() / 2])
}

fn main() {
    task1();
    task2();
}