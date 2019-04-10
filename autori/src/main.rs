use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    println!(
        "{}",
        stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split('-')
            .map(|x| x.chars().next().unwrap())
            .collect::<String>()
    );
}
