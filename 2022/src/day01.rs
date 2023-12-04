#![feature(binary_heap_into_iter_sorted)]

use std::{fs::File, io::{BufReader, BufRead}, collections::BinaryHeap};



fn main() {
    let f = File::open("data/day01.txt").unwrap();
    let mut current = 0;
    let mut q = BinaryHeap::<i32>::new();
    BufReader::new(f).lines().for_each(|line| {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            q.push(current);
            current = 0;
        } else {
            let meal_calories = line.parse::<i32>().unwrap();
            current += meal_calories;
        }
    });
    println!("Result One: {:?}", q.peek().unwrap());
    println!("Result Two: {:?}", q.into_iter_sorted().take(3).sum::<i32>())
}