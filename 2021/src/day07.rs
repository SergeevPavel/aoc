use std::fs::read_to_string;

fn dist(x: i32) -> i32 {
    return (x * x + x.abs()) / 2;
}

fn cost(positions: &[i32], guess: i32) -> i32 {
    positions.iter().map(|p| dist(*p - guess)).sum::<i32>()
}

fn main() {
    let mut positions: Vec<i32> = read_to_string("data/input07.txt").unwrap().split(",").map(|n| n.parse::<i32>().unwrap()).collect();
    positions.sort();
    let median = positions[positions.len() / 2];
    println!("result1: {:?}", positions.iter().map(|p| (*p - median).abs()).sum::<i32>());
    let average = (positions.iter().map(|p| *p as f32).sum::<f32>() / positions.len() as f32).round() as i32;
    let result2 = (median.min(average)..=median.max(average)).map(|guess| {
        cost(&positions, guess)
    }).min();
    println!("result2: {:?}", result2);
}