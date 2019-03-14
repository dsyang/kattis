use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let num : i64 = line.trim().parse::<i64>().unwrap();
    print!("{}", if num % 2 == 1 { "Alice" } else { "Bob" })
}
