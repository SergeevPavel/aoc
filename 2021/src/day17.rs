struct Area {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

fn main() {
//    let a = Area {
//        x_min: 20,
//        x_max: 30,
//        y_min: -10,
//        y_max: -5,
//    };
    let a = Area {
        x_min: 257,
        x_max: 286,
        y_min: -101,
        y_max: -57,
    };

    let mut result = 0;
    for dx in 0..(a.x_max + 1) {
        for dy in (a.y_min - 1)..(a.y_min.abs() + 1) {
            let mut x = 0;
            let mut y = 0;
            let mut dx = dx;
            let mut dy = dy;
            loop {
                if x >= a.x_min && x <= a.x_max && y >= a.y_min && y <= a.y_max {
                    result += 1;
                    break;
                }
                if x > a.x_max || y < a.y_min {
                    break;
                }
                x += dx;
                y += dy;
                dx = if dx > 0 { dx - 1 } else { dx };
                dy -= 1;
            }
        }
    }
    println!("result: {:?}", result);

//    for dx in 0..(a.x_max + 1) {
//        println!("dx: {:?}", dx);
//        let mut i = 0;
//        let mut x = 0;
//        let mut dx = dx;
//        loop {
//            if x >= a.x_min && x <= a.x_max {
//                println!("i: {:?}{}", i, if dx == 0 { "+" } else { "" });
//            }
//            if x > a.x_max || dx == 0 {
//                break;
//            }
//            i += 1;
//            x += dx;
//
//        }
//    }
//    println!("<<<======>>>");
//    for dy in 0..(a.y_min.abs() + 1) {
//        println!("dy: {:?}", dy);
//        let mut i = 0;
//        let mut y = 0;
//        let mut dy = dy;
//        loop {
//            if y >= a.y_min && y <= a.y_max {
//                println!("i: {:?} y: {}", i, y);
//            }
//            if y < a.y_min {
//                break;
//            }
//            i += 1;
//            y += dy;
//            dy -= 1;
//        }
//    }
}

// .............#....#............
// .......#..............#........
// ...............................
// S........................#.....
// ...............................
// ...............................
// ...........................#...
// ...............................
// ....................TTTTTTTTTTT
// ....................TTTTTTTTTTT
// ....................TTTTTTTT#TT
// ....................TTTTTTTTTTT
// ....................TTTTTTTTTTT
// ....................TTTTTTTTTTT
