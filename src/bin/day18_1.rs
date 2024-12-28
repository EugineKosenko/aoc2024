use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeSet;





fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let rows = args[2].parse::<usize>().unwrap();
    let cols = args[3].parse::<usize>().unwrap();
    let count = args[4].parse::<usize>().unwrap();
    let mut board = grid::Grid::init(rows, cols, '.');
    for _ in 0..count {
        let line = lines.next().unwrap();
        let point = line
            .split_once(',').unwrap();
        let point = (point.0.parse::<usize>().unwrap(), point.1.parse::<usize>().unwrap());
        *board.get_mut(point.1, point.0).unwrap() = '#';
    }
    let start = (0, 0);
    let finish = (rows - 1, cols - 1);
    let mut result = 0;
    let mut dists = grid::Grid::init(rows, cols, 0);
    let mut queue = BTreeSet::from([(1, start)]);
    let mut is_found = false;
    while let Some((dist, point)) = queue.pop_first() {
        if point == finish { result = dist - 1; is_found = true; break; }
        *dists.get_mut(point.0, point.1).unwrap() = dist;
        for step in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let row: usize = match (point.0 as isize + step.0).try_into() {
                Err(_) => { continue; },
                Ok(row) => row
            };
            if row == rows { continue; }
            let col: usize = match (point.1 as isize + step.1).try_into() {
                Err(_) => { continue; },
                Ok(col) => col
            };
            if col == cols { continue; }
            if *board.get(row, col).unwrap() == '#' { continue; }
            if *dists.get(row, col).unwrap() != 0 { continue; }
            queue.insert((dist + 1, (row, col)));
        }
    }
    let _is_found = is_found;
    
    println!("{}", result);
}
