use std::{fs::File, io::{BufReader, BufRead}};
use anyhow::{anyhow, Result, Error};

#[derive(Debug, Clone, Copy)]
enum Figure {
    Rock,
    Paper,
    Scissors
}

impl Figure {
    fn score(&self) -> i32 {
        match self {
            Figure::Rock     => 1,
            Figure::Paper    => 2,
            Figure::Scissors => 3,
        }
    }
}

fn read_input1() -> Result<Vec<(Figure, Figure)>> {
    let f = File::open("data/day02.txt")?;
    let b = BufReader::new(f);
    b.lines().map(|line| {
        let line = line?;
        let (f1, f2) = line.trim().split_once(" ").ok_or(Error::msg("Malformed input"))?;
        let f1 = match f1 {
            "A" => Ok(Figure::Rock),
            "B" => Ok(Figure::Paper),
            "C" => Ok(Figure::Scissors),
            _   => Err(anyhow!("Expected A B or C instead of {}", f1))
        }?;
        let f2 = match f2 {
            "X" => Ok(Figure::Rock),
            "Y" => Ok(Figure::Paper),
            "Z" => Ok(Figure::Scissors),
            _   => Err(anyhow!("Expected X Y or Z instead of {}", f2))
        }?;
        Ok((f1, f2))
    }).collect()
}

fn read_input2() -> Result<Vec<(Figure, Outcome)>> {
    let f = File::open("data/day02.txt")?;
    let b = BufReader::new(f);
    b.lines().map(|line| {
        let line = line?;
        let (f1, f2) = line.trim().split_once(" ").ok_or(Error::msg("Malformed input"))?;
        let f1 = match f1 {
            "A" => Ok(Figure::Rock),
            "B" => Ok(Figure::Paper),
            "C" => Ok(Figure::Scissors),
            _   => Err(anyhow!("Expected A B or C instead of {}", f1))
        }?;
        let f2 = match f2 {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _   => Err(anyhow!("Expected X Y or Z instead of {}", f2))
        }?;
        Ok((f1, f2))
    }).collect()
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Win  => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

fn play(me: Figure, other: Figure) -> Outcome {
    use Outcome::*;
    match (me, other) {
        (Figure::Rock, Figure::Rock)         => Draw,
        (Figure::Rock, Figure::Paper)        => Loss,
        (Figure::Rock, Figure::Scissors)     => Win,

        (Figure::Paper, Figure::Rock)        => Win,
        (Figure::Paper, Figure::Paper)       => Draw,
        (Figure::Paper, Figure::Scissors)    => Loss,

        (Figure::Scissors, Figure::Rock)     => Loss,
        (Figure::Scissors, Figure::Paper)    => Win,
        (Figure::Scissors, Figure::Scissors) => Draw,
    }
}

fn figure_out(other: Figure, outcome: Outcome) -> Figure {
    use Figure::*;
    use Outcome::*;
    match (other, outcome) {
        (Rock,     Win)  => Paper,
        (Rock,     Draw) => Rock,
        (Rock,     Loss) => Scissors,

        (Paper,    Win)  => Scissors,
        (Paper,    Draw) => Paper,
        (Paper,    Loss) => Rock,

        (Scissors, Win)  => Rock,
        (Scissors, Draw) => Scissors,
        (Scissors, Loss) => Paper,
    }
}

fn solve1() {
    let result: i32 = read_input1().unwrap().iter().map(|(other, me)| {
        let outcome = play(*me, *other);
        outcome.score() + me.score()
    }).sum();
    println!("Result1: {:?}", result);
}

fn solve2() {
    let result: i32 = read_input2().unwrap().iter().map(|(other, outcome)| {
        let me = figure_out(*other, *outcome);
        outcome.score() + me.score()
    }).sum();
    println!("Result2: {:?}", result);
}

fn main() {
    solve1();
    solve2();
}