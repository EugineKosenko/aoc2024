use std::{fs, env, io::{self, BufRead}};





fn main() {
    let mut result: usize = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let values = io::BufReader::new(file)
        .lines().nth(0).unwrap().unwrap()
        .split(' ')
        .map(|value| value.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut queue = [(0, 0); 1000];
    let mut qi = 0;
    for &value in &values {
        println!("Value {}", value);
        queue[qi] = (value, 25);
        qi += 1;
        while qi > 0 {
            //if result % 100_000_000 == 0 { println!("{} {}", result, qi); }
            qi -= 1;
            let (mut value, count) = queue[qi];
            for j in (0..count).rev() {
                if value == 0 { value = 1; continue; }
                let len = (value as f32).log10().floor() as u32 + 1;
                if len % 2 == 0 {
                    let p = 10_usize.pow(len / 2);
                    queue[qi] = (value % p, j);
                    qi += 1;
                    value /= p;
                } else {
                    value = value.checked_mul(2024).unwrap();
                }
            }
            result += 1;
        }
    }
    println!("{}", result);
}
