use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeSet;

#[derive(enum_iterator::Sequence, Clone, Ord, Eq, PartialEq, PartialOrd)]
enum Dir { Up, Right, Down, Left }

fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut grid = grid::Grid::new(0, 0);
    for line in lines {
        grid.push_row(line.unwrap().chars().collect());
    }
    let ((srow, scol), _) = grid.indexed_iter()
        .find(|((row, col), _)| *grid.get(*row, *col).unwrap() == '^').unwrap();
    *grid.get_mut(srow, scol).unwrap() = 'X';
    for orow in 0..grid.rows() {
        for ocol in 0..grid.cols() {
            if *grid.get(orow, ocol).unwrap() == '.' {
                let mut grid = grid.clone();
                *grid.get_mut(orow, ocol).unwrap() = '#';
                let (mut row, mut col) = (srow, scol);
                let mut dir = enum_iterator::all::<Dir>().cycle().peekable();
                let mut pos = BTreeSet::new();
                pos.insert(((row, col), dir.peek().unwrap().clone()));
                loop {
                    let (nrow, ncol) = match dir.peek().unwrap() {
                        Dir::Up => { if row == 0 { break; } else { (row - 1, col) } },
                        Dir::Right => { if col == grid.cols() - 1 { break; } else { (row, col + 1) } },
                        Dir::Down => { if row == grid.rows() - 1 { break; } else { (row + 1, col) } },
                        Dir::Left => { if col == 0 { break; } else { (row, col - 1) } }
                    };
                    if *grid.get(nrow, ncol).unwrap() == '#' {
                        dir.next();
                    } else {
                        (row, col) = (nrow, ncol);
                        *grid.get_mut(row, col).unwrap() = 'X';
                    }
                    if pos.contains(&((row, col), dir.peek().unwrap().clone())) {
                        result += 1;
                        break;
                    } else {
                        pos.insert(((row, col), dir.peek().unwrap().clone()));
                    }
                }
            }
        }
    }
    println!("{}", result);
}
