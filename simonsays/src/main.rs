use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    if let Ok(_) = stdin.read_line(&mut input) {
        let num: i32 = if let Ok(n) = input.trim().parse() {
            n
        } else {
            0
        };
        for _ in 0..num {
            input.clear();
            if let Ok(_) = stdin.read_line(&mut input) {
                if input.trim().starts_with("Simon says ") {
                    println!("{}", &input[11..]);
                }
            }
        }
    }
}
