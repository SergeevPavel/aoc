use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::{HashSet, VecDeque},
};

use itertools::{iproduct, Itertools};

struct Field {
    data: Vec<u32>,
    width: i32,
    height: i32,
}

impl Field {
    fn get(&self, x: i32, y: i32) -> Option<u32> {
        let result = if 0 <= x && x < self.width && 0 <= y && y < self.height {
            Some(self.data[(x + y * self.width) as usize])
        } else {
            None
        };
        result
    }

    fn neighbours(&self, x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
        let width = self.width;
        let height = self.height;
        [(0, -1), (-1, 0), (1, 0), (0, 1)].iter().filter_map(move |(dx, dy)| {
            let x = x + dx;
            let y = y + dy;
            if 0 <= x && x < width && 0 <= y && y < height {
                Some((x, y))
            } else {
                None
            }
        })
    }

    fn is_low_point(&self, x: i32, y: i32) -> bool {
        if let Some(v) = self.get(x, y) {
            for (nx, ny) in self.neighbours(x, y) {
                if self.get(nx, ny).unwrap() <= v {
                    return false;
                }
            }
            return true;
        } else {
            return false;
        }
    }
}

fn read_input() -> Result<Field, String> {
    let f = File::open("data/input09.txt").unwrap();
    let mut data = Vec::<u32>::new();
    let mut width = 0;
    let mut height = 0;
    for line in BufReader::new(f).lines() {
        let mut current_width = 0;
        match line {
            Ok(line) => {
                for d in line.chars().map(|ch| ch.to_digit(10)) {
                    match d {
                        Some(d) => {
                            current_width += 1;
                            data.push(d);
                        }
                        None => return Err("Unexpected symbol".to_string()),
                    }
                }
            }
            Err(err) => return Err(err.to_string()),
        }
        width = current_width;
        height += 1;
    }
    return Ok(Field {
        data,
        width,
        height,
    });
}

fn task1() {
    let field = read_input().unwrap();
    let mut result = 0;
    for x in 0..field.width {
        for y in 0..field.height {
            if field.is_low_point(x, y) {
//                println!("lp: {:?}", field.get(x, y));
                result += field.get(x, y).unwrap() + 1;
            }
        }
    }
    println!("result: {}", result);
}

fn basin_size(field: &Field,  x: i32, y: i32) -> u32 {
    let mut enqueued: HashSet<(i32, i32)> = Default::default();
    let mut queue = VecDeque::new();
    let mut result = 0;
    enqueued.insert((x, y));
    queue.push_back((x, y));
    loop {
        if let Some((x, y)) = queue.pop_front() {
//            println!("pop: {} {}", x, y);
            result += 1;
            for (nx, ny) in field.neighbours(x, y) {
                if !enqueued.contains(&(nx, ny)) && field.get(nx, ny) != Some(9) {
                    queue.push_back((nx, ny));
                    enqueued.insert((nx, ny));
                }
            }
        } else {
            break;
        }
    }
//    println!("at {} {} result {}", x, y, result);
    return result;
}

fn task2() {
    let field = read_input().unwrap();
    let mut result = iproduct!(0..field.width, 0..field.height)
        .filter(|(x, y)| field.is_low_point(*x, *y))
        .map(|(x, y)| basin_size(&field, x, y))
        .sorted().rev().take(3).reduce(|x, y| x * y);
    println!("result2: {:?}", result);
}

fn main() {
    task1();
    task2();
}