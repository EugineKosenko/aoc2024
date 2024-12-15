use std::{fs, env, io::{self, BufRead}};



struct PointIter {
    point: (usize, usize),
    delta: (isize, isize)
}

impl PointIter {
    fn new(point: (usize, usize), delta: (isize, isize)) -> Self {
        PointIter { point, delta }
    }
}

impl Iterator for PointIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.point.0 = ((self.point.0 as isize) + self.delta.0).try_into().unwrap();
        self.point.1 = ((self.point.1 as isize) + self.delta.1).try_into().unwrap();
        Some(self.point)
    }
}

fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut board = grid::Grid::new(0, 0);
    while let Some(line) = lines.next() {
        if line.is_empty() { break; }
        board.push_row(line.chars().collect());
    }
    let mut point = (0, 0);
    for (pt, &ch) in board.indexed_iter() {
        if ch == '@' {
            point = pt;
            break;
        }
    }
    while let Some(line) = lines.next() {
        for command in line.chars() {
            let delta = match command {
                '^' => (-1, 0),
                '>' => (0, 1),
                'v' => (1, 0),
                '<' => (0, -1),
                c => panic!("Unexpected command {}", c)
            };    
            if let Some(empty_point) = PointIter::new(point, delta)
                .take_while(|(row, col)| *board.get(*row, *col).unwrap() != '#')
                .find(|(row, col)| *board.get(*row, *col).unwrap() == '.') {
                    *board.get_mut(empty_point.0, empty_point.1).unwrap() = 'O';
                    *board.get_mut(point.0, point.1).unwrap() = '.';
                    point.0 = ((point.0 as isize) + delta.0).try_into().unwrap();
                    point.1 = ((point.1 as isize) + delta.1).try_into().unwrap();
                    *board.get_mut(point.0, point.1).unwrap() = '@';
                }
        }
    }
    for ((row, col), ch) in board.indexed_iter() {
        if *ch == 'O' {
            result += 100 * row + col;
        }
    }
    println!("{}", result);
}
