use std::collections::HashSet;

use itertools::Itertools;

type Galaxy = (i64, i64);

fn distance((x0, y0): Galaxy, (x1, y1): Galaxy) -> i64 {
    (x0.abs_diff(x1) + y0.abs_diff(y1)).try_into().unwrap()
}

fn solve_for_scale(input: &str, scale: i64) -> i64 {
    let mut input: Vec<_> = input.lines().enumerate().flat_map(|(y, line)| {
        line.trim().chars().enumerate().filter_map(move |(x, c)| {
            match c {
                '#' => Some((x as i64, y as i64)),
                _ => None
            }
        })
    }).collect();

    input.sort_by_key(|(x, _y)| *x);
    let mut expansion_by_x = 0;
    let mut x_expanded = vec![*input.first().unwrap()];
    input.windows(2).for_each(|gs| {
        //        println!("x expansion: {:?}", expansion_by_x);
        let (x0, _y0) = gs[0];
        let (x1, y1) = gs[1];
        if x1 - x0 > 1 {
            expansion_by_x += ((x1 - x0) - 1) * (scale - 1);
        }
        x_expanded.push((x1 + expansion_by_x, y1));
    });

    x_expanded.sort_by_key(|(_x, y)| *y);
    let mut expansion_by_y = 0;
    let mut xy_expanded = vec![*x_expanded.first().unwrap()];
    x_expanded.windows(2).for_each(|gs| {
        //        println!("y expansion: {:?}", expansion_by_y);
        let (_x0, y0) = gs[0];
        let (x1, y1) = gs[1];
        if y1 - y0 > 1 {
            expansion_by_y += ((y1 - y0) - 1) * (scale - 1);
        }
        xy_expanded.push((x1, y1 + expansion_by_y));
    });
    xy_expanded.iter().cartesian_product(xy_expanded.iter()).map(|(g1, g2)| distance(*g1, *g2)).sum::<i64>() / 2i64
}

fn part1(input: &str) {
    let result = solve_for_scale(input, 2);
    println!("Part1: {:?}", result);
}

fn part2(input: &str) {
    let result = solve_for_scale(input, 1000000);
    println!("Part2: {:?}", result);
}

fn main() {
    let input = include_str!("../data/day11.txt");
    part1(input);
    part2(input);
}