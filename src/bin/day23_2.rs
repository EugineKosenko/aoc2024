use std::{fs, env, io::{self, BufRead}};
use std::collections::{BTreeSet, BTreeMap};


type Cluster = BTreeSet<String>;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut nghbrs = BTreeMap::<String, Cluster>::new();
    let mut links = BTreeSet::new();
    for line in lines {
        let (c1, c2) = line.split_once('-').unwrap();
        links.insert((c1.to_string(), c2.to_string()));
        links.insert((c2.to_string(), c1.to_string()));
        nghbrs.entry(c1.to_string()).or_default().insert(c2.to_string());
        nghbrs.entry(c2.to_string()).or_default().insert(c1.to_string());
    }
    let nodes = nghbrs.keys().map(|key| key.clone()).collect::<BTreeSet<String>>();
    let mut result = Cluster::new();
    let mut queue = BTreeSet::from([(0, Cluster::new())]);
    let mut i = 0;
    while let Some((_, cluster)) = queue.pop_last() {
        i += 1;
        if i % 1 == 0 {
            println!(
                "{} {} {} {:?}", i, queue.len(),
                queue.iter().map(|(len, _)| len).max().unwrap_or(&0),
                cluster);
        }
        let mut is_extended = false;
        for node in nodes.iter() {
            if !cluster.contains(node) && cluster.is_subset(nghbrs.get(node).unwrap()) {
                let mut cluster = cluster.clone();
                cluster.insert(node.clone());
                queue.insert((cluster.len(), cluster));
                is_extended = true;
            }
        }
        if !is_extended && cluster.len() > result.len() {
            result = cluster;
            println!("{}", itertools::intersperse(result.iter().map(|node| node.clone()), ",".to_string()).collect::<String>());
        }
    }
    let result = itertools::intersperse(result.into_iter(), ",".to_string()).collect::<String>();
    println!("{}", result);
}
