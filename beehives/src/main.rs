use std::io;

/*
10.0 3
3.0 3.0
3.5 4.5
20.0 20.0
5.0 4
2.0 1.0
2.0 4.0
2.0 8.0
2.0 12.0
0.0 0

2 sour, 1 sweet
4 sour, 0 sweet
*/

fn calc( (x1,y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
    return ((x1 - x2).powf(2.0) + (y1-y2).powf(2.0)).sqrt();
}
fn main() {
    let mut input = String::new();
    loop {
        input.clear();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            let mut nums = input.trim().split(' ');
            let dist: f64 = nums.next().unwrap().parse().unwrap();
            let hives: i64 = nums.next().unwrap().parse().unwrap();
            
            if dist == 0.0 && hives == 0 { return;}

            let distances: Vec<(f64, f64)> = (0..hives).map(|_| {
                let mut input = String::new();
                if let Ok(_) = io::stdin().read_line(&mut input) {
                    let nums: Vec<f64> = input.trim().split(' ').map(|x| x.parse().unwrap()).collect();
                    return (nums[0], nums[1]);
                } else {
                    panic!();
                }
            }).collect();

            let mut sour: i64 = 0;
            let mut sweet: i64 = 0;
            'outer: for x in 0..distances.len() {
                let mut is_sour = false;
                'inner: for y in 0..distances.len() {
                    if x == y { continue; }
                    if calc(distances[x], distances[y]) < dist {
                        is_sour = true;
                        break 'inner;
                    }
                }
                if is_sour {
                    sour += 1;
                } else {
                    sweet += 1;
                }
            }
            println!("{} sour, {} sweet", sour, sweet);
        }
    }
}
