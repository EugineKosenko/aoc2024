use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeSet;
use std::collections::BTreeMap;

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
    let mut path = BTreeSet::new();
    let mut point = start;
    while point != finish {
        path.insert(point);
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
    }
    let fair_total = path.len();
    println!("{}", fair_total);
    let mut brdowns = BTreeSet::new();
    for start in path {
        let mut dists = grid::Grid::init(board.rows(), board.cols(), 0);
        let mut queue = STEPS.iter()
            .filter_map(|&step| {
                next(start, step, &board)
                    .and_then(|next| {
                        if next.0 > 0 && next.0 < board.rows() - 1
                            && next.1 > 0 && next.1 < board.cols() - 1
                            && *board.get(next.0, next.1).unwrap() == '#' {
                                Some((1, next))
                            } else {
                                None
                            }
                    })
            })
            .collect::<BTreeSet<_>>();
        while let Some((dist, point)) = queue.pop_first() {
            *dists.get_mut(point.0, point.1).unwrap() = dist;
            for step in STEPS {
                if let Some(next) = next(point, step, &board) {
                    if next.0 == 0 || next.0 == board.rows() - 1
                        || next.1 == 0 || next.1 == board.cols() - 1 { continue; }
                    if *board.get(next.0, next.1).unwrap() == '#' {
                        if dist < limit && *dists.get(next.0, next.1).unwrap() == 0 { queue.insert((dist + 1, next)); }
                    } else if next != start {
                        if *dists.get_mut(next.0, next.1).unwrap() == 0 {
                            *dists.get_mut(next.0, next.1).unwrap() = dist + 1;
                            brdowns.insert((start, next, dist));
                        }
                    }
                }
            }
        }
    }
    println!("{}", brdowns.len());
    //println!("{:?}", brdowns);
    let mut map = BTreeMap::<usize, usize>::new();
    let mut i = 0;
    for (from, to, jump) in brdowns {
        i += 1;
        if i % 10 == 0 { println!("{}", i); }
        if from == (1, 7) {
            //println!("{:?} {}", to, jump);
        }
        let mut point = start;
        let mut path = BTreeSet::from([point]);
        while point != from {
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
            path.insert(point);
        }
        if path.contains(&to) { continue; }
        let cheat_total = if to == finish {
            path.len() + jump
        } else {
            path.insert(to);
            let mut paths = STEPS.iter()
                .filter_map(|&step| {
                    next(to, step, &board)
                        .and_then(|next| {
                            if *board.get(next.0, next.1).unwrap() != '#' && !path.contains(&next) {
                                Some((next, path.clone()))
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<_>>();
            while !paths.is_empty() && !paths.iter().any(|&(point, _)| point == finish) {
                paths = paths.iter_mut()
                    .filter_map(|(point, ref mut path)| {
                        STEPS.iter()
                            .find_map(|&step| {
                                next(*point, step, &board)
                                    .and_then(|next| {
                                        if *board.get(next.0, next.1).unwrap() != '#' && !path.contains(&next) {
                                            Some(next)
                                        } else {
                                            None
                                        }
                                    })
                            })
                            .map(|next| {
                                path.insert(*point);
                                (next, path.to_owned())
                            })
                    })
                    .collect();
            }
            paths.iter().find(|(point, _)| *point == finish)
                .map(|(point, path)| {
                    if from == (1, 7) {
                        //println!("{:?} {}", point, path.len() + jump);
                    }
                    path.len() + jump
                })
                .unwrap_or(usize::MAX)
        };
        //println!("{:?} {}", (from, to, jump), cheat_total);
        if cheat_total <= fair_total - delta {
            *map.entry(fair_total - cheat_total).or_default() += 1;
            result += 1;
        }
    }
    println!("{:?}", map);
    println!("{}", result);
}
