
fn extrapolate_right(line: &Vec<i64>) -> i64 {
    if line.iter().all(|e| *e == 0) {
        return 0;
    }
    let next_line: Vec<_> = line.windows(2).map(|w| {
        w[1] - w[0]
    }).collect();
    let d = extrapolate_right(&next_line);
    line.last().unwrap() + d
}

fn extrapolate_left(line: &Vec<i64>) -> i64 {
    if line.iter().all(|e| *e == 0) {
        return 0;
    }
    let next_line: Vec<_> = line.windows(2).map(|w| {
        w[1] - w[0]
    }).collect();
    let d = extrapolate_left(&next_line);
    line.first().unwrap() - d
}

fn part1(input: &Vec<Vec<i64>>) {
    let result: i64 = input.iter().map(extrapolate_right).sum();
    println!("Part1: {}", result);
}

fn part2(input: &Vec<Vec<i64>>) {
    let result: i64 = input.iter().map(extrapolate_left).sum();
    println!("Part2: {}", result);
}

fn main() {
    let input = include_str!("../data/day09.txt");
    let input: Vec<_> = input.lines().map(|line| {
        line.split_whitespace().map(|num| num.parse().unwrap()).collect()
    }).collect();
    part1(&input);
    part2(&input);
}