use std::{fs, env, io::{self, BufRead}};





fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .peekable();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() { lines.next().unwrap(); }
        let mut item = [0, 0, 0, 0, 0];
        for _ in 0..5 {
            let line = lines.next().unwrap();
            for (i, ch) in line.chars().enumerate() {
                if ch == '#' { item[i] += 1 }
            }
        }
        if lines.next().unwrap() == "....." { locks.push(item); } else { keys.push(item); }
    }
    let result = locks.iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| {
                    lock.iter().zip(key.iter())
                        .all(|(lock, key)| lock + key < 6)
                })
                .count()
        })
        .sum::<usize>();
    println!("{}", result);
}
