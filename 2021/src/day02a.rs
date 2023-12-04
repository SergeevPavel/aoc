use crate::day02::*;

mod day02;


fn main() {
    let commands = read_input().unwrap();
    let mut position: i32 = 0;
    let mut depth: i32 = 0;
    for cmd in commands {
        match cmd {
            Command::Up { value } => depth -= value as i32,
            Command::Down { value } => depth += value as i32,
            Command::Forward { value } => position += value as i32,
        }
    }
    println!("result: {}", position * depth)
}