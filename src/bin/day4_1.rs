use std::{fs, env, io::{self, BufRead}};



fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines().collect::<Vec<_>>();
    let mut grid = grid::Grid::new(0, 0);
    for line in lines {
        grid.push_row(line.unwrap().chars().collect());
    }
    println!("{:#?}", grid);
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            for dr in -1..=1  {
                for dc in -1..=1 {
                    if (dr, dc) != (0, 0) {
                        let mut is_proper = true;
                        {
                            let mut r = r as isize;
                            let mut c = c as isize;
                            for s in "XMAS".chars() {
                                if r < 0 { is_proper = false; break; }
                                if c < 0 { is_proper = false; break; }
                                if r as usize == grid.rows() { is_proper = false; break; }
                                if c as usize == grid.cols() { is_proper = false; break; }
                        
                                if *grid.get(r as usize, c as usize).unwrap() != s { is_proper = false; break; }
                        
                                r += dr;
                                c += dc;
                            }
                        }
                        if is_proper { result += 1 }
                    }
                }
            }
        }
    }
    println!("{}", result);
}
