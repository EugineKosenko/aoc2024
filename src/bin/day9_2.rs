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
    for id in (0..=map.last().unwrap().unwrap()).rev() {
        let pos = map.iter().enumerate()
            .filter(|item| item.1.map(|fid| fid == id).unwrap_or(false))
            .map(|item| item.0)
            .collect::<Vec<_>>();
        let mut i = 0;
        while i < pos[0] {
            while i < pos[0] && map[i].is_some() { i +=1 }
            let mut j = i;
            while map[j].is_none() { j += 1 }
            if j - i >= pos.len() { break; }
            i = j;
        }
        if i < pos[0] {
            for k in pos {
                map[i] = Some(id);
                map[k] = None;
                i += 1;
            }
        }
    }
    let result = map.iter().enumerate().map(|(i, id)| i * id.unwrap_or(0)).sum::<usize>();
    println!("{}", result);
}
