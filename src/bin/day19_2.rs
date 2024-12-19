use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeMap;


fn combs_count(design: &str, towels: &[&str], counts: &mut BTreeMap<String, usize>) -> usize {
    if design.is_empty() { return 1; }
    if let Some(count) = counts.get(design) { return *count; }
    towels.iter()
        .filter_map(|towel| {
            design.strip_prefix(towel)
                .map(|design| {
                    let count = combs_count(design, towels, counts);
                    counts.insert(design.to_string(), count);
                    count
                })
        })
        .sum()
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
    let mut counts = BTreeMap::new();
    let result = lines
        .map(|design| combs_count(&design, &towels, &mut counts)).sum::<usize>();
    println!("{}", result);
}
