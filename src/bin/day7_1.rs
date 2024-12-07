use std::{fs, env, io::{self, BufRead}};


fn is_proper(value: usize, result: usize, args: &[usize]) -> bool {
    if args.is_empty() {
        value == result
    } else {
        is_proper(value, result + args[0], &args[1..])
            || is_proper(value, result * args[0], &args[1..])
    }
}

fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines();
    for line in lines.into_iter() {
        let line = line.unwrap();
        let (value, args) = line.split_once(": ").unwrap();
        let value = value.parse::<usize>().unwrap();
        let args = args
            .split_whitespace()
            .map(|arg| arg.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if is_proper(value, args[0], &args[1..]) {
            result += value;
        }
    }
    println!("{}", result);
}
