use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeSet;





fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut board = grid::Grid::new(0, 0);
    for line in lines {
        board.push_row(line.chars().collect());
    }
    let mut start = (0, 0);
    for (pt, &ch) in board.indexed_iter() {
        if ch == 'S' {
            start = pt;
            break;
        }
    }
    
    let mut finish = (0, 0);
    for (pt, &ch) in board.indexed_iter() {
        if ch == 'E' {
            finish = pt;
            break;
        }
    }
    let mut result = 0;
    let mut walls = BTreeSet::new();
    let mut dists = grid::Grid::init(board.rows(), board.cols(), 0);
    let mut queue = BTreeSet::from([(1, start)]);
    let mut total = 0;
    while let Some((dist, point)) = queue.pop_first() {
        if point == finish { total = dist - 1; break; }
        *dists.get_mut(point.0, point.1).unwrap() = dist;
        for step in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let row: usize = match (point.0 as isize + step.0).try_into() {
                Err(_) => { continue; },
                Ok(row) => row
            };
            if row == board.rows() { continue; }
            let col: usize = match (point.1 as isize + step.1).try_into() {
                Err(_) => { continue; },
                Ok(col) => col
            };
            if col == board.cols() { continue; }
            if *board.get(row, col).unwrap() == '#' {
                if row > 0 && row < board.rows() - 1
                    && col > 0 && col < board.cols() - 1 {
                        walls.insert((row, col));
                    }
                continue;
            }
            if *dists.get(row, col).unwrap() != 0 { continue; }
            queue.insert((dist + 1, (row, col)));
        }
    }
    println!("{}", walls.len());
    for (row, col) in walls {
        *board.get_mut(row, col).unwrap() = '.';
        let mut dists = grid::Grid::init(board.rows(), board.cols(), 0);
        let mut queue = BTreeSet::from([(1, start)]);
        let mut cheat_total = 0;
        while let Some((dist, point)) = queue.pop_first() {
            if point == finish { cheat_total = dist - 1; break; }
            if dist > total - 100 { cheat_total = dist; break; }
            *dists.get_mut(point.0, point.1).unwrap() = dist;
            for step in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let row: usize = match (point.0 as isize + step.0).try_into() {
                    Err(_) => { continue; },
                    Ok(row) => row
                };
                if row == board.rows() { continue; }
                let col: usize = match (point.1 as isize + step.1).try_into() {
                    Err(_) => { continue; },
                    Ok(col) => col
                };
                if col == board.cols() { continue; }
                if *board.get(row, col).unwrap() == '#' { continue; }
                if *dists.get(row, col).unwrap() != 0 { continue; }
                queue.insert((dist + 1, (row, col)));
            }
        }
        if cheat_total <= total - 100 {
            result += 1;
        }
        *board.get_mut(row, col).unwrap() = '#';
    }
    println!("{}", result);
}
