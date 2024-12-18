use std::{fs, env, io::{self, BufRead}};
use itertools::Itertools;




fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let width = args[2].parse::<usize>().unwrap();
    let height = args[3].parse::<usize>().unwrap();
    lazy_static::lazy_static! {
        static ref RE_ROBOT: regex::Regex = regex::Regex::new(r"^p=(?P<px>-?\d+),(?P<py>-?\d+) v=(?P<vx>-?\d+),(?P<vy>-?\d+)$").unwrap();
    }
    let mut robots = vec![];
    for line in lines {
        let cps = RE_ROBOT.captures(&line).unwrap();
        let px = cps.name("px").unwrap().as_str().parse::<isize>().unwrap();
        let py = cps.name("py").unwrap().as_str().parse::<isize>().unwrap();
        let vx = cps.name("vx").unwrap().as_str().parse::<isize>().unwrap();
        let vy = cps.name("vy").unwrap().as_str().parse::<isize>().unwrap();
    
        robots.push(((px, py), (vx, vy)));
    }
    
    let mut board = grid::Grid::init(height, width, 0);
    for &((px, py), _) in &robots {
        *board.get_mut(py, px).unwrap() += 1;
    }
    
    for i in 0..height {
        for j in 0..width {
            print!("{}", match *board.get(i, j).unwrap() {
                0 => ".".to_string(),
                v => v.to_string()
            });
        }
        println!();
    }
    println!("{:?}", robots);
    for i in 0.. {
        if robots.iter()
            .map(|(pos, _)| pos)
            .all_unique() {
                result = i;
                break;
            }
    
        for robot in robots.iter_mut() {
            let px = (robot.0.0 + robot.1.0) % width as isize;
            let px = if px < 0 { px + width as isize } else { px };
            let py = (robot.0.1 + robot.1.1) % height as isize;
            let py = if py < 0 { py + height as isize } else { py };
            robot.0 = (px, py);
        }
    }
    
    let mut board = grid::Grid::init(height, width, 0);
    for &((px, py), _) in &robots {
        *board.get_mut(py, px).unwrap() += 1;
    }
    
    for i in 0..height {
        for j in 0..width {
            print!("{}", match *board.get(i, j).unwrap() {
                0 => ".".to_string(),
                v => v.to_string()
            });
        }
        println!();
    }
    println!("{}", result);
}
