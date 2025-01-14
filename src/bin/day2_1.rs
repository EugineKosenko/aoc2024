use std::{fs, env, io::{self, BufRead}};

fn is_proper(v: &[usize]) -> bool {
    if !v.is_sorted_by(|a, b| a <= b) && !v.is_sorted_by(|a, b| a >= b) {
        return false;
    }
    let mut v = v.iter();
    let mut p = *v.next().unwrap();
    for &n in v {
        let d = n.abs_diff(p);
        if !(1..=3).contains(&d) {
            return false;
        }
        p = n;
    }
    true
}

fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines().collect::<Vec<_>>();
    for line in lines {
        let line = line.unwrap();
        let v = line.split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if is_proper(&v) { result += 1; }
    }
    println!("{}", result);
}
