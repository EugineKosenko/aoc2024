use std::{fs, env, io::{self, BufRead}};
use std::collections::{BTreeSet, BTreeMap};


type Set = BTreeSet<String>;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut nghbrs = BTreeMap::<String, Set>::new();
    let mut links = BTreeSet::new();
    for line in lines {
        let (c1, c2) = line.split_once('-').unwrap();
        links.insert((c1.to_string(), c2.to_string()));
        links.insert((c2.to_string(), c1.to_string()));
        nghbrs.entry(c1.to_string()).or_default().insert(c2.to_string());
        nghbrs.entry(c2.to_string()).or_default().insert(c1.to_string());
    }
    //println!("{:?}", links);
    //println!("{:?}", nghbrs);
    let triads = nghbrs.iter()
        .flat_map(|(first, seconds)| {
            seconds.iter()
                .filter(|second| first.clone() != **second)
                .flat_map(|second| {
                    nghbrs.get(second).unwrap().iter()
                        .filter_map(|third| {
                            if second.clone() != *third
                                && links.contains(&(third.clone(), first.clone()))
                                && (first.starts_with('t') || second.starts_with('t') || third.starts_with('t')) {
                                    Some(BTreeSet::from([first.clone(), second.clone(), third.clone()]))
                                } else {
                                    None
                                }
                        })
                })
        })
        .collect::<BTreeSet<_>>();
    let result = triads.len();
    println!("{}", result);
}
