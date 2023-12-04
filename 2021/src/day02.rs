use std::{fs::File, io::{BufReader, BufRead}};
use itertools::Itertools;

#[derive(Debug)]
pub enum Command {
    Up {
        value: u32
    },
    Down {
        value: u32
    },
    Forward {
        value: u32
    }
}

pub fn read_input() -> Result<Vec<Command>, String> {
    let f = File::open("data/input02.txt").map_err(|err| err.to_string())?;

    fn parse_line(line: String) -> Result<Command, String> {
        let (t, v) = line.split_whitespace().next_tuple().ok_or("failed to parse")?;
        return match t {
            "forward" => Ok(Command::Forward {
                value: v.parse::<u32>().map_err(|err| err.to_string())?
            }),
            "up" => Ok(Command::Up {
                value: v.parse::<u32>().map_err(|err| err.to_string())?
            }),
            "down" => Ok(Command::Down {
                value: v.parse::<u32>().map_err(|err| err.to_string())?
            }),
            _ => Err("unknown command".to_string())
        }
    }

    let xs: Result<Vec<_>, String> = BufReader::new(f).lines().map(|line| match line {
        Ok(s) => {
            parse_line(s)
        }
        Err(err) => Err(err.to_string()),
    } ).collect();
    return xs;
}