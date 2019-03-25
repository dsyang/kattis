use std::io::{self};
use std::collections::HashSet;


fn compute(numbers : Vec<i64>, solutions: &mut HashSet<i64>) {
    panic!()
}
fn main() {
    let mut solutions : HashSet<i64> = HashSet::new();
    let mut input = String::new();
    let stdin = io::stdin();
    if let Ok(_) = stdin.read_line(&mut input) {
        let numbers : Vec<i64> = input.trim().split('+').map(|x| x.parse::<i64>().unwrap()).collect();
        compute(numbers, &mut solutions);
    }
    
    println!("{}", solutions.len());
}
