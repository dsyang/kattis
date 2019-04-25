use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        if let Ok(_) = stdin.read_line(&mut input) {
            match input.trim().parse::<i32>() {
                Ok(x) => {
                    if x == 0 {
                        return;
                    }
                    let mut v = (0..x).map(|_| { String::new()}).collect();
                    //v.resize_with(x, || {String::new()});
                    for i in 0..x {
                        if let Ok(_) = stdin.read_line(&mut input) {
                            if i % 2 == 0:
                            v.push(String::from(input.trim()));
                        }
                    }
                    

                },
                Err(_) => return,
            }
        }
    }
}
