use std::{fs::File, io::{BufReader, BufRead}, collections::{HashMap, HashSet}};

#[derive(Debug)]
struct Task {
    start: String,
    end: String,
    edges: Vec<(String, String)>,
    neighbours: HashMap<String, Vec<String>>,
    max_cycles_count: u32,
}

impl Task {
    fn is_big_cave(&self, cave: String) -> bool {
        cave.chars().all(|c| c.is_uppercase())
    }
}

fn read_input() -> Task {
    let f = File::open("data/input12.txt").unwrap();
    let mut edges = Vec::new();
    let mut neighbours = HashMap::<String, Vec<String>>::new();
    for line in BufReader::new(f).lines() {
        let line = line.unwrap();
        let (from, to) = line.split_once("-").unwrap();
        edges.push((from.to_string(), to.to_string()));
        neighbours.entry(from.to_string()).or_insert(Vec::new()).push(to.to_string());
        neighbours.entry(to.to_string()).or_insert(Vec::new()).push(from.to_string());
    }
    Task {
        start: "start".to_string(),
        end: "end".to_string(),
        edges,
        neighbours,
        max_cycles_count: 1
    }
}

fn paths1(task: &Task) -> u32 {
    fn go(task: &Task,
          visited: &mut HashSet<String>,
          current: String,
          result: &mut u32) {
        visited.insert(current.clone());
        for neighbour in &task.neighbours[&current] {
            if !visited.contains(neighbour) || task.is_big_cave(neighbour.to_string()) {
                if *neighbour == task.end {
                    *result += 1;
                } else {
                    go(task, visited, neighbour.to_string(), result);
                }
            }
        }
        visited.remove(&current);
    }
    let mut visited = HashSet::new();
    let mut result: u32 = 0;
    go(&task, &mut visited, task.start.clone(), &mut result);
    return result;
}

fn paths2(task: &Task) -> u32 {
    fn go(task: &Task,
          visited: &mut HashMap<String, u32>,
          current: String,
          cycles_count: u32,
          result: &mut u32) {
        *visited.entry(current.clone()).or_insert(0) += 1;
        for neighbour in &task.neighbours[&current] {
            if *neighbour == task.start {
                continue;
            } else if *neighbour == task.end {
                *result += 1;
            } else if task.is_big_cave(neighbour.to_string()) {
                go(task, visited, neighbour.to_string(), cycles_count, result);
            } else if *visited.entry(neighbour.to_string()).or_insert(0) == 0 {
                go(task, visited, neighbour.to_string(), cycles_count, result);
            } else if cycles_count < task.max_cycles_count {
                go(task, visited, neighbour.to_string(), cycles_count + 1, result);
            }
        }
        *visited.get_mut(&current).unwrap() -= 1;
    }
    let mut visited = HashMap::new();
    let mut result: u32 = 0;
    go(&task, &mut visited, task.start.clone(), 0, &mut result);
    return result;
}

fn main() {
    let task = read_input();
    println!("result1: {:?}", paths1(&task));
    println!("result2: {:?}", paths2(&task));
}