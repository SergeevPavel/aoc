use std::{io::BufRead, ops::Range};
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn from_string(point_str: &str) -> Self {
        return point_str.split_once(",").map(|point|
            Point {
                x: point.0.parse().unwrap(),
                y: point.1.parse().unwrap()
        }).unwrap()
    }
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point
}

impl Segment {
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_diagonal(&self) -> bool {
        (self.end.x - self.start.x).abs() == (self.end.y - self.start.y).abs()
    }

    fn points(&self) -> Vec<Point> {
        if self.is_vertical() {
            let min_y = self.start.y.min(self.end.y);
            let max_y = self.start.y.max(self.end.y);
            return (min_y..=max_y).map(|y| Point { x: self.start.x, y }).collect()
        }
        if self.is_horizontal() {
            let min_x = self.start.x.min(self.end.x);
            let max_x = self.start.x.max(self.end.x);
            return (min_x..=max_x).map(|x| Point { x, y: self.start.y }).collect()
        }
        if self.is_diagonal() {
            let l = (self.end.x - self.start.x).abs();
            let dx = (self.end.x - self.start.x) / l;
            let dy = (self.end.y - self.start.y) / l;
            let mut points: Vec<Point> = Vec::with_capacity(l as usize);
            for i in 0..=l {
                points.push(Point {
                    x: self.start.x + dx * i,
                    y: self.start.y + dy * i
                })
            }
            return points
        }
        panic!("Only horizontal and vertcal lines supported")
    }
}

fn read_input() -> Vec<Segment> {
    let f = std::fs::File::open("data/input05.txt").expect("File not found");
    return std::io::BufReader::new(f).lines().map(|line| {
        let line = line.unwrap();
        let (start, end) = line.split_once(" -> ").unwrap();
        Segment {
            start: Point::from_string(start),
            end: Point::from_string(end)
        }
    }).collect()
}

fn task1() {
    let input = read_input();
    let points = input.iter()
        .filter(|segment| segment.is_horizontal() || segment.is_vertical())
        .flat_map(|segment| segment.points());
    println!("result1 : {:?}", points.counts().iter().filter(|(k, v)| **v > 1).count());
}

fn task2() {
    let input = read_input();
    let points = input.iter()
        .filter(|segment| segment.is_horizontal() || segment.is_vertical() || segment.is_diagonal())
        .flat_map(|segment| segment.points());
    println!("result2 : {:?}", points.counts().iter().filter(|(k, v)| **v > 1).count());
}

fn main() {
    task1();
    task2();
}