use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    if let Ok(_) = stdin.read_line(&mut input) {
        let num: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => 0,
        };
        for _ in 0..num {
            input.clear();
            if let Ok(_) = stdin.read_line(&mut input) {
                let mut digits: Vec<i32> = input
                    .trim()
                    .split(' ')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
                digits.remove(0);
                for i in 1..(digits.len() - 1) {
                    let out_of_order = (digits[i] > digits[i - 1] && digits[i] > digits[i + 1])
                        || (digits[i] < digits[i - 1] && digits[i] < digits[i + 1]);
                    if out_of_order && digits[i - 1] < digits[i + 1] {
                        println!("{}", i + 1);
                    } else {
                    }
                }
            }
        }
    }
}
