use std::io::{self};

fn to_decimal(num: &str, source_language: &str) -> i64 {
    // turn str into decimal by looking at length of input to 
    // determine the base.
    return 0;
}

fn from_decimal(num: i64, target_language: &str) -> String {
    // turn num into str by looking at length of target language to determine base.
    return String::new();
}

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let num_cases: i64 = input.trim().parse().unwrap();
        input.clear();
        for x in 1..num_cases+1 {
            if let Ok(_) = io::stdin().read_line(&mut input) {
                let languages = input.clone();
                let strs: Vec<&str> = languages.trim().split(' ').collect();
                let in_decimal: i64 = to_decimal(strs[0], strs[1]);
                let out: String = from_decimal(in_decimal, strs[2]);
                println!("Case #{}: {}", x, out);
                input.clear();
            }
        }
    }
    println!("Hello, world!");
}
