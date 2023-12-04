use std::{io::BufRead, collections::HashMap};

use crate::day03::read_input;

mod day03;

fn main() {
    let lines = read_input().unwrap();
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for line in &lines {
        for (i, ch) in line.chars().rev().enumerate() {
            if ch == '1' {
                *counts.entry(i).or_insert(0) += 1;
            }
        }
    }
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for (i, cnt) in counts {
        if cnt > lines.len() / 2 {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }
    println!("result: {:?}", gamma * epsilon);
}