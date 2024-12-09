use std::{fs, env, io::{self, BufRead}};




fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut map = vec![];
    let mut id = 0;
    let mut is_file = true;
    for c in lines.next().unwrap().unwrap().chars() {
        for _ in 0..c.to_digit(10).unwrap() {
            map.push(if is_file { Some(id) } else { None });
        }
        if is_file { id += 1; }
        is_file = !is_file;
    }
    let mut i = 0;
    while i < map.len() {
        while i < map.len() && map[i].is_some() { i += 1 }
        if i < map.len() - 1 {
            map[i] = map.pop().unwrap();
            i += 1;
            while map.last().unwrap().is_none() { map.pop(); }
        }
    }
    let result = map.iter().enumerate().map(|(i, id)| i * id.unwrap_or(0)).sum::<usize>();
    println!("{}", result);
}
