use std::{fs, env, io::{self, BufRead}};


fn path_counts(grid: &grid::Grid<u32>, row: usize, col: usize, val: u32) -> usize {
    if val == 9 { return 1; }

    let mut result = 0;
    if row > 0 && *grid.get(row - 1, col).unwrap() == val + 1 {
        result += path_counts(grid, row - 1, col, val + 1);
    }
    if row < grid.rows() - 1 && *grid.get(row + 1, col).unwrap() == val + 1 {
        result += path_counts(grid, row + 1, col, val + 1);
    }
    if col > 0 && *grid.get(row, col - 1).unwrap() == val + 1 {
        result += path_counts(grid, row, col - 1, val + 1);
    }
    if col < grid.cols() - 1 && *grid.get(row, col + 1).unwrap() == val + 1 {
        result += path_counts(grid, row, col + 1, val + 1);
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
            path_counts(&grid, row, col, 0)
        } else { 0 })
        .sum::<usize>();
    println!("{}", result);
}
