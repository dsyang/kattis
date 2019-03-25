use std::io::{self};

fn process(input: &str) -> (i64, String) {
    let items : Vec<&str>= input.trim().split(' ').collect();
    assert!(items.len() == 2);
    if let Ok(num) = items[0].trim().parse::<i64>() {
        return (num/2, items[1].to_string());
    } else if let Ok(num) = items[1].trim().parse::<i64>() {
        return (num, items[0].to_string());
    } else {
        panic!();
    }
}

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let num_cups: i64 = input.trim().parse().unwrap();
        let mut things: Vec<(i64, String)> = Vec::new();
        input.clear();
        for _ in 0..num_cups {
            input.clear();
            if let Ok(_) = io::stdin().read_line(&mut input) {
                things.push(process(&input))
            }
        }
        things.sort_by(|a,b| a.0.partial_cmp(&b.0).unwrap());
        for x in things {
            println!("{}", x.1);
        }
    }

}
