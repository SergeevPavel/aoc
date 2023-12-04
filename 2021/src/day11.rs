use std::{fs::File, io::{BufReader, BufRead}, cmp::{min}, collections::HashSet, fmt::Display};

#[derive(Debug)]
struct Field {
    data: Vec<u32>,
    width: i32,
    height: i32,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_str(&self.get(x, y).unwrap().to_string())?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
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

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut u32> {
        let result = if 0 <= x && x < self.width && 0 <= y && y < self.height {
            self.data.get_mut((x + y * self.width) as usize)
        } else {
            None
        };
        result
    }

    fn mut_cells(&mut self) -> impl Iterator<Item = &mut u32> {
        return self.data.iter_mut();
    }

    fn neighbours(&self, x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
        let width = self.width;
        let height = self.height;
        [(0, -1), (-1, 0), (1, 0), (0, 1),
         (-1, -1), (-1, 1), (1, -1), (1, 1)].iter().filter_map(move |(dx, dy)| {
            let x = x + dx;
            let y = y + dy;
            if 0 <= x && x < width && 0 <= y && y < height {
                Some((x, y))
            } else {
                None
            }
        })
    }
}

fn read_input() -> Result<Field, String> {
    let f = File::open("data/input11.txt").unwrap();
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

fn step(field: &mut Field) -> i32 {
    let mut flashes = 0;
    field.mut_cells().for_each(|cell| *cell += 1);
    let mut flashed = HashSet::new();
    loop {
        let mut should_continue = false;
        for x in 0..field.width {
            for y in 0..field.height {
                if field.get(x, y).unwrap() > 9 {
                    should_continue = true;
                    flashes += 1;
                    *field.get_mut(x, y).unwrap() = 0;
                    flashed.insert((x, y));
                    for (nx, ny) in field.neighbours(x, y) {
                        if !flashed.contains(&(nx, ny)) {
                            let n = field.get_mut(nx, ny).unwrap();
                            *n = min(*n + 1, 10);
                        }
                    }
                }
            }
        }
        if !should_continue { break; }
    }
    return flashes;
}

fn task1() {
    let mut input = read_input().unwrap();
    let mut result = 0;
//    println!("Before any steps:\n{}", input);
    for st in 1..=100 {
        result += step(&mut input);
//        println!("After step {}:\n{}", st, input);
    }
    println!("result1: {}", result);
}

fn task2() {
    let mut input = read_input().unwrap();
    let mut result = 0;
    println!("Before any steps:\n{}", input);
    loop {
        result += 1;
        if step(&mut input) == input.width * input.height {
            break;
        }
    }
    println!("result2: {}", result);
}


fn main() {
//    task1();
    task2();
}