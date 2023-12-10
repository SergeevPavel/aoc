use std::{collections::{HashSet, VecDeque}, fmt::{Display, Write}};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Start,
    Empty,
    HPipe,
    VPipe,
    NWBend,
    NEBend,
    SWBend,
    SEBend
}

impl Tile {
    fn symbol(&self) -> char {
        match self {
            Tile::VPipe => '┃',
            Tile::HPipe => '━',
            Tile::NEBend => '┏',
            Tile::NWBend => '┓',
            Tile::SWBend => '┛',
            Tile::SEBend => '┗',
            Tile::Start => 'S',
            Tile::Empty => '.',
        }
    }
}

fn parse_tile(c: char) -> Tile {
    match c {
        '|' => Tile::VPipe,
        '-' => Tile::HPipe,
        'L' => Tile::SEBend,
        'J' => Tile::SWBend,
        '7' => Tile::NWBend,
        'F' => Tile::NEBend,
        '.' => Tile::Empty,
        'S' => Tile::Start,
        _ => panic!("Unexpected: {:?}", c)
    }
}

type Point = (i32, i32);

struct Field {
    field: Vec<Tile>,
    width: i32,
    height: i32
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for h in 0..self.height {
            for w in 0..self.width {
                let t = self.get((w, h)).unwrap();
                f.write_char(t.symbol())?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Field {
    fn find(&self, r: Tile) -> Option<Point> {
        self.field.iter().position(|tile| *tile == r).map(|offset| {
            let offset: i32 = offset.try_into().unwrap();
            (offset % self.width, offset / self.width)
        })
    }
    
    fn get(&self, (x, y): Point) -> Option<Tile> {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            let offset: usize = (x + y * self.width).try_into().unwrap();
            Some(self.field[offset])
        } else {
            None
        }
    }
    
    fn get_mut(&mut self, (x, y): Point) -> Option<&mut Tile> {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            let offset: usize = (x + y * self.width).try_into().unwrap();
            Some(&mut self.field[offset])
        } else {
            None
        }
    }
    
    fn neighbours(&self, (x, y): Point) -> Vec<Point> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().map(|(dx, dy)| {
            (x + dx, y + dy)
        }).filter(|(x, y)| {
            0 <= *x && *x <= self.width && 0 <= *y && *y < self.height
        }).collect()
    }
}

fn pipe_loop(field: &Field) -> Vec<Point> {
    let start = field.find(Tile::Start).unwrap();
    let mut current = start;
    let mut visited = Vec::new();

    while let Some(next) = connected_tiles(&field, current).iter().filter(|p| visited.last() != Some(*p)).next() {
        visited.push(current);
        current = *next;
    }
    visited.push(current);
    return visited
}

fn part1(input: &str) {
    let field = parse_input(input);
    println!("{}", field);
    let visited = pipe_loop(&field);
    let result = visited.len().div_ceil(2);
    println!("Part1: {}", result);
}

fn cleanup_field(field: &mut Field, pipe_loop: &Vec<Point>) {
    let pipe_loop: HashSet<Point> = pipe_loop.iter().cloned().collect();
    for y in 0..field.height {
        for x in 0..field.width {
            if !pipe_loop.contains(&(x, y)) {
                *field.get_mut((x, y)).unwrap() = Tile::Empty
            }
        }
    }
}

fn bfs(field: &Field, start: Point, visited: &mut HashSet<Point>) {
    let mut queue = VecDeque::new();
    if !visited.contains(&start) && field.get(start) == Some(Tile::Empty) {
        queue.push_back(start);
        visited.insert(start);
    }
    while let Some(p) = queue.pop_front() {
        field.neighbours(p).iter().for_each(|n| {
            if !visited.contains(n) && field.get(*n) == Some(Tile::Empty) {
                queue.push_back(*n);
                visited.insert(*n);
            }
        })
    }
} 

fn print_field(field: &Field, left: &HashSet<Point>, right: &HashSet<Point>) {
    for y in 0..field.height {
        for x in 0..field.width {
            let c = match field.get((x, y)).unwrap() {
                Tile::Empty if left.contains(&(x, y)) => {
                    '#'
                },
                Tile::Empty if right.contains(&(x, y)) => {
                    '@'
                },
                t => t.symbol()
            };
            print!("{}", c);
        }
        println!();
    }
}

fn part2(input: &str) {
    let mut field = parse_input(input);
    let pipe_loop = pipe_loop(&field);
    cleanup_field(&mut field, &pipe_loop);
    let mut left_side = HashSet::new();
    let mut right_side = HashSet::new();
    pipe_loop.windows(2).for_each(|pts| {
        let (x1, y1) = pts[0];
        let (x2, y2) = pts[1];
        if x2 > x1 { // Moving Right
            bfs(&field, (x1, y1 - 1), &mut left_side);
            bfs(&field, (x1, y1 + 1), &mut right_side);
            
            bfs(&field, (x2, y2 - 1), &mut left_side);
            bfs(&field, (x2, y2 + 1), &mut right_side);
        } else if x2 < x1 { // Moving Left
            bfs(&field, (x1, y1 - 1), &mut right_side);
            bfs(&field, (x1, y1 + 1), &mut left_side);
            
            bfs(&field, (x2, y2 - 1), &mut right_side);
            bfs(&field, (x2, y2 + 1), &mut left_side);
        } else if y2 > y1 { // Moving Down
            bfs(&field, (x1 - 1, y1), &mut right_side);
            bfs(&field, (x1 + 1, y1), &mut left_side);
            
            bfs(&field, (x2 - 1, y2), &mut right_side);
            bfs(&field, (x2 + 1, y2), &mut left_side);
        } else if y2 < y1 { // Moving Up
            bfs(&field, (x1 - 1, y1), &mut left_side);
            bfs(&field, (x1 + 1, y1), &mut right_side);
            
            bfs(&field, (x2 - 1, y2), &mut left_side);
            bfs(&field, (x2 + 1, y2), &mut right_side);
        }
    });
    print_field(&mut field, &left_side, &right_side);
    println!("Left: {} Right: {}", left_side.len(), right_side.len());
}

fn parse_input(input: &str) -> Field {
    let mut field = Vec::<Tile>::new();
    let mut width = None;
    input.lines().for_each(|line| {
        let len_before = field.len();
        field.extend(line.chars().map(|c| parse_tile(c)));
        let len_after = field.len();
        match width {
            Some(width) => {
                assert!(width == (len_after - len_before))
            },
            None => {
                width = Some(len_after - len_before);
            },
        }
    });
    let height: i32 = (field.len() / width.unwrap()).try_into().unwrap();
    let width: i32 = width.unwrap().try_into().unwrap();
    Field {
        field,
        width,
        height
    }
}

fn main() {
    let input = include_str!("../data/day10.txt");
    part1(input);
    part2(input);
}

fn connected_tiles(field: &Field, (x, y): Point) -> Vec<Point> {
    let mut result = Vec::new();
    let current = field.get((x, y)).unwrap();
    match current {
        Tile::Start => {
            match field.get((x - 1, y)) {
                Some(t) if t == Tile::HPipe || t == Tile::NEBend || t == Tile::SEBend => {
                    result.push((x - 1, y));
                }
                _ => {},
            }
            match field.get((x + 1, y)) {
                Some(t) if t == Tile::HPipe || t == Tile::NWBend || t == Tile::SWBend => {
                    result.push((x + 1, y));
                }
                _ => {},
            }
            match field.get((x, y - 1)) {
                Some(t) if t == Tile::VPipe || t == Tile::NWBend || t == Tile::NEBend => {
                    result.push((x, y - 1));
                }
                _ => {},
            }
            match field.get((x, y + 1)) {
                Some(t) if t == Tile::VPipe || t == Tile::SWBend || t == Tile::SEBend => {
                    result.push((x, y + 1));
                }
                _ => {},
            }
        },
        Tile::HPipe => {
            match field.get((x - 1, y)) {
                Some(t) if t == Tile::HPipe || t == Tile::NEBend || t == Tile::SEBend => {
                    result.push((x - 1, y));
                }
                _ => {},
            }
            match field.get((x + 1, y)) {
                Some(t) if t == Tile::HPipe || t == Tile::NWBend || t == Tile::SWBend => {
                    result.push((x + 1, y));
                }
                _ => {},
            }
        },
        Tile::VPipe => {
            match field.get((x, y - 1)) {
                Some(t) if t == Tile::VPipe || t == Tile::NWBend || t == Tile::NEBend => {
                    result.push((x, y - 1));
                }
                _ => {},
            }
            match field.get((x, y + 1)) {
                Some(t) if t == Tile::VPipe || t == Tile::SWBend || t == Tile::SEBend => {
                    result.push((x, y + 1));
                }
                _ => {},
            }
        },
        Tile::NWBend => { // '┓'
            match field.get((x - 1, y)) {
            Some(t) if t == Tile::HPipe || t == Tile::NEBend || t == Tile::SEBend => {
                result.push((x - 1, y));
            }
                _ => {},
            }
            match field.get((x, y + 1)) {
            Some(t) if t == Tile::VPipe || t == Tile::SWBend || t == Tile::SEBend => {
                result.push((x, y + 1));
            }
                _ => {},
            }
        },
        Tile::NEBend => { // '┏'
            match field.get((x + 1, y)) {
            Some(t) if t == Tile::HPipe || t == Tile::NWBend || t == Tile::SWBend => {
                result.push((x + 1, y));
            }
                _ => {},
            }
            match field.get((x, y + 1)) {
            Some(t) if t == Tile::VPipe || t == Tile::SWBend || t == Tile::SEBend => {
                result.push((x, y + 1));
            }
                _ => {},
            }
        },
        Tile::SWBend => { // '┛'
            match field.get((x - 1, y)) {
            Some(t) if t == Tile::HPipe || t == Tile::NEBend || t == Tile::SEBend => {
                result.push((x - 1, y));
            }
                _ => {},
            }
            match field.get((x, y - 1)) {
            Some(t) if t == Tile::VPipe || t == Tile::NWBend || t == Tile::NEBend => {
                result.push((x, y - 1));
            }
                _ => {},
            }
        },
        Tile::SEBend => { // '┗'
            match field.get((x + 1, y)) {
            Some(t) if t == Tile::HPipe || t == Tile::NWBend || t == Tile::SWBend => {
                result.push((x + 1, y));
            }
                _ => {},
            }
            match field.get((x, y - 1)) {
            Some(t) if t == Tile::VPipe || t == Tile::NWBend || t == Tile::NEBend => {
                result.push((x, y - 1));
            }
                _ => {},
            }
        },
        _ => {}
    }

    //    println!("x: {:?} y: {:?} CT: {:?}", x, y, result);
    result
}
