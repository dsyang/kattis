use std::io::{self};
use std::collections::HashMap;

fn tile(left: i64, memo: mut HashMap<i64, i64>) -> (i64, HashMap<i64, i64>) {
    match memo.get(&left) {
        None => {
            let (a,b) = tile(left, memo);
        },
        Some(x) => {

        }
    }
    return 2 + 2*tile(left - 2, memo);
}

fn main() {
    let mut input = String::new();
    let mut memo : HashMap<i64, i64> = HashMap::new();
    memo.insert(0, 0);
    memo.insert(2, 3);
    loop {
        if let Ok(_) = io::stdin().read_line(&mut input) {
            let num: i64 = input.trim().parse().unwrap();
            if num == -1 {
                return;
            } else {

                println!("{}", tile(num, memo));
            }
            input.clear();
        }
    }
}
