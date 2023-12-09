use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;


#[derive(Debug, Clone, Copy)]
enum Turn {
    Right,
    Left
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Node([char; 3]);

impl Node {
    fn is_start(&self) -> bool {
        self.0[2] == 'A'
    }
    
    fn is_end(&self) -> bool {
        self.0[2] == 'Z'
    }
}

impl TryFrom<&str> for Node {
    type Error = Vec<char>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tag: [char; 3] = value.chars().collect_vec().try_into()?;
        Ok(Node(tag))
    }
}

type Network = HashMap<Node, (Node, Node)>; 

#[derive(Debug)]
struct Input {
    turns: Vec<Turn>,
    network: Network
}

fn parse(input: &str) -> Input {
    let (turns, network) = input.split_once("\n\n").unwrap();
    let turns = turns.trim().chars().map(|ch| {
        match ch {
            'R' => Turn::Right,
            'L' => Turn::Left,
            _ => panic!("Unexpected symbol: {:?}", ch)
        }
    }).collect();
    let re = Regex::new(r"^([\p{ascii}]+) = \(([\p{ascii}]+), ([\p{ascii}]+)\)$").unwrap();
    let network = network.lines().map(|line| {
        let caps = re.captures(line.trim()).unwrap();
        let node = caps[1].try_into().unwrap();
        let left = caps[2].try_into().unwrap();
        let right = caps[3].try_into().unwrap();
        (node, (left, right))
    }).collect();
    Input {
        turns,
        network
    }
}

fn follow(current: Node, turn: Turn, network: &Network) -> Node {
    let (left, right) = network[&current];
    match turn {
        Turn::Right => right,
        Turn::Left => left,
    }
}

fn ghost_follow(current: &HashSet<Node>, turn: Turn, network: &Network) -> HashSet<Node> {
    current.iter().map(|n| follow(*n, turn, network)).collect()
}

fn part1(input: &Input) {
    let start: Node = "AAA".try_into().unwrap();
    let end: Node = "ZZZ".try_into().unwrap();
    let result = input.turns.iter().cycle().scan(start, |current, turn| {
        *current = follow(*current, *turn, &input.network);
        Some(*current)
    }).take_while(|current| {
        *current != end
    }).take(100_000_000).count() + 1;
    println!("Part1: {:?}", result);
}

fn part2_brute_force(input: &Input) {
    let mut current: HashSet<Node> = input.network.keys()
                                 .filter(|n| n.is_start())
                                 .cloned()
                                 .collect();
    let result = input.turns.iter().cycle().take_while(|turn| {
        current = ghost_follow(&current, **turn, &input.network);
        !current.iter().all(|n| n.is_end())
    }).take(100_000_000).count() + 1;
    println!("Part2: {:?}", result);
}

fn print_cycles(start: Node, input: &Input) {
    let mut prev = 0;
    let result = input.turns.iter().cycle().enumerate().scan(start, |current, (idx, turn)| {
        *current = follow(*current, *turn, &input.network);
        if current.is_end() {
            println!("Step: {:?}", idx - prev);
            prev = idx;
        }
        Some(*current)
    }).take(100_000).count() + 1;
}

fn part2(input: &Input) {
    let start: Node = "AAA".try_into().unwrap();
    input.network.keys().filter(|n| n.is_start()).for_each(|start| {
        println!("Node: {:?}", start);
        print_cycles(*start, input);
    });
}
//13301 12169 20659 22357 20093 18961
fn main() {
    let input = include_str!("../data/day08.txt");
    let input = parse(input);
    part1(&input);
//    part2_brute_force(&input);
    part2(&input);
}