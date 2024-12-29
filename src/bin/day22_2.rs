use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeMap;




fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut snippets = BTreeMap::<_, isize>::new();
    for line in lines {
        let mut tsnippets = BTreeMap::new();
        let mut value = line.parse::<isize>().unwrap();
        let mut values = vec![];
        let mut deltas = vec![];
        for _ in 0..2000 {
            let prev = value % 10;
            value = ((64 * value) ^ value) % 16777216;
            value = ((value / 32) ^ value) % 16777216;
            value = ((2048 * value) ^ value) % 16777216;
            values.push(value % 10);
            deltas.push(value % 10 - prev);
        }
        for i in 0..deltas.len() - 3 {
            tsnippets.entry([deltas[i], deltas[i+1], deltas[i+2], deltas[i+3]]).or_insert(values[i+3]);
        }
        for (snippet, value) in tsnippets {
            *snippets.entry(snippet).or_default() += value;
        }
    }
    let result = *snippets.values().max().unwrap();
    for (snippet, value) in snippets {
        if value == result { println!("{:?}", snippet); }
    }
    println!("{}", result);
}
