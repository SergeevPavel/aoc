use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet}, ops::{Index}};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Image {
    background: char,
    turned: HashSet<(i32, i32)>
}

impl Image {
    fn new(background: char) -> Self {
        Image {
            turned: HashSet::new(),
            background
        }
    }

    fn set(&mut self, index: (i32, i32), value: char) {
        if value == self.background {
            self.turned.remove(&index);
        } else {
            self.turned.insert(index);
        }
    }

    fn finite(&self) -> impl Iterator<Item = &(i32, i32)> {
        return self.turned.iter();
    }
}

impl Index<(i32, i32)> for Image {
    type Output = char;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        if self.turned.contains(&index) {
            match self.background {
                '.' => &'#',
                '#' => &'.',
                _ => unreachable!()
            }
        } else {
            &self.background
        }
    }
}

fn read_input() -> (Vec<char>, Image) {
    let f = File::open("data/input20.txt").unwrap();
    let mut lines = BufReader::new(f).lines();
    let algo = lines.next().unwrap().unwrap().chars().collect();
    let mut image = Image::new('.');
    for (i, line) in lines.skip(1).enumerate() {
        let line = line.unwrap();
        for (j, ch) in line.chars().enumerate() {
            image.set((i as i32, j as i32), ch);
        }
    }
    return (algo, image);
}

fn neightbours(i: i32, j: i32) -> impl Iterator<Item = (i32, i32)> {
    (-1..=1).cartesian_product(-1..=1).map(move |(di, dj)| (i + di, j +dj))
}

fn new_value(algo: &Vec<char>, pxs: &Vec<char>) -> char {
    let mut n = 0;
    let mut k = 1;
    for c in pxs.iter().rev() {
        if *c == '#' {
            n += k;
        }
        k *= 2;
    }
    return algo[n];
}

fn enhance(image: &Image, algo: &Vec<char>) -> Image {
    let new_background = match image.background {
        '.' => algo.first(),
        '#' => algo.last(),
        _ => unreachable!(),
    }.unwrap();


    let mut result = Image::new(*new_background);
    let coords = image.finite().flat_map(|(i, j)| neightbours(*i, *j)).unique();
    for (i, j) in coords {
        let pxs: Vec<_> = neightbours(i, j).map(|(i, j)| image[(i, j)]).collect();
        result.set((i, j), new_value(algo, &pxs));
    }
    result
}

fn iterate_enhance(image: &Image, algo: &Vec<char>, n: usize) -> Image {
    (0..n).fold(image.clone(), |im, _| enhance(&im, algo))
}

fn main() {
    let (algo, image) = read_input();
    println!("Enhance 2X: {}", iterate_enhance(&image, &algo, 2).turned.len());
    println!("Enhance 50X: {}", iterate_enhance(&image, &algo, 50).turned.len());
}