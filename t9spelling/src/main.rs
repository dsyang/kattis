use std::collections::HashMap;
use std::io;

fn main() {
    let mut t9: HashMap<char, &str> = HashMap::new();
    t9.insert('a', "2");
    t9.insert('b', "22");
    t9.insert('c', "222");
    t9.insert('d', "3");
    t9.insert('e', "33");
    t9.insert('f', "333");
    t9.insert('g', "4");
    t9.insert('h', "44");
    t9.insert('i', "444");
    t9.insert('j', "5");
    t9.insert('k', "55");
    t9.insert('l', "555");
    t9.insert('m', "6");
    t9.insert('n', "66");
    t9.insert('o', "666");
    t9.insert('p', "7");
    t9.insert('q', "77");
    t9.insert('r', "777");
    t9.insert('s', "7777");
    t9.insert('t', "8");
    t9.insert('u', "88");
    t9.insert('v', "888");
    t9.insert('w', "9");
    t9.insert('x', "99");
    t9.insert('y', "999");
    t9.insert('z', "9999");
    t9.insert(' ', "0");

    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let num_cases: i64 = input.trim().parse().unwrap();
        input.clear();
        for x in 1..(num_cases + 1) {
            if let Ok(_) = io::stdin().read_line(&mut input) {
                let list = input.trim().chars().map(|c| {
                    t9[&c].to_string()
                }).collect::<Vec<String>>();

                let mut output = format!("Case #{}: ", x);
                for code in list {
                    if output.chars().last().unwrap() == code.chars().next().unwrap() {
                        output = format!("{} {}", output, code);
                    } else {
                        output = format!("{}{}", output, code);
                    }
                }
                println!("{}", output);
                input.clear();
            }
        }
    }
    println!("");
}
/*
4
hi
yes
foo  bar
hello world

Case #1: 44 444
Case #2: 999337777
Case #3: 333666 6660 022 2777
Case #4: 4433555 555666096667775553
*/