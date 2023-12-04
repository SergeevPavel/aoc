use std::{iter::repeat_with, collections::HashMap};

#[derive(Clone)]
struct Player {
    position: i32,
    score: i32,
}

impl Player {
    fn new(position: i32) -> Self {
        Player { position, score: 0 }
    }

    fn update(&mut self, shift: i32) -> i32 {
        self.position += shift;
        self.position = (self.position - 1) % 10 + 1;
        self.score += self.position;
        self.score
    }

    fn play(&mut self, dice: &mut Dice) -> bool {
        let shift: i32 = repeat_with(|| dice.roll()).take(3).sum();
        self.update(shift) > 1000
    }
}

struct Dice {
    state: i32,
    rolls: i32,
}

impl Dice {
    fn new() -> Self {
        Dice { state: 99, rolls: 0 }
    }

    fn roll(&mut self) -> i32 {
        self.rolls += 1;
        self.state += 1;
        self.state %= 100;
        self.state + 1
    }
}

fn solve1() {
    let mut dice = Dice::new();
    let mut p1 = Player::new(8);
    let mut p2 = Player::new(10);
    loop {
        if p1.play(&mut dice) {
            println!("P1 wins! {}", dice.rolls * p2.score);
            break;
        }
        if p2.play(&mut dice) {
            println!("P2 wins! {}", dice.rolls * p1.score);
            break;
        }
    }
}

#[derive(Debug)]
struct RollResult {
    result: i32,
    universes: i64,
}

struct DiracDice {
    roll_results3x: Vec<RollResult>
}

impl DiracDice {
    fn new() -> DiracDice {
        let mut counts = HashMap::<i32, i64>::new();
        for r1 in DiracDice::roll() {
            for r2 in DiracDice::roll() {
                for r3 in DiracDice::roll() {
                    let total = r1.result + r2.result + r3.result;
                    let universes = r1.universes * r2.universes * r3.universes;
                    *counts.entry(total).or_insert(0) += universes;
                }
            }
        }
        let roll_results3x = counts.iter()
                                  .map(|(result, universes)| RollResult { result: *result, universes: *universes })
                                  .collect::<Vec<_>>();
        DiracDice { roll_results3x }

    }

    fn roll() -> Vec<RollResult> {
        (1..=3).map(|result| RollResult { result, universes: 1 }).collect()
    }

    fn roll3x(&self) -> &Vec<RollResult> {
        &self.roll_results3x
    }
}

fn play_quantum_game(player1_turn: bool, p1: &mut Player, p2: &mut Player, dice: &DiracDice) -> (i64, i64) {
    const WIN_SCORE: i32 = 21;
    let mut p1_win = 0i64;
    let mut p2_win = 0i64;
    for RollResult { result, universes } in dice.roll3x() {
        if player1_turn {
            let mut new_p1 = p1.clone();
            if new_p1.update(*result) >= WIN_SCORE {
                p1_win += *universes;
            } else {
                let (p1_w, p2_w) = play_quantum_game(false, &mut new_p1, p2, dice);
                p1_win += p1_w * universes;
                p2_win += p2_w * universes;
            }
        } else {
            let mut new_p2 = p2.clone();
            if new_p2.update(*result) >= WIN_SCORE {
                p2_win += *universes;
            } else {
                let (p1_w, p2_w) = play_quantum_game(true, p1, &mut new_p2, dice);
                p1_win += p1_w * universes;
                p2_win += p2_w * universes;
            }
        }
    }
    return (p1_win, p2_win);
}

fn solve2() {
    let mut p1 = Player::new(8);
    let mut p2 = Player::new(10);
    let dice = DiracDice::new();
    let (p1_win, p2_win) = play_quantum_game(true, &mut p1, &mut p2, &dice);
    println!("p1_win: {}", p1_win);
    println!("p2_win: {}", p2_win);
}

fn main() {
    solve1();
    solve2();
}