use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

use itertools::Itertools;

type Pattern = (char, char);

#[derive(Debug)]
struct Task {
    initial: String,
    productions: HashMap<Pattern, char>
}

fn read_input() -> Task {
    let f = File::open("data/input14.txt").unwrap();
    let mut lines = BufReader::new(f).lines().map(|line| line.unwrap());
    let initial = lines.by_ref().next().unwrap().to_string();
    let productions = lines.skip(1).map(|line| {
        let (pattern, insertition) = line.split_once("->").unwrap();
        (pattern.trim().chars().tuples().next().unwrap(), insertition.trim().chars().next().unwrap())

    }).collect();
    Task {
        initial,
        productions
    }
}

fn apply(s: String, productions: &HashMap<Pattern, char>) -> String {
    let mut result = String::new();
    for t in s.chars().tuple_windows::<(char, char)>() {
        result.push(t.0);
        match productions.get(&t) {
            Some(ch) => result.push(*ch),
            None => {},
        }
    }
    result.push(s.chars().last().unwrap());
    result
}

fn part1(steps: usize) -> usize {
    let task = read_input();
    let mut result = task.initial;
    for step in 0..steps {
        result = apply(result, &task.productions);
    }
    let counts = result.chars().counts();
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    max - min
}

fn part2(steps: usize) -> usize {
    let task = read_input();
    let mut state = task.initial.chars().tuple_windows::<(char, char)>().counts();
    for step in 0..steps {
        let mut next_state = state.clone();
        for (p, ch) in &task.productions {
            if let Some(count) = state.get(p) {
                *next_state.entry(*p).or_insert(0) -= count;
                *next_state.entry((p.0, *ch)).or_insert(0) += count;
                *next_state.entry((*ch, p.1)).or_insert(0) += count;
            }
        }
        state = next_state;
    }

    let mut counts = state.iter().fold(HashMap::new(), |mut counts, (p, count)| {
        *counts.entry(p.0).or_insert(0) += count;
        *counts.entry(p.1).or_insert(0) += count;
        counts
    });
    *counts.get_mut(&task.initial.chars().next().unwrap()).unwrap() += 1;
    *counts.get_mut(&task.initial.chars().last().unwrap()).unwrap() += 1;
    counts.values_mut().for_each(|v| *v /= 2);
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    max - min
}

fn main() {
//    println!("part1: {}", part1(10));
    println!("part2: {}", part2(40));
}