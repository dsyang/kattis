use std::collections::HashMap;
use std::io::{self, BufRead};

fn rotate<'a>(
    int_char: &'a HashMap<i32, char>,
    map: &HashMap<char, i32>,
    num_rotate: i64,
    c: char,
) -> &'a char {
    let idx = map.get(&c).unwrap();
    return int_char
        .get(&((idx + num_rotate as i32) % int_char.len() as i32))
        .unwrap();
}
fn main() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ_.";
    let int_char: HashMap<i32, char> = alphabet
        .char_indices()
        .map(|(a, b)| (a as i32, b))
        .collect();
    let char_int: HashMap<char, i32> = alphabet
        .char_indices()
        .map(|(a, b)| (b, a as i32))
        .collect();

    let stdin = io::stdin();
    let _lines = stdin.lock().lines().take_while(|s| {
        !(if let Ok(input) = s {
            input.trim().parse::<i64>() == Ok(0)
        } else {
            false
        })
    });

    for line in _lines {
        if let Ok(input_str) = line {
            let mut inputs = input_str.trim().split(' ');
            let num_rotate = inputs.next().unwrap().trim().parse::<i64>().unwrap();
            println!(
                "{}",
                inputs
                    .next()
                    .unwrap()
                    .trim()
                    .chars()
                    .rev()
                    .map(|c| rotate(&int_char, &char_int, num_rotate, c))
                    .collect::<String>()
            );
        }
    }
}
