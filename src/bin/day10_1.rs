use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeSet;


fn path_ends(grid: &grid::Grid<u32>, row: usize, col: usize, val: u32) -> BTreeSet<(usize, usize)> {
    let mut result = BTreeSet::new();
    if val == 9 {
        result.insert((row, col));
        return result;
    }

    if row > 0 && *grid.get(row - 1, col).unwrap() == val + 1 {
        result = result.union(&path_ends(grid, row - 1, col, val + 1)).cloned().collect();
    }
    if row < grid.rows() - 1 && *grid.get(row + 1, col).unwrap() == val + 1 {
        result = result.union(&path_ends(grid, row + 1, col, val + 1)).cloned().collect();
    }
    if col > 0 && *grid.get(row, col - 1).unwrap() == val + 1 {
        result = result.union(&path_ends(grid, row, col - 1, val + 1)).cloned().collect();
    }
    if col < grid.cols() - 1 && *grid.get(row, col + 1).unwrap() == val + 1 {
        result = result.union(&path_ends(grid, row, col + 1, val + 1)).cloned().collect();
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut grid = grid::Grid::new(0, 0);
    for line in lines {
        grid.push_row(line.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect());
    }
    let result = grid.indexed_iter()
        .map(|((row, col), &val)| if val == 0 {
            let ends = path_ends(&grid, row, col, 0);
            ends.len()
        } else { 0 })
        .sum::<usize>();
    println!("{}", result);
}
