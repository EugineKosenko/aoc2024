use std::{fs, env, io::{self, BufRead}};



fn check(design: &str, towels: &[&str]) -> bool {
    if design.is_empty() { return true; }
    for towel in towels {
        if let Some(design) = design.strip_prefix(towel) {
            if check(design, towels) { return true; }
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let line = lines.next().unwrap();
    let towels = line.split(", ").collect::<Vec<_>>();
    lines.next().unwrap();
    let result = lines.filter(|design| check(design, &towels)).count();
    println!("{}", result);
}
