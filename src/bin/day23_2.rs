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
    let nodes = nghbrs.keys().cloned().collect::<BTreeSet<String>>();
    let mut visited = BTreeSet::new();
    let result = nodes.into_iter()
        .filter_map(|start| {
            if visited.contains(&start) { None } else {
                let mut cluster = Cluster::new();
                let mut queue = BTreeSet::from([start]);
                while let Some(node) = queue.pop_first() {
                    if !visited.contains(&node) && cluster.is_subset(nghbrs.get(&node).unwrap()) {
                        visited.insert(node.clone());
                        cluster.insert(node.clone());
                        queue = queue.union(&nghbrs.get(&node).unwrap().difference(&visited).cloned().collect()).cloned().collect();
                    }
                }
                Some(cluster)
            }
        })
        .max_by_key(|cluster| cluster.len())
        .unwrap();
    let result = itertools::intersperse(result, ",".to_string()).collect::<String>();
    println!("{}", result);
}
