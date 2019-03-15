use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let x = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();
    let y = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();
    match (x > 0, y > 0) {
        (true, true) => println!("1"),
        (false, true) => println!("2"),
        (false, false) => println!("3"),
        (true, false) => println!("4"),
    }
}
