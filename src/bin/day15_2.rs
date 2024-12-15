use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeSet;


fn push(board: &mut grid::Grid<char>, front: BTreeSet<(usize, usize)>, delta: isize) -> bool {
    let row: usize = (front.first().unwrap().0 as isize + delta).try_into().unwrap();
    let mut nfront = BTreeSet::new();
    for &(_, col) in &front {
        match *board.get(row, col).unwrap() {
            '#' => { return false; },
            '.' => { /* nothing */ },
            c @ '[' | c @ ']' => {
                nfront.insert((row, col));
                nfront.insert((row, match c {
                    '[' => col + 1,
                    ']' => col - 1,
                    c => panic!("Unexpected block {}", c)
                }
                ));
            },
            c => panic!("Unexpected block {}", c)
        }
    }
    if nfront.is_empty() || push(board, nfront, delta) {
        for (r, col) in front {
            *board.get_mut(row, col).unwrap() = *board.get(r, col).unwrap();
            *board.get_mut(r, col).unwrap() = '.';
        }
        true
    } else {
        false
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
        board.push_row(
            line.chars()
                .flat_map(|ch| match ch {
                    '#' => "##",
                    'O' => "[]",
                    '.' => "..",
                    '@' => "@.",
                    ch => panic!("Unexpected point {}", ch)
                }.chars())
                .collect());
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
            match command {
                '^' | 'v' => {
                    let delta = match command {
                        '^' => -1,
                        'v' => 1,
                        c => panic!("Unexpected command {}", c)
                    };
                    let row = (point.0 as isize + delta).try_into().unwrap();
                    match *board.get(row, point.1).unwrap() {
                        '#' => { /* стінка */ },
                        '.' => {
                            *board.get_mut(point.0, point.1).unwrap() = '.';
                            point.0 = row;
                            *board.get_mut(point.0, point.1).unwrap() = '@';
                        },
                        '[' | ']' => {
                            let mut front = BTreeSet::new();
                            front.insert((row, point.1));
                            front.insert((row, match *board.get(row, point.1).unwrap() {
                                '[' => point.1 + 1,
                                ']' => point.1 - 1,
                                c => panic!("Unexpected block {}", c)
                            }));
                            if push(&mut board, front, delta) {
                                *board.get_mut(point.0, point.1).unwrap() = '.';
                                point.0 = row;
                                *board.get_mut(point.0, point.1).unwrap() = '@';
                            }
                        },
                        c => panic!("Unexpected block {}", c)
                    }
                },
                '>' => {
                    for i in (point.1+1)..board.cols() {
                        let ch = *board.get(point.0, i).unwrap();
                        if ch == '#' { break; } // стінка
                        if ch == '.' { // вільна точка
                            for j in ((point.1+2)..i).step_by(2) {
                                *board.get_mut(point.0, j).unwrap() = '[';
                                *board.get_mut(point.0, j + 1).unwrap() = ']';
                            }
                            *board.get_mut(point.0, point.1).unwrap() = '.';
                            point.1 += 1;
                            *board.get_mut(point.0, point.1).unwrap() = '@';
                            break;
                        }
                    }
                },
                '<' => {
                    for i in (0..point.1).rev() {
                        let ch = *board.get(point.0, i).unwrap();
                        if ch == '#' { break; } // стінка
                        if ch == '.' { // вільна точка
                            for j in (i..point.1).step_by(2) {
                                *board.get_mut(point.0, j).unwrap() = '[';
                                *board.get_mut(point.0, j + 1).unwrap() = ']';
                            }
                            *board.get_mut(point.0, point.1).unwrap() = '.';
                            point.1 -= 1;
                            *board.get_mut(point.0, point.1).unwrap() = '@';
                            break;
                        }
                    }
                },
                c => panic!("Unexpected command {}", c)
            }
        }
    }
    for ((row, col), ch) in board.indexed_iter() {
        if *ch == '[' {
            result += 100 * row + col;
        }
    }
    println!("{}", result);
}
