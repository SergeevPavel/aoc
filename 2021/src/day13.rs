use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

//type Point = (i32, i32);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
enum Fold {
    Horizontal { x: i32 },
    Vertical { y: i32 }
}

impl Fold {
    fn apply(&self, point: Point) -> Point {
        match self {
            Self::Horizontal { x } if point.x > *x => {
                Point { x: 2 * *x - point.x, y: point.y }
            }
            Self::Vertical { y } if point.y > *y => {
                Point { x: point.x, y: 2 * *y - point.y }
            }
            _ => point
        }
    }
}

#[derive(Debug)]
struct Task {
    points: HashSet<Point>,
    folds: Vec<Fold>
}

fn read_input() -> Task {
    let f = File::open("data/input13.txt").unwrap();
    let mut lines = BufReader::new(f).lines().map(|l| l.unwrap());
    let points = lines.by_ref().take_while(|line| line != "").map(|line| {
        let (x, y) = line.split_once(",").unwrap();
        Point {
            x: x.parse::<i32>().unwrap(),
            y: y.parse::<i32>().unwrap()
        }
    }).collect::<HashSet<_>>();
    let folds: Vec<_> = lines.map(|line| {
        let (direction, value) = line.split_once("=").unwrap();
        match direction {
            "fold along y" => Fold::Vertical { y: value.parse().unwrap() },
            "fold along x" => Fold::Horizontal { x: value.parse().unwrap() },
            _ => unreachable!()
        }
    }).collect();
    Task { points, folds }
}

fn part1() {
    let input = read_input();
    let fold = input.folds.first().unwrap();
    let points: HashSet<_> = input.points.iter().map(|p| fold.apply(*p) ).collect();
    println!("result1: {:?}", points.len());
}

fn part2() {
    let input = read_input();
    let points: HashSet<_> = input.points.iter().map(|p| {
        input.folds.iter().fold(*p, |p, f| f.apply(p))
    }).collect();
    print!("{}", termion::clear::All);
    for p in points {
        print!("{}#", termion::cursor::Goto((p.x + 1) as u16, (p.y + 1) as u16));
    }
}

fn main() {
    part1();
    part2();
}