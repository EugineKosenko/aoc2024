use std::{fs, env, io::{self, BufRead}};





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
        let x_a = cps.name("x").unwrap().as_str().parse::<usize>().unwrap();
        let y_a = cps.name("y").unwrap().as_str().parse::<usize>().unwrap();
        let line = lines.next().unwrap();
        let cps = RE_BUTTON.captures(&line).unwrap();
        let x_b = cps.name("x").unwrap().as_str().parse::<usize>().unwrap();
        let y_b = cps.name("y").unwrap().as_str().parse::<usize>().unwrap();
        let line = lines.next().unwrap();
        let cps = RE_PRIZE.captures(&line).unwrap();
        let x = cps.name("x").unwrap().as_str().parse::<usize>().unwrap();
        let y = cps.name("y").unwrap().as_str().parse::<usize>().unwrap();
        lines.next();
        let mut sol = usize::MAX;
        for m in 0..=(x/x_a) {
            for n in 0..=((x - x_a*m)/x_b) {
                if x_a * m + x_b * n == x
                    && y_a * m + y_b * n == y
                    && 3 * m + n < sol {
                            sol = 3 * m + n;
                    }
            }
        }
        if sol < usize::MAX { result += sol; }
    }
    println!("{}", result);
}
