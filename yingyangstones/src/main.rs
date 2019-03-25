use std::char;
use std::io;

fn look_and_say(s: &str) -> Vec<(char, u32)> {
    if s.is_empty() {
        return vec![];
    }
    let mut output: Vec<(char, u32)> = vec![];
    let mut current: char = match s.chars().next() {
        Some(c) => c,
        None => {
            println!("No more chars :(");
            panic!();
        }
    };
    let mut counter: u32 = 0;
    for c in s.chars() {
        if c == current {
            counter += 1;
        } else {
            output.push((current, counter));
            current = c;
            counter = 1;
        }
    }
    output.push((current, counter));
    return output;
}

fn perform_operation(circle: Vec<(char, u32)>, idx: usize) -> (bool, Vec<(char, u32)>) {
    // If all one letter no operations left or we are past size in index
    if circle.len() == 1 || idx >= circle.len()-1 {
        return (true, circle);
    }

    let mut output = vec![];
    for x in 0..idx {
        output.push(circle[x]);
    }
    let (c1, n1) = circle[idx];
    let (c2, n2) = circle[idx+1];
    match (n1 < n2, n1 > n2) {
        (true, false) => {
            output.push((c1, 1));
            output.push((c2, (n2 - n1) + 1)); //WWWBBBBBB -> WWWBB BBBB -> W BBBB
        }
        (false, true) => {
            // WWWWWBB -> WW WWWBB -> WW W -> WWW
            output.push((c1, (n1 - n2)));
        }
        (false, false) => {
            // WWWBBB -> WWWBB B -> WB
            output.push((c1, 1));
            output.push((c2, 1));
        }
        _ => panic!(),
    }
    println!("after idx: {} output: {:?}, input; {:?}", idx, output, circle);
    if circle.len() > idx + 2 {
        if let Some((c2, _)) = output.last().cloned() {
            let (c3, n3) = circle[idx + 2];
            if c2 == c3 {
                if let Some((c2, n2)) = output.pop() {
                    output.push((c2, n2 + n3));
                }
            } else {
                output.push((c3, n3));
            }
        }
        for x in (idx+3)..circle.len() {
            output.push(circle[x])
        }
    }
    let output = sanitize(output);
    println!("Finish operation {:?}", output);
    return (output.len() <= 2, output);
}

fn sanitize(mut v: Vec<(char, u32)>) -> Vec<(char, u32)> {
    if v.len() > 1 {
        // combine first and last if same letter
        match (v.first().cloned(), v.last().cloned()) {
            (Some((c1, n1)), Some((c2, n2))) => {
                if c1 == c2 {
                    v[0] = (char::clone(&c1), n1 + n2);
                    v.pop();
                }
            }
            _ => (),
        }
    }
    return v;
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    if let Ok(_) = stdin.read_line(&mut input) {
        let mut new_input = sanitize(look_and_say(&input.trim()));
        let mut x = 0;
        loop {
            let (done, output) = perform_operation(new_input, x);
            new_input = output;
            if done {
                break;
            } else {
                x += 1;
            }
        }
        println!("{:?}", new_input);
        if new_input.len() == 2 {
            match (
                new_input[0].0 != new_input[1].0,
                new_input[0].1 == new_input[1].1 && new_input[0].1 == 1,
            ) {
                (true, true) => println!("1"),
                _ => println!("0"),
            }
        } else {
            println!("0");
        }
    }
}
