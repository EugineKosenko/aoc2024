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
    println!("{:#?}", board);
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
    println!("{:?} {:?}", start, finish);
    let mut dist = grid::Grid::init(board.rows(), board.cols(), 0);
    let mut front = BTreeSet::new();
    *dist.get_mut(start.0, start.1).unwrap() = 1;
    front.insert((1, (0, 1), start));
    while let Some((d, dir, point)) = front.pop_first() {
        for delta in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let row = (point.0 as isize + delta.0).try_into().unwrap();
            let col = (point.1 as isize + delta.1).try_into().unwrap();
            if *board.get(row, col).unwrap() != '#' && *dist.get(row, col).unwrap() == 0 {
                if delta == dir {
                    *dist.get_mut(row, col).unwrap() = d + 1;
                    front.insert((d + 1, dir, (row, col)));
                } else {
                    *dist.get_mut(row, col).unwrap() = d + 1001;
                    front.insert((d + 1001, delta, (row, col)));
                }
            }
        }
    }
    println!("{:#?}", dist);
    let result = *dist.get(finish.0, finish.1).unwrap() - 1;
    println!("{}", result);
}
