use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeMap;
use std::collections::BTreeSet;




fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut nodes = BTreeMap::<char, Vec<(usize, usize)>>::new();
    let (mut rows, mut cols) = (0isize, 0isize);
    for (r, row) in lines.enumerate() {
        rows += 1;
        let row = row.unwrap();
        cols = row.len() as isize;
        for (c, a) in row.chars().enumerate() {
            if a != '.' {
                nodes.entry(a).or_default().push((r, c));
            }
        }
    }
    let mut antinodes = BTreeSet::<(usize, usize)>::new();
    for nodes in nodes.values() {
        for i in 0..nodes.len() {
            let n1 = nodes[i];
            let r1 = n1.0 as isize;
            let c1 = n1.1 as isize;
            for n2 in nodes.iter().skip(i+1) {
                let r2 = n2.0 as isize;
                let c2 = n2.1 as isize;
                let (dr, dc) = (r2 - r1, c2 - c1);
                let ar = r1 - dr;
                let ac = c1 - dc;
                if ar >= 0 && ac >= 0 && ar < rows && ac < cols {
                    antinodes.insert((ar as usize, ac as usize));
                }
                let ar = r2 + dr;
                let ac = c2 + dc;
                if ar >= 0 && ac >= 0 && ar < rows && ac < cols {
                    antinodes.insert((ar as usize, ac as usize));
                }
            }
        }
    }
    let result = antinodes.len();
    println!("{}", result);
}
