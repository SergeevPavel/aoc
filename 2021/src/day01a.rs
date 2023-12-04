mod day01;

use crate::day01::read_input;

fn main() {
    let xs = read_input().unwrap();
    let result = xs.windows(2).filter(|sl| {
        sl[0] < sl[1]
    }).count();
    println!("result: {:?}", result);
}