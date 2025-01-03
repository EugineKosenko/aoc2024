use std::{fs, env, io::{self, BufRead}};

type Board = grid::Grid<char>;
type Point = (usize, usize);

fn is_inside(point: Point, board: &Board) -> bool {
    point.0 < board.rows() && point.1 < board.cols()
}
type Step = (isize, isize);

const STEPS: [Step; 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn next(point: Point, step: Step, board: &Board) -> Option<Point> {
    (point.0 as isize + step.0).try_into().ok()
        .and_then(|row| {
            (point.1 as isize + step.1).try_into().ok()
                .and_then(|col| {
                    if is_inside((row, col), board) { Some((row, col)) } else { None }
                })
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut board = Board::new(0, 0);
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
    let limit = args[2].parse::<usize>().unwrap();
    let delta = args[3].parse::<usize>().unwrap();
    let mut result = 0;
    let mut path = vec![start];
    let mut point = start;
    while point != finish {
        point = STEPS.iter()
            .find_map(|&step| {
                next(point, step, &board)
                    .and_then(|next| {
                        if *board.get(next.0, next.1).unwrap() != '#' && !path.contains(&next) {
                            Some(next)
                        } else {
                            None
                        }
                    })
            }).unwrap();
        path.push(point);
    }
    for n in 0..(path.len() - delta) {
        for m in (n + delta)..path.len() {
            let dist = ((path[n].0 as isize - path[m].0 as isize).abs()
                        + (path[n].1 as isize - path[m].1 as isize).abs()) as usize;
            if dist <= limit && dist + delta <= (m - n) { result += 1; }
        }
    }
    println!("{}", result);
}
