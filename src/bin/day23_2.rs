use std::{fs, env, io::{self, BufRead}};
use std::collections::{BTreeSet, BTreeMap};


type Set = BTreeSet<String>;
fn largest(set: &mut Set, cnds: &Set, nghbrs: &BTreeMap<String, Set>) -> Set {
    cnds.iter()
        .filter_map(|cnd| {
            if !set.contains(cnd) && set.is_subset(nghbrs.get(cnd).unwrap()) {
                set.insert(cnd.to_string());
                let result = Some(largest(set, nghbrs.get(cnd).unwrap(), nghbrs));
                set.remove(cnd);
                result
            } else {
                None
            }
        })
        .max_by_key(|cnd| cnd.len())
        .unwrap_or_else(|| {
            println!("{:?}", set);
            set.clone()
        })
}

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
    let result = itertools::intersperse(
        largest(
            &mut BTreeSet::new(),
            &nghbrs.keys().map(|key| key.clone()).collect(),
            &nghbrs),
        ",".to_string())
        .collect::<String>();
    println!("{}", result);
}
