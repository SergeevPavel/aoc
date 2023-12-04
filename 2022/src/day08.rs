use std::{collections::HashSet, cmp::max};


static INPUT: &str = include_str!("../data/day08.txt");

fn solve1(input: &Vec<Vec<i8>>) {
    let mut visible = HashSet::<(usize, usize)>::new();
    let size = input.len();

    for i in 0..size {
        let mut max = -1;
        for j in 0..size {
            if input[i][j] > max {
                visible.insert((i, j));
                max = input[i][j];
            }
        }

        let mut max = -1;
        for j in (0..size).rev() {
            if input[i][j] > max {
                visible.insert((i, j));
                max = input[i][j];
            }
        }
    }

    for i in 0..size {
        let mut max = -1;
        for j in 0..size {
            if input[j][i] > max {
                visible.insert((j, i));
                max = input[j][i];
            }
        }

        let mut max = -1;
        for j in (0..size).rev() {
            if input[j][i] > max {
                visible.insert((j, i));
                max = input[j][i];
            }
        }
    }
    println!("Result1: {:?}", visible.len());
}

fn score(input: &Vec<Vec<i8>>, i: usize, j: usize) -> i32 {
    let size = input.len();
    let mut up = 0;
    for k in 1..size {
        if i < k {
            break;
        }
        up += 1;
        if input[i - k][j] >= input[i][j] {
            break;
        }
    }
    let mut down = 0;
    for k in 1..size {
        if i + k >= size {
            break;
        }
        down += 1;
        if input[i + k][j] >= input[i][j] {
            break;
        }
    }
    let mut left = 0;
    for k in 1..size {
        if j < k {
            break;
        }
        left += 1;
        if input[i][j - k] >= input[i][j] {
            break;
        }
    }
    let mut right = 0;
    for k in 1..size {
        if j + k >= size {
            break;
        }
        right += 1;
        if input[i][j + k] >= input[i][j] {
            break;
        }
    }
    up * down * left * right
}

fn solve2(input: &Vec<Vec<i8>>) {
    let size = input.len();
    let mut result = 0;
    for i in 0..size {
        for j in 0..size {
            result = max(score(input, i, j), result);
        }
    }
    println!("Result2: {:?}", result);
}

fn main() {
    let input: Vec<Vec<i8>> = INPUT.lines().map(|line| {
        line.chars().map(|ch| ch.to_digit(10).unwrap() as i8).collect()
    }).collect();
    solve1(&input);
    solve2(&input);
}