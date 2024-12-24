use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeMap;


type Point = (isize, isize);
fn num(c: char) -> Point {
    match c {
        'A' => (3, 2),
        '0' => (3, 1),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        c => panic!("Unexpected num {}", c)
    }
}

fn cmd(c: char) -> Point {
    match c {
        'A' => (0, 2),
        '^' => (0, 1),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        c => panic!("Unexpected num {}", c)
    }
}
const NUM_GAP: Point = (3, 0);
const CMD_GAP: Point = (0, 0);
type Snippet = String;
fn expanded(start: Point, points: Vec<Point>, gap: Point, moves_memo: &mut BTreeMap<(Point, Point, Point), Vec<Snippet>>) -> Vec<Snippet> {
    let mut result = vec![String::new()];
    let mut prev = start;
    for point in points {
        let moves = moves(prev, point, gap, moves_memo);
        result = result.iter()
            .flat_map(|snippet| {
                moves.iter().map(|move_| snippet.chars().chain(move_.chars()).collect::<String>())
            })
            .collect::<Vec<Snippet>>();
        prev = point;
    }
    result
}
fn moves(from: Point, to: Point, gap: Point, moves_memo: &mut BTreeMap<(Point, Point, Point), Vec<Snippet>>) -> Vec<Snippet> {
    if let Some(result) = moves_memo.get(&(from, to, gap)) { return result.to_vec(); }
    let mut result = vec![];
    if from == to { return vec!["A".to_string()]; }
    let (dr, dc) = (to.0 - from.0, to.1 - from.1);
    use std::cmp::Ordering;
    if let Some((d, cmd)) = match dr.cmp(&0) {
        Ordering::Greater => Some((1, 'v')),
        Ordering::Less => Some((-1, '^')),
        Ordering::Equal => None
    } {
        if (from.0 + d, from.1) != gap {
            for move_ in moves((from.0 + d, from.1), to, gap, moves_memo).iter_mut() {
                move_.insert(0, cmd);
                result.push(move_.to_string());
            }
        }
    }
    if let Some((d, cmd)) = match dc.cmp(&0) {
        Ordering::Greater => Some((1, '>')),
        Ordering::Less => Some((-1, '<')),
        Ordering::Equal => None
    } {
        if (from.0, from.1 + d) != gap {
            for move_ in moves((from.0, from.1 + d), to, gap, moves_memo).iter_mut() {
                move_.insert(0, cmd);
                result.push(move_.to_string());
            }
        }
    }
    moves_memo.insert((from, to, gap), result.clone());
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut result = 0;
    let mut moves_memo = BTreeMap::new();
    for line in lines {
        let points = line.chars().map(num).collect::<Vec<_>>();
        let mut snippets = expanded(num('A'), points, NUM_GAP, &mut moves_memo);
        for i in 0..3 {
            snippets = snippets.iter()
                .map(|snippet| snippet.chars().map(cmd).collect())
                .map(|points| expanded(cmd('A'), points, CMD_GAP, &mut moves_memo))
                .flatten()
                .collect();
            println!("{} {}", i, snippets.len());
        }
        let len = snippets.iter().map(|snippet| snippet.len()).min().unwrap();
        let mult = line[0..line.len()-1].parse::<usize>().unwrap();
        println!("{} {}", mult, len);
        result += mult * len;
    }
    println!("{}", result);
}
