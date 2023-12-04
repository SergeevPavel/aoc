use std::{fs::File, io::{BufReader, BufRead}};

pub fn read_input() -> Result<Vec<u32>, String> {
    let f = File::open("data/input01.txt").map_err(|err| err.to_string())?;
    let xs: Result<Vec<_>, String> = BufReader::new(f).lines().map(|line| match line {
        Ok(s) => {
            s.parse::<u32>().map_err(|err| err.to_string())
        }
        Err(err) => Err(err.to_string()),
    } ).collect();
    return xs;
}