use std::io;

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let mut word = input.trim().clone().chars().collect::<Vec<char>>();
        word.sort();
        word.dedup();
        let mut num_to_guess = word.len();
        let mut tries = 10;
        //println!("{:?}", word);
        input.clear();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            for c in input.trim().chars() {
                if word.contains(&c) {
                    //println!("Guessed {}", c);
                    num_to_guess -= 1;
                } else {
                    tries -= 1;
                }
                if (num_to_guess == 0 && tries > 0) {
                    println!("WIN");
                    return;
                }
            }
            println!("LOSE");
        }
    }
}
