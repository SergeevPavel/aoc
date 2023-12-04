use std::collections::HashMap;

type Id = i32;

struct Fs {
    next_id: Id,
    entries: HashMap<Id, FsEntry>,
    current_path: Vec<Id>,
}

impl Fs {
    fn new() -> Self {
        Fs {
            next_id: 1,
            entries: HashMap::from([(0,
                                     FsEntry::Dir {
                name: "/".to_string(),
                                         child_ids: Vec::new()
            })]),
            current_path: vec![0],
        }
    }

    fn current_entry_id(&self) -> Id {
        *self.current_path.last().unwrap()
    }

    fn cd(&mut self, path: &str) {
        match path {
            ".." => {
                self.current_path.pop();
            }
            "/" => {
                self.current_path.truncate(1);
            }
            name => {
                let current_id = self.current_entry_id();
                if let FsEntry::Dir { child_ids, .. } = self.entries.get(&current_id).unwrap() {
                    let child_id = child_ids.iter().find(|child_id| {
                        match self.entries.get(child_id).unwrap() {
                            FsEntry::Dir { name: ch_name, .. } => {
                                ch_name == name
                            },
                            _ => false
                        }
                    }).unwrap_or_else(|| panic!("No such folder: {:?}", name));
                    self.current_path.push(*child_id);
                }
            }
        }
    }

    fn new_entry(&mut self, entry: FsEntry) {
        let id = self.next_id;
        self.next_id += 1;
        self.entries.insert(id, entry);
        let current_id = self.current_entry_id();
        if let FsEntry::Dir { name: _, child_ids } = self.entries.get_mut(&current_id).unwrap() {
            child_ids.push(id);
        }
    }

    fn new_file(&mut self, name: String, size: i32) {
        self.new_entry(FsEntry::File { name, size })
    }

    fn new_dir(&mut self, name: String) {
        self.new_entry(FsEntry::Dir { name, child_ids: Vec::new() });
    }

    fn measure<F>(&self, mut f: F) where F: FnMut(&str, i32) {
        fn go<F>(fs: &Fs, id: Id, f: &mut F) -> i32 where F: FnMut(&str, i32) {
            match fs.entries.get(&id).unwrap() {
                FsEntry::File { name: _, size } => *size,
                FsEntry::Dir { name, child_ids } => {
                    let mut dir_size = 0;
                    for child_id in child_ids {
                        dir_size += go(fs, *child_id, f);
                    }
                    f(name, dir_size);
//                    println!("{:?}: {:?}", name, dir_size);
                    dir_size
                },
            }
        }
        go(self, 0, &mut f);
    }

//    fn measure(&self) {
//        struct State {
//            current_id: Id,
//            size: i32,
//            processed_child: usize
//        }
//
//        let mut stack = vec![State {
//            current_id: 0,
//            size: 0,
//            processed_child: 0
//        }];
//
//        loop {
//            let State { current_id, size, processed_child } = stack.pop().unwrap();
//            if let FsEntry::Dir { name, child_ids } = self.entries.get(&current_id).unwrap() {
//                if child_ids.len() > processed_child {
//                    stack.push(State {
//                        current_id: current_id,
//                        size: 0,
//                        processed_child: processed_child + 1,
//                    });
//                    stack.push(State {
//                        current_id: child_ids[processed_child],
//                        size: 0,
//                        processed_child: 0,
//                    });
//                    continue;
//                }
//
//            }
//        }
//    }
}

enum FsEntry {
    File {
        name: String,
        size: i32
    },
    Dir {
        name: String,
        child_ids: Vec<Id>
    }
}

static INPUT: &str = include_str!("../data/day07.txt");

fn parse<'a>(input: impl Iterator<Item = &'a str>) -> Fs {
    let input = &mut input.peekable();
    let mut fs = Fs::new();
    while let Some(line) = input.next() {
        match line.trim() {
            line if line.starts_with("$ cd") => {
                let path = &line[4..].trim();
                fs.cd(path);
            }
            "$ ls" => {
                while let Some(line) = input.next_if(|line| !line.starts_with("$")) {
                    match line.trim() {
                        line if line.starts_with("dir ") => {
                            let dir = &line[4..].trim();
                            fs.new_dir(dir.to_string());
                        }
                        _ => {
                            let (size, name) = line.split_once(" ").expect("expected size name");
                            fs.new_file(name.trim().to_string(), size.parse().expect("expected number"));
                        }
                    }
                }
            }
            _ => unreachable!()
        }
    }
    fs
}

fn solve1() {
    let fs = parse(INPUT.lines());
    let mut result = 0;
    fs.measure(|_name, size| {
        if size <= 100000 {
            result += size;
        }
    });
    println!("Result1: {}", result);
}

fn solve2() {
    let fs = parse(INPUT.lines());
    let mut total = 0;
    fs.measure(|name, size| {
        if name == "/" {
            total = size;
        }
    });
    let free_space = 70000000 - total;
    let mut result = total;
    fs.measure(|_name, size| {
        if free_space + size >= 30000000 && size < result {
            result = size;
        }
    });
    println!("Result2: {}", result);
}

fn main() {
    solve1();
    solve2();
}