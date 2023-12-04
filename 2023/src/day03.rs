use std::collections::HashMap;


fn is_symbol(ch: char) -> bool {
    !ch.is_digit(10) && !(ch == '.') 
}

fn symbols_nearby(schem: &Vec<Vec<char>>, i: usize, j: usize) -> HashMap<(usize, usize), char> {
    let mut result = HashMap::new();
    for di in -1i32..=1 {
        for dj in -1i32..=1 {
            if di == 0 && dj == 0 { continue; }
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni < 0 || ni >= schem.len() as i32 { continue; }
            if nj < 0 || nj >= schem[i].len() as i32 { continue; }
            let ch = schem[ni as usize][nj as usize];
            if is_symbol(ch) {
                result.insert((ni as usize, nj as usize), ch);
            }
        }
    }
    return result;
}

fn numbers(schem: &Vec<Vec<char>>) -> Vec<(u32, HashMap<(usize, usize), char>)> {
    let mut result: Vec<(u32, HashMap<(usize, usize), char>)> = Vec::new();
    schem.iter().enumerate().for_each(|(i, line)| {
        let mut number = 0;
        let mut adjacent_symbols = HashMap::new();
        line.iter().enumerate().for_each(|(j, ch)| {
            match ch {
                digit if digit.is_digit(10) => {
                    adjacent_symbols.extend(symbols_nearby(&schem, i, j));
                    number = number * 10 + digit.to_digit(10).unwrap();
                }
                _ => {
                    if !adjacent_symbols.is_empty() {
                        result.push((number, adjacent_symbols.clone()));
                    }
                    adjacent_symbols.clear();
                    number = 0;
                }
            }
        });
        if !adjacent_symbols.is_empty() {
            result.push((number, adjacent_symbols.clone()));
        }
    });
    return result;
}

fn part1(input: &str) {
    let schem: Vec<_> = input.lines().map(|line| line.trim().chars().collect::<Vec<_>>()).collect();
    let result: u32 = numbers(&schem).iter().map(|(num, _)| num).sum();
    println!("Part1: {:?}", result);
}

fn part2(input: &str) {
    let schem: Vec<_> = input.lines().map(|line| line.trim().chars().collect::<Vec<_>>()).collect();
    let numbers = numbers(&schem);
    let mut result = 0;
    schem.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, ch)| {
            if *ch == '*' {
                let adjacent_numbers: Vec<_> = numbers.iter().filter(|(num, syms)| {
                    syms.contains_key(&(i, j))
                }).map(|(num, _)| *num).collect();
                if adjacent_numbers.len() == 2 {
                    result += adjacent_numbers.iter().product::<u32>();
                }
            }
        });
    });
    println!("Part2: {:?}", result);
}

fn main() {
    let input = include_str!("../data/day03.txt");
    part1(input);
    part2(input);
}