
static INPUT: &str = include_str!("../data/day10.txt");

enum Operation {
    Addx(i32),
    Noop
}

impl Operation {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "noop" => Some(Operation::Noop),
            _ if s.starts_with("addx ") => {
                let (_, v) = s.split_once(" ")?;
                Some(Operation::Addx(v.parse().ok()?))
            }
            _ => None
        }
    }
}

struct Machine {
    x: i32,
    cycle: i32,
    signal: i32,
}

impl Machine {
    fn new() -> Self {
        Machine {
            x: 1,
            cycle: 0,
            signal: 0
        }
    }

    fn check_signal(&mut self) {
        const CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];
        if CYCLES.contains(&self.cycle) {
            self.signal += self.x * self.cycle;
        }
    }

    fn draw_pixel(&self) {
        let col = (self.cycle - 1) % 40;
        if self.x - 1 <= col && col <= self.x + 1 {
            print!("#");
        } else {
            print!(".");
        }
        if col == 39 {
            println!();
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;
        self.check_signal();
        self.draw_pixel();
    }

    fn apply(&mut self, op: &Operation) {
        match op {
            Operation::Addx(v) => {
                self.tick();
                self.tick();
                self.x += v;
            },
            Operation::Noop => {
                self.tick();
            },
        }
    }
}

fn main() {
    let mut m = Machine::new();
    INPUT.lines().flat_map(|s| Operation::from_str(s.trim())).for_each(|op| {
        m.apply(&op);
    });
    println!("Result: {}", m.signal);
}