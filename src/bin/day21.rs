use std::{fs, env, io::{self, BufRead}};
use std::{cmp::Ordering, iter};
use std::collections::BTreeMap;

type Point = (isize, isize);
fn from_num(c: char) -> Point {
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

fn from_cmd(c: char) -> Point {
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
type Snippet = Vec<Point>;
type MovesMemo = BTreeMap<(Point, Point, Point), Vec<Snippet>>;
fn moves(from: Point, to: Point, gap: Point, moves_memo: &mut MovesMemo) -> Vec<Snippet> {
    if let Some(result) = moves_memo.get(&(from, to, gap)) { return result.to_vec(); }
    let mut result = vec![];
    if from == to { return vec![vec![from_cmd('A')]]; }
    let (dr, dc) = (to.0 - from.0, to.1 - from.1);
    if let Some((d, point)) = match dr.cmp(&0) {
        Ordering::Greater => Some((1, from_cmd('v'))),
        Ordering::Less => Some((-1, from_cmd('^'))),
        Ordering::Equal => None
    } {
        if (from.0 + d, from.1) != gap {
            result.extend(
                moves((from.0 + d, from.1), to, gap, moves_memo)
                    .into_iter()
                    .map(|move_| iter::once(point).chain(move_.into_iter()).collect()));
        }
    }
    if let Some((d, point)) = match dc.cmp(&0) {
        Ordering::Greater => Some((1, from_cmd('>'))),
        Ordering::Less => Some((-1, from_cmd('<'))),
        Ordering::Equal => None
    } {
        if (from.0, from.1 + d) != gap {
            result.extend(
                moves((from.0, from.1 + d), to, gap, moves_memo)
                    .into_iter()
                    .map(|move_| iter::once(point).chain(move_.into_iter()).collect()));
        }
    }
    moves_memo.insert((from, to, gap), result.clone());
    result
}
type Rank = usize;
type FindMemo = BTreeMap<(Snippet, Point, Rank), usize>;
fn find_len(
    snippet: Snippet, gap: Point, rank: Rank,
    moves_memo: &mut MovesMemo, find_memo: &mut FindMemo) -> usize {
    if let Some(result) = find_memo.get(&(snippet.clone(), gap, rank)) { return *result; }
    if rank == 0 { return snippet.len() - 1; }
    let result = (0..snippet.len() - 1)
        .map(|i| {
            moves(snippet[i], snippet[i+1], gap, moves_memo)
                .into_iter()
                .map(|move_| find_len(iter::once(from_cmd('A')).chain(move_.into_iter()).collect(), CMD_GAP, rank - 1, moves_memo, find_memo))
                .min()
                .unwrap()
        })
        .sum();
    find_memo.insert((snippet, gap, rank), result);
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let rank = args[2].parse::<Rank>().unwrap();
    let mut result = 0;
    let mut moves_memo: MovesMemo = BTreeMap::new();
    let mut find_memo = BTreeMap::new();
    for line in lines {
        let snippet: Snippet = iter::once(from_num('A')).chain(line.chars().map(from_num)).collect();
        let len = find_len(snippet, NUM_GAP, rank, &mut moves_memo, &mut find_memo);
        result += len * line[0..line.len()-1].parse::<usize>().unwrap();
    }
    println!("{}", result);
}
