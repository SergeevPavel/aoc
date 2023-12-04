use itertools::*;

const INPUT: &str = include_str!("../data/day06.txt");

fn find_maker(input: &str, unique: usize) -> Option<(usize, &str)> {
    input.as_bytes().windows(unique).enumerate().find(|(_offset, symbols)| {
        symbols.iter().all_unique()
    }).map(|(offset, symbols)| {
        (offset + unique, std::str::from_utf8(symbols).unwrap())
    })
}

fn main() {
    println!("{:?}", find_maker(INPUT, 4));
    println!("{:?}", find_maker(INPUT, 14));
}