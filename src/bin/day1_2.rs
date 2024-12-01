use std::{fs, env, io::{self, BufRead}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines().collect::<Vec<_>>();
    let mut vec1 = vec![];
    let mut vec2 = vec![];
    for line in lines {
        let line = line.unwrap();
        let mut line = line.split_whitespace();
        let v = line.next().unwrap().parse::<usize>().unwrap();
        vec1.push(v);
        let v = line.next().unwrap().parse::<usize>().unwrap();
        vec2.push(v);
    }
    let result: usize = vec1.iter()
        .map(|&v1| v1 * vec2.iter()
             .filter(|&&v2| v2 == v1)
             .count())
        .sum();
    println!("{}", result);
}
