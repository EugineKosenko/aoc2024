use std::{fs, env, io::{self, BufRead}};

#[derive(enum_iterator::Sequence, Clone, Ord, Eq, PartialEq, PartialOrd)]
enum Dir { Up, Right, Down, Left }

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut grid = grid::Grid::new(0, 0);
    for line in lines {
        grid.push_row(line.unwrap().chars().collect());
    }
    let ((mut row, mut col), _) = grid.indexed_iter()
        .find(|((row, col), _)| *grid.get(*row, *col).unwrap() == '^').unwrap();
    *grid.get_mut(row, col).unwrap() = 'X';
    let mut dir = enum_iterator::all::<Dir>().cycle().peekable();
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
    }
    let result = grid.iter()
        .filter(|&&pos| pos == 'X')
        .count();
    println!("{}", result);
}
