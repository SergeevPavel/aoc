use std::ops::RangeInclusive;

use itertools::Itertools;


static INPUT: &str = include_str!("../data/input22.txt");

#[derive(Debug)]
struct Step {
    on: bool,
    xr: RangeInclusive<i32>,
    yr: RangeInclusive<i32>,
    zr: RangeInclusive<i32>,
}

fn input() -> Vec<Step> {
    INPUT.lines().map(|line| {
        let line = line.trim();
        let (on, q) = line.split_once(" ").expect("expected space");
        let on = match on {
            "on" => true,
            "off" => false,
            _ => panic!("{} not expected here", on)
        };
        let (xr, yr, zr) = q.split(",").map(|r| {
            let (from, to) = r[2..].split_once("..").expect("incorrect range");
            from.parse::<i32>().unwrap()..=to.parse::<i32>().unwrap()
        }).next_tuple().unwrap();

        Step { on, xr, yr, zr }
    }).collect()
}

fn solve1() {
    let commands = input();

    let check = |x: i32, y: i32, z: i32| -> bool {
        let mut state = false;
        for c in &commands {
            if c.xr.contains(&x) &&
               c.yr.contains(&y) &&
               c.zr.contains(&z) {
                state = c.on;
            }
        }
        state
    };

    let mut result = 0;
    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                if check(x, y, z) {
                    result += 1;
                }
            }
        }
    }
    println!("Result1: {:?}", result);
}

fn main() {
    solve1()
}