use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    if let Ok(_) = stdin.read_line(&mut input) {
        let v = input
            .split(' ')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let ans: i64 = v[0] * (v[1] - 1) + 1;
        println!("{}", ans);
    }
}
