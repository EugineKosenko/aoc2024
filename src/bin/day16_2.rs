use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeMap;
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
    let mut weights = BTreeMap::new();
    weights.insert((start, (0, 1)), 0);
    let mut queue = BTreeSet::new();
    queue.insert((0, (start, (0, 1))));
    while let Some((weight, (point, dir))) = queue.pop_first() {
        if weight > *weights.entry((point, dir)).or_insert(usize::MAX) { continue; }
        for step in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let next = ((point.0 as isize + step.0).try_into().unwrap(),
                        (point.1 as isize + step.1).try_into().unwrap());
            let weight = weight + if step == dir { 1 } else { 1001 };
            if *board.get(next.0, next.1).unwrap() == '#'
                || *weights.entry((next, step)).or_insert(usize::MAX) <= weight {
                    continue;
                }
            *weights.get_mut(&(next, step)).unwrap() = weight;
            queue.insert((weight, (next, step)));
        }
    }
    let mut points = BTreeSet::from([finish]);
    let weight = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter()
        .map(|&step| *weights.entry((finish, step)).or_insert(usize::MAX))
        .min().unwrap();
    let mut queue = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter()
        .filter_map(|&step| {
            if *weights.entry((finish, step)).or_insert(usize::MAX) == weight {
                Some((((finish.0 as isize - step.0).try_into().unwrap(),
                       (finish.1 as isize - step.1).try_into().unwrap()),
                      step, weight))
            } else {
                None
            }
        })
        .collect::<Vec<((usize, usize), (isize, isize), usize)>>();
    while let Some((point, dir, weight)) = queue.pop() {
        points.insert(point);
        if point != start {
            for step in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let weight = weight - if step == dir { 1 } else { 1001 };
                if *weights.entry((point, step)).or_insert(usize::MAX) == weight {
                    let next = ((point.0 as isize - step.0).try_into().unwrap(),
                                (point.1 as isize - step.1).try_into().unwrap());
                    queue.push((next, step, weight));
                }
            }
        }
    }
    let result = points.len();
    println!("{}", result);
}
