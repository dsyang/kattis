use std::char;
use std::io;

fn look_and_say_decode(s: &str) -> String {
    return s
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            String::from_utf8(vec![pair[0]])
                .unwrap()
                .repeat((pair[1] as char).to_digit(10).unwrap() as usize)
        })
        .collect::<Vec<String>>()
        .concat();
}

fn look_and_say_encode(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut output = String::new();
    let mut current: char = s.chars().next().unwrap();
    let mut counter: u32 = 0;
    for c in s.chars() {
        if c == current {
            counter += 1;
        } else {
            output.push(current);
            output.push(char::from_digit(counter, 10).unwrap());
            current = c;
            counter = 1;
        }
    }
    output.push(current);
    output.push(char::from_digit(counter, 10).unwrap());
    return output;
}
fn main() {
    let mut buf = String::new();
    if let Ok(_) = io::stdin().read_line(&mut buf) {
        let mut inputs = buf.trim().split(' ');
        let encode: bool = inputs.next().unwrap().trim() == "E";
        match encode {
            true => println!("{}", look_and_say_encode(inputs.next().unwrap())),
            false => println!("{}", look_and_say_decode(inputs.next().unwrap())),
        }
    }
}
