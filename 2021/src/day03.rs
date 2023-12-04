use std::io::BufRead;



pub fn read_input() -> Result<Vec<String>, String> {
    let f = std::fs::File::open("data/input03.txt").map_err(|err| err.to_string())?;
    std::io::BufReader::new(f).lines().map(|line| line.map_err(|err| err.to_string())).collect()
}