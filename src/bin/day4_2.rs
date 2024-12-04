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
    for r in 1..grid.rows() - 1{
        for c in 1..grid.cols() - 1{
            if *grid.get(r, c).unwrap() == 'A' {
                let lt = *grid.get(r - 1, c - 1).unwrap();
                let rb = *grid.get(r + 1, c + 1).unwrap();
                let rt = *grid.get(r - 1, c + 1).unwrap();
                let lb = *grid.get(r + 1, c - 1).unwrap();
                
                if (lt == 'M' && rb == 'S' || lt == 'S' && rb == 'M')
                    && (rt == 'M' && lb == 'S' || rt == 'S' && lb == 'M') {
                        result += 1;
                    }
            }
        }
    }
    println!("{}", result);
}
