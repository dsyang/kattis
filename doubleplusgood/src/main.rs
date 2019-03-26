use std::io::{self};
use std::collections::HashSet;


fn create(numbers : &Vec<&str>, solutions: &mut Vec<String>, prefix: &str, idx : usize) {
    if idx == numbers.len() {
        solutions.push(prefix.to_owned());
        return;
    }
    if prefix != "" {
        create(numbers, solutions, &format!("{}+{}", prefix, numbers[idx]), idx+1);
    }
    create(numbers, solutions, &format!("{}{}", prefix, numbers[idx]), idx+1);
}

fn compute(solutions: &Vec<String>) -> usize {
    let mut uniques : HashSet<i64> = HashSet::new();
    for equation in solutions {
        let number : i64 = equation.trim().split('+').map(|x| x.parse::<i64>().unwrap()).fold(0, |acc,x| acc+x);
        uniques.insert(number);
    }
    return uniques.len();
}

fn main() {
    let mut solutions : Vec<String> = Vec::new();
    let mut input = String::new();
    let stdin = io::stdin();
    if let Ok(_) = stdin.read_line(&mut input) {
        let items : Vec<&str> = input.trim().split('+').collect();
        create(&items, &mut solutions, "", 0);
    }
    println!("{:?}", solutions);
    
    println!("{}", compute(&solutions));
    
    
}
