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
        let mut seq = line.split(',')
            .map(|page| page.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut improper_pair = None;
        for i in 0..seq.len() {
            if improper_pair.is_none() {
                for j in i+1..seq.len() {
                    if !rules.contains(&(seq[i], seq[j])) {
                        improper_pair = Some((i, j));
                        break;
                    }
                }
            }
        }
        if improper_pair.is_some() {
            loop {
                match improper_pair {
                    None => { break; },
                    Some((i, j)) => { (seq[i], seq[j]) = (seq[j], seq[i]); }
                }
                improper_pair = None;
                for i in 0..seq.len() {
                    if improper_pair.is_none() {
                        for j in i+1..seq.len() {
                            if !rules.contains(&(seq[i], seq[j])) {
                                improper_pair = Some((i, j));
                                break;
                            }
                        }
                    }
                }
            }
            result += seq[(seq.len() - 1) / 2];
        }
    }
    println!("{}", result);
}
