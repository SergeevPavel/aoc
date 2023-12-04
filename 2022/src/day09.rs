use std::{
    collections::{HashSet},
    ops::{Add, AddAssign, Div, Sub}, io::{stdin, Read},
};

static INPUT: &str = include_str!("../data/day09.txt");

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<i32> for Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Direction {
    fn to_offset(&self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: 1 },
            Direction::Down => Point { x: 0, y: -1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

struct Rope {
    knotts: Vec<Point>,
}

impl Rope {
    fn new(len: usize) -> Self {
        assert!(len > 0);
        Rope {
            knotts: vec![Point { x: 0, y: 0 }; len],
        }
    }

    fn apply(&mut self, dir: &Direction) {
        self.knotts[0] += dir.to_offset();
        for i in 1..self.knotts.len() {
            let offset = Rope::next_offset(self.knotts[i - 1], self.knotts[i]);
            self.knotts[i] += offset;
        }
    }

    fn next_offset(prev: Point, next: Point) -> Point {
        let dist = prev - next;
        let mut offset = dist / 2;

        if offset.x != 0 && offset.y == 0 {
            offset.y = dist.y
        }
        if offset.y != 0 && offset.x == 0 {
            offset.x = dist.x
        }

        offset
    }

    fn tail(&self) -> Point {
        *self.knotts.last().unwrap()
    }
}

fn parse_input() -> Vec<(Direction, i32)> {
    INPUT
        .lines()
        .map(|l| {
            let (dir, steps) = l.split_once(" ").unwrap();
            let dir = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unexpected value: {}", dir),
            };
            let steps: i32 = steps.parse().unwrap();
            (dir, steps)
        })
        .collect()
}

fn solve(len: usize) -> usize {
    let input = parse_input();
    let mut visited = HashSet::new();
    let mut rope = Rope::new(len);
    visited.insert(rope.tail());
    for (dir, count) in input {
        for _ in 0..count {
//            print_rope(&rope, 6, 6);
            rope.apply(&dir);
            visited.insert(rope.tail());
        }
    }
    return visited.len();
}

fn print_rope(rope: &Rope, width: i32, height: i32) {
    print!("{}[2J", 27 as char);
    for y in (0..height).rev() {
        for x in 0..width {
            let pos = rope.knotts.iter().position(|p| *p == Point { x, y });
            let ch = match pos {
                Some(0) => 'H',
                Some(idx) => char::from_digit((idx).try_into().unwrap(), 10).unwrap(),
                None => '.',
            };
            print!("{}", ch);
        }
        println!();
    }
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    println!("Result1: {:?}", solve(2));
    println!("Result2: {:?}", solve(10));
}
