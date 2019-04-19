use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    let mut trips: HashMap<String, Vec<String>> = HashMap::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let num_trips: i64 = input.trim().parse().unwrap();
        input.clear();
        for _ in 0..num_trips {
            if let Ok(_) = io::stdin().read_line(&mut input) {
                let parsed: Vec<&str> = input.trim().split(' ').collect();
                let country: &str = parsed[0];
                let year: &str = parsed[1];

                match trips.contains_key(country) {
                    true => {
                        if let Some(mut v) = trips.get_mut(country.clone()) {
                            v.push(String::from(year));
                        }
                    }
                    false => {
                        trips.insert(String::from(country), vec![String::from(year)]);
                    }
                }
            }
            input.clear();
        }

        for (_, v) in trips.iter_mut() {
            v.sort();
        }

        if let Ok(_) = io::stdin().read_line(&mut input) {
            let num_queries: i64 = input.trim().parse().unwrap();
            input.clear();
            for _ in 0..num_queries {
                if let Ok(_) = io::stdin().read_line(&mut input) {
                    let parsed: Vec<&str> = input.trim().split(' ').collect();
                    let country: &str = parsed[0];
                    let num: usize = parsed[1].parse::<usize>().unwrap();
                    println!("{}", trips.get(country).unwrap()[num-1]);
                }
                input.clear();
            }
        }
    }
}
