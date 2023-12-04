use std::{fmt::Display, collections::{HashMap, BinaryHeap}};

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Field {
    data: Vec<u32>,
    width: i32,
    height: i32,
}

impl Field {
    fn new(width: i32, height: i32) -> Self {
        Field {
            width,
            height,
            data: vec![0; (width * height) as usize]
        }
    }

    fn offset(&self, p: Point) -> usize {
        (p.x + p.y * self.width) as usize
    }

    fn set(&mut self, p: Point, v: u32) {
        let offset = self.offset(p);
        self.data[offset] = v;
    }

    fn get(&self, p: Point) -> Option<u32> {
        let result = if 0 <= p.x && p.x < self.width && 0 <= p.y && p.y < self.height {
            Some(self.data[self.offset(p)])
        } else {
            None
        };
        result
    }

    fn neighbours(&self, p: Point) -> impl Iterator<Item = Point> {
        let width = self.width;
        let height = self.height;
        [(0, -1), (-1, 0), (1, 0), (0, 1)].iter().filter_map(move |(dx, dy)| {
            let x = p.x + dx;
            let y = p.y + dy;
            if 0 <= x && x < width && 0 <= y && y < height {
                Some(Point { x, y })
            } else {
                None
            }
        })
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_str(&self.get(Point { x, y }).unwrap().to_string())?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn read_input() -> Field {
    let input = std::fs::read_to_string("data/input15.txt").unwrap();
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    let data = input.chars().filter_map(|ch| ch.to_digit(10)).collect();
    Field {
        width,
        height,
        data
    }
}

fn dijkstra(field: &Field, start: Point, end: Point) -> u32 {
    let mut dst = HashMap::<Point, u32>::new();

    #[derive(Debug, Eq, PartialEq)]
    struct State {
        position: Point,
        distance: u32,
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.distance.cmp(&other.distance).reverse().then_with(|| self.position.cmp(&other.position))
        }
    }

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State { position: start, distance: 0 });

    while let Some(st) = heap.pop() {
        if dst.contains_key(&st.position) {
            continue;
        }
        dst.insert(st.position, st.distance);

        for n in field.neighbours(st.position) {
            if !dst.contains_key(&n) {
                heap.push(State {
                    position: n,
                    distance: st.distance + field.get(n).unwrap()
                })
            }
        }
    }

    *dst.get(&end).unwrap()
}

fn tile_field(field: &Field, x_tiles: i32, y_tiles: i32) -> Field {
    let mut new_filed = Field::new(field.width * x_tiles, field.height * y_tiles);
    for i in 0..x_tiles {
        for j in 0..y_tiles {
            for x in 0..field.width {
                for y in 0..field.height {
                    let original = field.get(Point { x, y }).unwrap() + i as u32 + j as u32;
                    new_filed.set(Point { x: i * field.width + x,
                                          y: j * field.height + y},
                                  (original - 1) % 9 + 1)
                }
            }
        }
    }
    return new_filed;
}

fn main() {
    let field = read_input();
    println!("task1: {}", dijkstra(&field, Point { x: 0, y: 0 }, Point { x: field.width - 1, y: field.height - 1}));
    let tiled_field = tile_field(&field, 5, 5);
    println!("task2: {}", dijkstra(&tiled_field, Point { x: 0, y: 0 }, Point { x: tiled_field.width - 1, y: tiled_field.height - 1}))
}