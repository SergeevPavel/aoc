use std::{collections::{HashMap, HashSet}};

#[derive(Debug)]
struct Board {
    positions: HashMap<u32, (usize, usize)>,
    unmarket_numbers: HashSet<u32>,
    marked_in_rows: HashMap<usize, usize>,
    marked_in_cols: HashMap<usize, usize>,
    in_game: bool,
}

impl Board {
    const NUMBERS_IN_ROW: usize = 5;
    const NUMBERS_IN_COL: usize = 5;

    fn from_string(s: &str) -> Self {
        let mut positions: HashMap<u32, (usize, usize)> = HashMap::new();
        let mut unmarket_numbers: HashSet<u32> = HashSet::new();
        for (i, line) in s.split("\n").enumerate() {
            for (j, num) in line.split_whitespace().enumerate() {
                let num = num.parse().unwrap();
                unmarket_numbers.insert(num);
                if let Some(_) = positions.insert(num, (i, j)) {
                    panic!("duplicate")
                }
            }
        }
        return Board {
            marked_in_rows: Default::default(),
            marked_in_cols: Default::default(),
            positions,
            unmarket_numbers,
            in_game: true
        }
    }



    fn check(&mut self, number: u32) -> bool {
        if let Some((i, j)) = self.positions.get(&number) {
            let marked_in_row = self.marked_in_rows.entry(*i).or_insert(0);
            *marked_in_row += 1;
            let marked_in_col = self.marked_in_cols.entry(*j).or_insert(0);
            *marked_in_col += 1;
            self.unmarket_numbers.remove(&number);
            if *marked_in_row == Board::NUMBERS_IN_ROW ||
               *marked_in_col == Board::NUMBERS_IN_COL {
                self.in_game = false;
                return true
            }
        }
        return false
    }

    fn remaining_sum(&self) -> u32 {
        self.unmarket_numbers.iter().cloned().reduce(|x, y| x + y).unwrap_or(0)
    }
}

#[derive(Debug)]
struct InputData {
    boards: Vec<Board>,
    numbers: Vec<u32>,
}

fn read_input() -> Result<InputData, String> {
    let input = std::fs::read_to_string("data/input04.txt").map_err(|err| err.to_string())?;
    let mut input_blocks = input.split("\n\n");
    let numbers: Result<Vec<_>, String> = input_blocks.next().expect("empty input")
                                                      .split(",").map(|n| n.parse::<u32>().map_err(|err| err.to_string()))
                                                      .collect();
    let numbers = numbers?;
    let boards: Vec<_> = input_blocks.map(|board_str| Board::from_string(board_str)).collect();
    return Ok(InputData {
        numbers,
        boards
    })
}

fn main() {
    let InputData { mut boards, numbers } = read_input().unwrap();
    for number in numbers {
        for board in &mut boards {
            if board.in_game && board.check(number) {
                println!("result: {:?}", board.remaining_sum() * number);
            }
        }
    }
}