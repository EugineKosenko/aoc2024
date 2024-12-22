use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

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
fn find_brdowns(board: &Board, limit: usize,
                point: Point, path: &BTreeSet<Point>,
                brdowns: &mut BTreeSet<(Point, usize)>) {
    if path.len() == limit { return; }
    let mut path = path.clone();
    path.insert(point);
    for step in STEPS {
        if let Some(next) = next(point, step, board) {
            if next.0 > 0 && next.0 < board.rows() - 1
                && next.1 > 0 && next.1 < board.cols() - 1 {
                    if *board.get(next.0, next.1).unwrap() == '#' {
                        if !path.contains(&next) {
                            find_brdowns(board, limit, next, &path, brdowns);
                        }
                    } else {
                        brdowns.insert((next, path.len() + 1));
                    }
                }
        }
    }
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
    let mut map = BTreeMap::<usize, usize>::new();
    for (n, start) in path.iter().enumerate() {
        if *start != finish {
            let mut brdowns = BTreeSet::new();
            for step in STEPS {
                if let Some(next) = next(*start, step, &board) {
                    if next.0 > 0 && next.0 < board.rows() - 1
                        && next.1 > 0 && next.1 < board.cols() - 1
                        && *board.get(next.0, next.1).unwrap() == '#' {
                            find_brdowns(&board, limit, next, &BTreeSet::new(), &mut brdowns);
                        }
                }
            }
            //println!("{:?} {:?}", start, brdowns);
            for (finish, dist) in brdowns {
                if let Some(m) = path.iter().skip(n + 1).position(|&point| point == finish) {
                    let m = m + 1;
                    if dist + delta <= m {
                        //println!("{:?}", (start, finish, dist));
                        *map.entry(m - dist).or_default() += 1;
                        result += 1;
                    }
                };
            }
        }
    }
    println!("{:?}", map);
    println!("{}", result);
}
