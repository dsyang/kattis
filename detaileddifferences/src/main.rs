use std::io::{self, BufRead};

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let num = input.trim().parse::<i64>().unwrap();
        let stdin = io::stdin();
        for _ in 0..num {
            let mut lines = stdin.lock().lines().take(2);
            let line1 = String::from(lines.next().unwrap().unwrap().trim());
            let line2 = String::from(lines.next().unwrap().unwrap().trim());
            println!(
                "{}\n{}\n{}\n",
                line1,
                line2,
                line1
                    .chars()
                    .zip(line2.chars())
                    .map(|(a, b)| if a == b { '.' } else { '*' })
                    .collect::<String>()
            )
        }
    }
}
