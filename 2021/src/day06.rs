

fn main() {
    let input = std::fs::read_to_string("data/input06.txt").expect("file not found");
    let max_timer: usize = 9;
    let mut fishes: Vec<usize> = vec![0; max_timer];
    for fish_t in input.split(",").map(|f| f.parse::<usize>().unwrap()) {
        fishes[fish_t] += 1;
    }
    let days_count: usize = 256;
    for d in 0..days_count {
        let mut next_fishes: Vec<usize> = vec![0; max_timer];
        for t in 1..max_timer {
            next_fishes[t - 1] = fishes[t];
        }
        next_fishes[max_timer - 1] = fishes[0];
        next_fishes[6] += fishes[0];
        fishes = next_fishes;
//        println!("day: {:?} result: {:?}", d, fishes.iter().sum::<usize>());
    }
    println!("result: {:?}", fishes.iter().sum::<usize>());
}