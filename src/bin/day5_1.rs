use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeSet;



fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut rules = BTreeSet::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() { break; }
        let mut pages = line.split('|');
        rules.insert((pages.next().unwrap().parse::<usize>().unwrap(),
                      pages.next().unwrap().parse::<usize>().unwrap()));
    }
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let seq = line.split(',')
            .map(|page| page.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut is_proper = true;
        for i in 0..seq.len() {
            if is_proper {
                for j in i+1..seq.len() {
                    if !rules.contains(&(seq[i], seq[j])) {
                        is_proper = false;
                        break;
                    }
                }
            }
        }
        if is_proper {
            result += seq[(seq.len() - 1) / 2];
        }
    }
    println!("{}", result);
}
