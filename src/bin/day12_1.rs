use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeSet;


fn region(
    size @ (rows, cols): (usize, usize),
    (pos @ (row, col), type_): ((usize, usize), char),
    plants: &mut BTreeSet<((usize, usize), char)>
) -> Vec<(usize, usize)> {
    let mut result = vec![pos];
    if row > 0        { if let Some(plant) = plants.take(&((row - 1, col), type_)) { result.extend(region(size, plant, plants)); }}
    if col < cols - 1 { if let Some(plant) = plants.take(&((row, col + 1), type_)) { result.extend(region(size, plant, plants)); }}
    if row < rows - 1 { if let Some(plant) = plants.take(&((row + 1, col), type_)) { result.extend(region(size, plant, plants)); }}
    if col > 0        { if let Some(plant) = plants.take(&((row, col - 1), type_)) { result.extend(region(size, plant, plants)); }}
    result
}


fn main() {
    let mut result: usize = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut plants = BTreeSet::new();
    let (mut rows, mut cols) = (0, 0);
    for (row, line) in lines.enumerate() {
        rows += 1;
        cols = 0;
        for (col, ch) in line.chars().enumerate() {
            cols += 1;
            plants.insert(((row, col), ch));
        }
    }
    while let Some(plant) = plants.pop_first() {
        let region = region((rows, cols), plant, &mut plants);
        let perm = region.iter()
            .map(|(row, col)| {
                (if *row == 0          { 1 } else if region.contains(&(*row - 1, *col)) { 0 } else { 1 })
                + (if *col == cols - 1 { 1 } else if region.contains(&(*row, *col + 1)) { 0 } else { 1 })
                + (if *row == rows - 1 { 1 } else if region.contains(&(*row + 1, *col)) { 0 } else { 1 })
                + (if *col == 0        { 1 } else if region.contains(&(*row, *col - 1)) { 0 } else { 1 })
            })
            .sum::<usize>();
        result += perm * region.len();
    }
    println!("{}", result);
}
