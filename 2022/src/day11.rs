#![feature(binary_heap_into_iter_sorted)]

use std::collections::{HashMap, BinaryHeap};

type MonkeyId = usize;
type WorryLevel = i64;

struct Monkey {
    id: MonkeyId,
    items: Vec<WorryLevel>,
    operation: fn(WorryLevel) -> WorryLevel,
    test: fn(WorryLevel) -> MonkeyId,
}

fn monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            id: 0,
            items: vec![57, 58],
            operation: |old| old * 19,
            test: |old| if old % 7 == 0 { 2 } else { 3 },
        },
        Monkey {
            id: 1,
            items: vec![66, 52, 59, 79, 94, 73],
            operation: |old| old + 1,
            test: |old| if old % 19 == 0 { 4 } else { 6 },
        },
        Monkey {
            id: 2,
            items: vec![80],
            operation: |old| old + 6,
            test: |old| if old % 5 == 0 { 7 } else { 5 },
        },
        Monkey {
            id: 3,
            items: vec![82, 81, 68, 66, 71, 83, 75, 97],
            operation: |old| old + 5,
            test: |old| if old % 11 == 0 { 5 } else { 2 },
        },
        Monkey {
            id: 4,
            items: vec![55, 52, 67, 70, 69, 94, 90],
            operation: |old| old * old,
            test: |old| if old % 17 == 0 { 0 } else { 3 },
        },
        Monkey {
            id: 5,
            items: vec![69, 85, 89, 91],
            operation: |old| old + 7,
            test: |old| if old % 13 == 0 { 1 } else { 7 },
        },
        Monkey {
            id: 6,
            items: vec![75, 53, 73, 52, 75],
            operation: |old| old * 7,
            test: |old| if old % 2 == 0 { 0 } else { 4 },
        },
        Monkey {
            id: 7,
            items: vec![94, 60, 79],
            operation: |old| old + 2,
            test: |old| if old % 3 == 0 { 1 } else { 6 },
        },
    ]
}

fn monkey_business(rounds: i32, dont_worry: bool) -> i64 {
    const M: i64 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
    let mut monkeys = monkeys();
    let mut inspections = HashMap::<MonkeyId, usize>::new();
    for _round in 0..rounds {
        for m_id in 0..monkeys.len() {
            let items = std::mem::replace(&mut monkeys[m_id].items, Vec::new());
            *inspections.entry(m_id).or_insert_with(|| 0) += items.len();
            for item in items.into_iter() {
                let div = if dont_worry {
                    3
                } else {
                    1
                };
                let item = ((monkeys[m_id].operation)(item) / div) % M;
                let next_m_id = (monkeys[m_id].test)(item);
                monkeys[next_m_id].items.push(item);
            }
        }
    }
    let inspections: BinaryHeap<usize> = inspections.into_values().collect();
    return inspections.into_iter_sorted().take(2).reduce(|a, b| a * b).unwrap() as i64;
}

fn main() {
    println!("Result1: {}", monkey_business(20, true));
    println!("Result2: {}", monkey_business(10000, false));
}
