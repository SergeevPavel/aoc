use std::{collections::HashSet};

type Point = [i32; 2];

#[derive(Debug)]
struct Field {
    text: String,
    size: Point,
    rocks: HashSet<Point>
}

#[derive(Debug)]
struct Symmetry {
    dim: usize,
    offset: i32,
}

fn mirror(p: &Point, dim: usize, offset: i32) -> Point {
    let x = p[dim];
    let new_x = 2 * offset + 1 - x;
    let mut new_p = p.clone();
    new_p[dim] = new_x;
    new_p
}

fn in_field(field: &Field, p: &Point) -> bool {
    (0..field.size.len()).all(|dim| {
        p[dim] > 0 && p[dim] <= field.size[dim]
    })
}

fn find_symmetry(field: &Field, fix_smudge: bool) -> Option<Symmetry> {
    for dim in 0..field.size.len() {
        for offset in 1..field.size[dim] {
            let smudges = field.rocks.iter().filter(|rock| {
                let mirrored_rock = mirror(rock, dim, offset);
                in_field(field, &mirrored_rock) && !field.rocks.contains(&mirrored_rock)
            }).count();
            let is_symmetry = if fix_smudge {
                smudges == 1
            } else {
                smudges == 0
            };
            if is_symmetry {
                return Some(Symmetry {
                    dim,
                    offset
                })
            }
        }
    }
    None
}

fn parse(input: &str) -> Vec<Field> {
    input.split("\n\n").map(|field| {
        let mut width = 0;
        let height = field.lines().count();
        let rocks = field.lines().enumerate().flat_map(|(i, line)| {
            let line = line.trim();
            width = line.chars().count();
            line.chars().enumerate().filter_map(move |(j, ch)| {
                if ch == '#' {
                    Some([i as i32 + 1, j as i32 + 1])
                } else {
                    None
                }
            })
        }).collect();
        Field {
            size: [height.try_into().unwrap(), width.try_into().unwrap()],
            rocks,
            text: field.to_owned(),
        }
    }).collect()
}

fn part1(input: &str) {
    let input = parse(input);
    let result: i32 = input.iter().map(|field| {
        if let Some(sym) = find_symmetry(field, false) {
            match sym.dim {
                0 => sym.offset * 100,
                1 => sym.offset,
                _ => unreachable!()
            }
        } else {
            println!("{:?}", field);
            0
        }
    }).sum();
    println!("Part1: {:?}", result);
}

fn part2(input: &str) {
    let input = parse(input);
    let result: i32 = input.iter().map(|field| {
        if let Some(sym) = find_symmetry(field, true) {
            match sym.dim {
                0 => sym.offset * 100,
                1 => sym.offset,
                _ => unreachable!()
            }
        } else {
            println!("{:?}", field);
            0
        }
    }).sum();
    println!("Part2: {:?}", result);
}

fn main() {
    let input =  include_str!("../data/day13.txt");
    part1(input);
    part2(input);
}