use std::{fs, env, io::{self, BufRead}};





fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut result = 0;
    for line in lines {
        let mut value = line.parse::<usize>().unwrap();
        for _ in 0..2000 {
            value = ((64 * value) ^ value) % 16777216;
            value = ((value / 32) ^ value) % 16777216;
            value = ((2048 * value) ^ value) % 16777216;
        }
        result += value;
    }
    println!("{}", result);
}
