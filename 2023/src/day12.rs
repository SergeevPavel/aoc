use std::collections::HashMap;

use itertools::{repeat_n, intersperse};


#[derive(Debug)]
struct Task {
    row: Vec<char>,
    groups: Vec<usize>,
}

fn parse(input: &str) -> Vec<Task> {
    input.lines().map(|line| {
        let (row, groups) = line.trim().split_once(" ").unwrap();
        let row = row.trim().chars().collect();
        let groups = groups.split(",").map(|n| n.parse().unwrap()).collect();
        Task {
            row,
            groups
        }
    }).collect()
}

fn can_arrange_at(row: &Vec<char>, pos: usize, group_size: usize) -> bool {
    let is_group_fit = pos + group_size <= row.len() && row[pos..(pos + group_size)].iter().all(|ch| *ch != '.');
    if !is_group_fit {
        return false;
    }
    let till_the_end_or_blank = pos + group_size == row.len() || row[pos + group_size] != '#';
    if !till_the_end_or_blank {
        return false;
    }
    return true;
}

fn arrangments(task: &Task, row_pos: usize, group_pos: usize, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    if let Some(result) = memo.get(&(row_pos, group_pos)) {
        return *result
    }
    let result = if group_pos == task.groups.len() {
        if row_pos >= task.row.len() || task.row[row_pos..].iter().all(|ch| *ch != '#') {
            1
        } else {
            0
        }
    } else if row_pos >= task.row.len() {
        0
    } else {
        let group_size = task.groups[group_pos];
        match task.row[row_pos] {
            '.' => {
                arrangments(task, row_pos + 1, group_pos, memo)
            }
            '#' => {
                if can_arrange_at(&task.row, row_pos, group_size) {
                    arrangments(task, row_pos + group_size + 1, group_pos + 1, memo)
                } else {
                    0
                }
            }
            '?' => {
                let place_dot_arrangments = arrangments(task, row_pos + 1, group_pos, memo);
                let place_sharp_arrangements = if can_arrange_at(&task.row, row_pos, group_size) {
                    arrangments(task, row_pos + group_size + 1, group_pos + 1, memo) 
                } else {
                    0
                };
                place_dot_arrangments + place_sharp_arrangements
            }
            c => unreachable!("Unexpected {}", c)
        }
    };
    memo.insert((row_pos, group_pos), result);
    result
}

fn unfold(task: &Task) -> Task {
    let row: Vec<char> = intersperse(repeat_n(task.row.iter(), 5), vec!['?'].iter()).flatten().copied().collect();
    let groups = task.groups.repeat(5);
    Task {
        row,
        groups
    }
}

fn part1(input: &str) {
    let input = parse(input);
    let result: u64 = input.into_iter().map(|task| {
        let mut memo = HashMap::new();
        let a = arrangments(&task, 0, 0, &mut memo);
//        println!("Task: {:?} a = {:?}", task, a);
        a
    }).sum();
    println!("Part1: {:?}", result);
}

fn part2(input: &str) {
    let input = parse(input);
    let result: u64 = input.iter().map(unfold).map(|task| {
        let mut memo = HashMap::new();
        let a = arrangments(&task, 0, 0, &mut memo);
//        println!("Unfolded task: {:?} a = {:?}", task, a);
        a
    }).sum();
    println!("Part2: {:?}", result);
}

fn main() {
    let input = include_str!("../data/day12.txt");
    part1(input);
    part2(input);
}