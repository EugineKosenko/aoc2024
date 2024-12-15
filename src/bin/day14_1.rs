use std::{fs, env, io::{self, BufRead}};





fn main() {
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
    let mut board = grid::Grid::init(height, width, 0);
    let (mut c1, mut c2, mut c3, mut c4) = (0, 0, 0, 0);
    for line in lines {
        let cps = RE_ROBOT.captures(&line).unwrap();
        let px = cps.name("px").unwrap().as_str().parse::<isize>().unwrap();
        let py = cps.name("py").unwrap().as_str().parse::<isize>().unwrap();
        let vx = cps.name("vx").unwrap().as_str().parse::<isize>().unwrap();
        let vy = cps.name("vy").unwrap().as_str().parse::<isize>().unwrap();
    
        let px = (px + 100 * vx) % width as isize;
        let px = (if px < 0 { px + width as isize } else { px }) as usize;
        let py = (py + 100 * vy) % height as isize;
        let py = (if py < 0 { py + height as isize } else { py }) as usize;
        *board.get_mut(py, px).unwrap() += 1;
        let (cx, cy) = ((width - 1) / 2, (height - 1) / 2);
        if px < cx && py < cy { c1 += 1; }
        if px < cx && py > cy { c2 += 1; }
        if px > cx && py < cy { c3 += 1; }
        if px > cx && py > cy { c4 += 1; }
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
    
    let result = c1 * c2 * c3 * c4;
    println!("{}", result);
}
