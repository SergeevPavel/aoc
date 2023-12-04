#![feature(linked_list_cursors)]

use std::{collections::LinkedList, io::{BufReader, BufRead}};


//#[derive(Debug)]
//enum Element {
//    Number { n: i32 },
//    Pair { p: Pair }
//}
//
//#[derive(Debug)]
//struct Pair {
//    l: Box<Element>,
//    r: Box<Element>
//}
//
//fn parse_element(s: &str) -> (Element, &str) {
//    if s.starts_with("[") {
//        let (left, remaining) = parse_element(&s[1..]);  // skip [
//        let (right, remaining) = parse_element(&remaining[1..]); // skip ,
//        let p = Pair {
//            l: Box::new(left),
//            r: Box::new(right),
//        };
//        return (Element::Pair { p }, &remaining[1..]); // skip ]
//    } else if let Ok(n) = s[0..1].parse::<i32>() {
//        return (Element::Number { n }, &s[1..]);
//    } else {
//        unreachable!()
//    }
//}

#[derive(Eq, PartialEq, Clone)]
enum Element {
    Number { n: u32 },
    Open,
    Close,
}

impl Element {
    fn unwrap_number_ref(&mut self) -> &mut u32 {
        match self {
            Element::Number { n } => n,
            Element::Open => panic!(),
            Element::Close => panic!(),
        }
    }
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Number { n } => f.write_str(&n.to_string())?,
            Element::Open => f.write_str("[")?,
            Element::Close => f.write_str("]")?,
        }
        return Ok(());
    }
}

fn parse_input(line: &str) -> LinkedList<Element> {
    line.chars().filter_map(|ch| {
        match ch {
            '[' => Some(Element::Open),
            ']' => Some(Element::Close),
            ch if ch.is_digit(10) => Some(Element::Number { n: ch.to_digit(10).unwrap() }),
            _ => None
        }
    }).collect()
}

fn explode(sn: &mut LinkedList<Element>) -> bool {
    let mut depth = 0;
    let mut cursor = sn.cursor_front_mut();
    loop {
        cursor.move_next();
        if let Some(it) = cursor.current() {
            match it {
                Element::Number { n } => {},
                Element::Open => depth += 1,
                Element::Close => depth -= 1,
            }
            if depth >= 4 {
                break;
            }
        } else {
            break;
        }
    }
    if cursor.current().is_some() {
        assert_eq!(cursor.remove_current(), Some(Element::Open));
        let left = *cursor.remove_current().unwrap().unwrap_number_ref();
        let right = *cursor.remove_current().unwrap().unwrap_number_ref();
        *cursor.current().unwrap() = Element::Number { n: 0 };
        let index = cursor.index().unwrap();
        loop {
            cursor.move_prev();
            if let Some(it) = cursor.current() {
                match it {
                    Element::Number { n } => {
                        *n += left;
                        break;
                    },
                    _ => {}
                }
            } else {
                break;
            }
        }
        drop(cursor);
        let mut cursor = sn.cursor_front_mut();
        cursor.move_next();
        for _ in 0..(index - 1) {
            cursor.move_next();
        }
        loop {
            cursor.move_next();
            if let Some(it) = cursor.current() {
                match it {
                    Element::Number { n } => {
                        *n += right;
                        break;
                    },
                    _ => {}
                }
            } else {
                break;
            }
        }
        return true;
    } else {
        return false;
    }
}

fn split(sn: &mut LinkedList<Element>) -> bool {
    let mut cursor = sn.cursor_front_mut();
    loop {
        cursor.move_next();
        if let Some(it) = cursor.current() {
            match *it {
                Element::Number { n } if n > 9 => {
                    cursor.remove_current();
                    cursor.insert_before(Element::Open);
                    cursor.insert_before(Element::Number { n: n / 2 });
                    cursor.insert_before(Element::Number { n: (n + 1) / 2 });
                    cursor.insert_before(Element::Close);
                    return true;
                },
                _ => {}
            }
        } else {
            break;
        }
    }
    return false;
}

#[test]
fn test_explode() {
    fn test_with(input: &str, output: &str) {
        let mut state = parse_input(input);
        explode(&mut state);
        assert_eq!(state, parse_input(output));
    }
    test_with("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
    test_with("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
    test_with("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
    test_with("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    test_with("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");

}

fn add(base: &mut LinkedList<Element>, other: &LinkedList<Element>) {
    base.push_front(Element::Open);
    base.append(&mut other.clone());
    base.push_back(Element::Close);
    loop {
        while explode(base) {}
        if !split(base) {
            break;
        }
    }
}

fn magnitude(els: &mut dyn Iterator<Item = Element>) -> u32 {
    match els.next() {
        Some(Element::Open) => {
            let left = magnitude(els);
            let right = magnitude(els);
            assert_eq!(els.next(), Some(Element::Close));
            return 3 * left + 2 * right;
        }
        Some(Element::Number { n }) => {
            return n;
        }
        _ => unreachable!()
    }
}

fn task1() {
    let f = std::fs::File::open("data/input18.txt").unwrap();
    let mut numbers = BufReader::new(f).lines().map(|l| parse_input(&l.unwrap()));
    let mut base = numbers.next().unwrap();
    for number in numbers {
        add(&mut base, &number);
    }
    println!("magnitude: {:?}", magnitude(&mut base.iter().cloned()));
}

fn task2() {
    let f = std::fs::File::open("data/input18.txt").unwrap();
    let numbers: Vec<_> = BufReader::new(f).lines().map(|l| parse_input(&l.unwrap())).collect();
    let mut result = 0;
    for (i, n1) in numbers.iter().enumerate() {
        for (j, n2) in numbers.iter().enumerate() {
            if i != j {
                let mut n1 = n1.clone();
                add(&mut n1, &n2);
                let m = magnitude(&mut n1.iter().cloned());
                if m > result {
                    result = m;
                }
            }
        }
    }
    println!("max magnitude: {:?}", result);
}

fn main() {
    task1();
    task2();
}