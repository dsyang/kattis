use std::io;

fn main() {
    let mut input = String::new();
    if let Ok(_) = io::stdin().read_line(&mut input) {
        let numbers = input
            .trim()
            .split(' ')
            .map(|x| match x.trim().parse::<i32>() {
                Ok(n) => return n,
                Err(_) => {
                    println!("Can't parse: {}", x);
                    return 0;
                }
            })
            .collect::<Vec<i32>>();
        let mut time = numbers[1];
        //let mut taketime = 0;
        let mut jobs = 0;
        input.clear();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            let job_times = input
                .trim()
                .split(' ')
                .map(|x| match x.trim().parse::<i32>() {
                    Ok(n) => return n,
                    Err(_) => {
                        println!("Can't parse: {}", x);
                        return 0;
                    }
                });
            for i in job_times {
                time -= i;
                if time < 0 {
                    break;
                } else {
                    jobs += 1;
                }
            }
            println!("{}", jobs);
        }
    }
}
