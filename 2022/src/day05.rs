use std::collections::HashMap;

use itertools::Itertools;

static INPUT: &str = include_str!("../data/day05.txt");

#[derive(Debug)]
struct State {
    crates: HashMap<usize, Vec<char>>
}

#[derive(Debug)]
struct Operation {
    count: usize,
    from: usize,
    to: usize
}

fn parse_input(input: &str) -> (State, Vec<Operation>) {
    let state_input = input.lines()
    .rev()
    .skip_while(|line| !line.trim().is_empty())
    .skip(2);
    let crates = state_input.fold(HashMap::<usize, Vec<char>>::new(), |mut state, line| {
        line.chars().chunks(4).into_iter().enumerate().for_each(|(id, mut ch)| {
            state.entry(id + 1).or_insert(Vec::new()).extend(ch.nth(1).filter(|c| c.is_ascii_alphabetic()))
        });
        state
    });
    let state = State {
        crates
    };
    let operations = input.lines().skip_while(|line| !line.trim().is_empty()).skip(1).map(|line| {
        let mut parts = line.split_whitespace();
        assert_eq!(parts.next().unwrap(), "move");
        let count = parts.next().unwrap().parse().unwrap();
        assert_eq!(parts.next().unwrap(), "from");
        let from = parts.next().unwrap().parse().unwrap();
        assert_eq!(parts.next().unwrap(), "to");
        let to = parts.next().unwrap().parse().unwrap();
        Operation {
            count,
            from,
            to
        }
    }).collect();
    (state, operations)
}

impl State {
    fn perform_crate_mover_9000(&mut self, op: &Operation) {
        for _ in 0..op.count {
            let crates = &mut self.crates;
            let cr = crates.get_mut(&op.from).unwrap().pop();
            crates.get_mut(&op.to).unwrap().extend(cr);
        }
    }

    fn perform_crate_mover_9001(&mut self, op: &Operation) {
        let crates = &mut self.crates;
        let mut stack_from = crates.remove(&op.from).unwrap();
        let cr = stack_from.drain((stack_from.len() - op.count)..);
        crates.get_mut(&op.to).unwrap().extend(cr);
        crates.insert(op.from, stack_from);
    }

    fn print_tops(&self) {
        self.crates.iter().sorted_by_key(|(id, _cr)| *id).for_each(|(_id, cr)| {
            print!("{}", cr.last().map(|c| *c).unwrap_or(' '));
        });
    }
}

fn solve1() {
    let (mut state, operations) = parse_input(INPUT);
    for op in &operations {
        state.perform_crate_mover_9000(op);
    }
    state.print_tops();
}

fn solve2() {
    let (mut state, operations) = parse_input(INPUT);
    for op in &operations {
        state.perform_crate_mover_9001(op);
    }
    state.print_tops();
}

fn main() {
    solve1();
    println!();
    solve2();
}