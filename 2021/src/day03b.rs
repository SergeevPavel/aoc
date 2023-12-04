use day03::read_input;

mod day03;

fn raiting(mut lines: Vec<String>, most_common: bool) -> u32 {
    let bits_count = lines.first().expect("Empty input").len();
    assert!(lines.iter().any(|line| line.len() == bits_count));
    for i in 0..bits_count {
        let ones_count = lines.iter().map(|line| line.chars().nth(i).unwrap())
                                     .filter(|ch| *ch == '1')
                                     .count();
        if 2 * ones_count >= lines.len() {
            lines = lines.drain(..).filter(|line| line.chars().nth(i).unwrap() == if most_common { '1' } else { '0' }).collect();
        } else {
            lines = lines.drain(..).filter(|line| line.chars().nth(i).unwrap() == if most_common { '0' } else { '1' }).collect();
        }
        if lines.len() == 1 {
            break;
        }
    }
    assert!(lines.len() == 1);
    return u32::from_str_radix(lines.first().unwrap(), 2).unwrap();
}

fn main() {
    let lines = read_input().unwrap();
    let oxygen = raiting(lines.clone(), true);
    let co2 = raiting(lines, false);
    println!("result: {:?}", oxygen * co2);
}