use std::{io::BufRead, collections::{HashSet, VecDeque}};

use itertools::Itertools;

type Coord = i32;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Vector([Coord; 3]);

impl Vector {
    fn zeros() -> Vector {
        Vector([0; 3])
    }

    fn add(&self, other: &Vector) -> Vector {
        let mut result = self.clone();
        for i in 0..3 {
            result.0[i] += other.0[i];
        }
        return result
    }

    fn sub(&self, other: &Vector) -> Vector {
        let mut result = self.clone();
        for i in 0..3 {
            result.0[i] -= other.0[i];
        }
        return result
    }

    fn dist(&self, other: &Self) -> Coord {
        self.0.iter().zip(other.0).map(|(s, o)| (s - o).abs()).sum()
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Matrix([Coord; 9]);

impl Matrix {
    fn zeros() -> Self {
        Matrix([0; 9])
    }

    fn id() -> Self {
        Matrix([ 1,  0,  0,
                 0,  1,  0,
                 0,  0,  1 ])
    }

    fn rotate_x() -> Self {
        Matrix([ 1,  0,  0,
                 0,  0, -1,
                 0,  1,  0 ])
    }

    fn rotate_y() -> Self {
        Matrix([ 0,  0,  1,
                 0,  1,  0,
                -1,  0,  0 ])
    }

    fn rotate_z() -> Self {
        Matrix([ 0, -1,  0,
                 1,  0,  0,
                 0,  0,  1 ])
    }

    fn repeat(&self, r: usize) -> Self {
        let mut result = Matrix::id();
        for _ in 0..r {
            result = result.compose(self);
        }
        result
    }

    fn all_rotations() -> Vec<Matrix> {
        let mut result: HashSet<Matrix> = HashSet::new();
        for x_rot in 0..4 {
            for y_rot in 0..4 {
                for z_rot in 0..4 {
                    let m_x = Matrix::rotate_x().repeat(x_rot);
                    let m_y = Matrix::rotate_y().repeat(y_rot);
                    let m_z = Matrix::rotate_z().repeat(z_rot);
                    result.insert(m_x.compose(&m_y).compose(&m_z));
                }
            }
        }
        return result.into_iter().collect()
    }

    fn get(&self, i: usize, j: usize) -> Coord {
        self.0[i + j * 3]
    }

    fn get_mut(&mut self, i: usize, j: usize) -> &mut Coord {
        &mut self.0[i + j * 3]
    }

    fn compose(&self, other: &Self) -> Matrix {
        let mut result = Matrix::zeros();
        for j in 0..3 {
            for i in 0..3 {
                for k in 0..3 {
                    *result.get_mut(i, j) += self.get(k, j) * other.get(i, k);
                }
            }
        }
        return result
    }

    fn apply(&self, v: &Vector) -> Vector {
        let mut result = Vector::zeros();
        for j in 0..3 {
            for i in 0..3 {
                result.0[j] += self.get(i, j) * v.0[i]
            }
        }
        return result
    }
}

#[derive(Debug)]
struct Scaner {
    beacons: HashSet<Vector>
}

fn read_input() -> Vec<Scaner> {
    let f = std::fs::File::open("data/input19.txt").unwrap();
    let lines = std::io::BufReader::new(f).lines().map(|line| line.unwrap());
    lines.group_by(|line| line == "").into_iter().filter_map(|(key, mut lines)| {
        if key == false {
            lines.next();
            let beacons: HashSet<_> = lines.map(|line| {
                let mut numbers = line.split(",");
                Vector([numbers.next().unwrap().parse().unwrap(),
                        numbers.next().unwrap().parse().unwrap(),
                        numbers.next().unwrap().parse().unwrap()])
            }).collect();
            Some(Scaner {
                beacons
            })
        } else {
            None
        }
    }).collect()
}

const MIN_COMMON_BEACONS: usize = 12;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum Transformation {
    Rotation(Matrix),
    Offset(Vector)
}

impl Transformation {
    fn apply(&self, v: &Vector) -> Vector {
        match self {
            Transformation::Rotation(m) => m.apply(v),
            Transformation::Offset(d) => v.add(d),
        }
    }
}

fn relative_position(s1: &Scaner, s2: &Scaner) -> Option<Vec<Transformation>> {
    for rot in Matrix::all_rotations() {
        let beacons: Vec<_> = s2.beacons.iter().map(|b| rot.apply(b)).collect();
        for b1 in &s1.beacons {
            for b2 in &beacons {
                let d = b1.sub(b2);
                let count = beacons.iter()
                              .map(|b| b.add(&d))
                              .filter(|b| s1.beacons.contains(b))
                              .count();
                if count >= MIN_COMMON_BEACONS {
                    return Some(vec![Transformation::Rotation(rot),
                                     Transformation::Offset(d)]);
                }
            }
        }
    }
    return None;
}

fn all_scaners_and_beacons(scaners: &Vec<Scaner>) -> (Vec<Vector>, Vec<Vector>) {
    let mut q: VecDeque<(usize, Vec<Transformation>)> = VecDeque::new();
    let mut known_ids = HashSet::<usize>::new();
    let mut beacons = HashSet::<Vector>::new();
    let mut scaners_positions = Vec::<Vector>::new();

    q.push_back((0, Vec::new()));
    known_ids.insert(0);

    while let Some((id, tr)) = q.pop_front() {
        scaners_positions.push(tr.iter().fold(Vector::zeros(), |p, tr| tr.apply(&p)));

        for b in &scaners[id].beacons {
            let b = tr.iter().fold(b.clone(), |b, tr| tr.apply(&b));
            beacons.insert(b);
        }

        for neighbour_id in 0..scaners.len() {
            if known_ids.contains(&neighbour_id) {
                continue;
            }

            if let Some(mut neigh_tr) = relative_position(&scaners[id], &scaners[neighbour_id]) {
                neigh_tr.extend(tr.clone());
                known_ids.insert(neighbour_id);
                q.push_back((neighbour_id, neigh_tr));
            }
        }
    }
    return (scaners_positions, beacons.into_iter().collect());
}

fn max_distance(beacons: &Vec<Vector>) -> Coord {
    return beacons.iter().cartesian_product(beacons.iter()).map(|(b1, b2)| b1.dist(b2)).max().unwrap();
}

fn main() {
    let sc = read_input();
    let (scaner_positions, beacons) = all_scaners_and_beacons(&sc);
    println!("Beacons count: {}", beacons.len());
    println!("Max distance: {}", max_distance(&scaner_positions));

}