use std::{fs, env, io::{self, BufRead}};
use ndarray::prelude::*;
use ndarray_linalg::Solve;




fn main() {
    let mut result: usize = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut lines = lines.peekable();
    while lines.peek().is_some() {
        lazy_static::lazy_static! {
            static ref RE_BUTTON: regex::Regex = regex::Regex::new(r"^Button [AB]: X\+(?P<x>\d+), Y\+(?P<y>\d+)$").unwrap();
            static ref RE_PRIZE: regex::Regex = regex::Regex::new(r"^Prize: X=(?P<x>\d+), Y=(?P<y>\d+)$").unwrap();
        }
        let line = lines.next().unwrap();
        let cps = RE_BUTTON.captures(&line).unwrap();
        let x_a = cps.name("x").unwrap().as_str().parse::<usize>().unwrap() as f64;
        let y_a = cps.name("y").unwrap().as_str().parse::<usize>().unwrap() as f64;
        let line = lines.next().unwrap();
        let cps = RE_BUTTON.captures(&line).unwrap();
        let x_b = cps.name("x").unwrap().as_str().parse::<usize>().unwrap() as f64;
        let y_b = cps.name("y").unwrap().as_str().parse::<usize>().unwrap() as f64;
        let line = lines.next().unwrap();
        let cps = RE_PRIZE.captures(&line).unwrap();
        let x = (cps.name("x").unwrap().as_str().parse::<usize>().unwrap() + 10_000_000_000_000) as f64;
        let y = (cps.name("y").unwrap().as_str().parse::<usize>().unwrap() + 10_000_000_000_000) as f64;
        lines.next();
        let a: Array2<f64> = array![[x_a, x_b], [y_a, y_b]];
        let b: Array1<f64> = array![x, y];
        let sol = a.solve_into(b).unwrap().to_vec();
        let m = sol[0].round();
        let n = sol[1].round();
        if (m as f64 - sol[0]).abs() < 1e-4 && (n as f64 - sol[1]).abs() < 1e-4 {
            result += 3 * (m as usize) + (n as usize);
        }
    }
    println!("{}", result);
}
