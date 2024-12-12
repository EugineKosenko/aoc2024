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
        let rbegin = region.iter().map(|(r, _)| *r).min().unwrap();
        let rend = region.iter().map(|(r, _)| *r).max().unwrap() + 1;
        let cbegin = region.iter().map(|(_, c)| *c).min().unwrap();
        let cend = region.iter().map(|(_, c)| *c).max().unwrap() + 1;
        let mut sides_count = 0;
        for row in rbegin..rend {
            let mut col = cbegin;
            while col < cend {
                while col < cend && !region.contains(&(row, col)) { col += 1 }
                while col < cend
                    && region.contains(&(row, col))
                    && (row > 0 && region.contains(&(row - 1, col))) { col += 1 }
                if col < cend
                    && region.contains(&(row, col))
                    && (row == 0 || !region.contains(&(row - 1, col))) {
                    sides_count += 1;
                    while col < cend
                        && region.contains(&(row, col))
                        && (row == 0 || !region.contains(&(row - 1, col))) { col += 1 }
                }
            }
        }
        for row in rbegin..rend {
            let mut col = cbegin;
            while col < cend {
                while col < cend && !region.contains(&(row, col)) { col += 1 }
                while col < cend
                    && region.contains(&(row, col))
                    && (row < rows - 1 && region.contains(&(row + 1, col))) { col += 1 }
                if col < cend
                    && region.contains(&(row, col))
                    && (row == rows - 1 || !region.contains(&(row + 1, col))) {
                    sides_count += 1;
                    while col < cend
                        && region.contains(&(row, col))
                        && (row == rows - 1 || !region.contains(&(row + 1, col))) { col += 1 }
                }
            }
        }
        for col in cbegin..cend {
            let mut row = rbegin;
            while row < rend {
                while row < rend && !region.contains(&(row, col)) { row += 1 }
                while row < rend
                    && region.contains(&(row, col))
                    && (col > 0 && region.contains(&(row, col - 1))) { row += 1 }
                if row < rend
                    && region.contains(&(row, col))
                    && (col == 0 || !region.contains(&(row, col - 1))) {
                    sides_count += 1;
                    while row < rend
                        && region.contains(&(row, col))
                        && (col == 0 || !region.contains(&(row, col - 1))) { row += 1 }
                }
            }
        }
        for col in cbegin..cend {
            let mut row = rbegin;
            while row < rend {
                while row < rend && !region.contains(&(row, col)) { row += 1 }
                while row < rend
                    && region.contains(&(row, col))
                    && (col < cols - 1 && region.contains(&(row, col + 1))) { row += 1 }
                if row < rend
                    && region.contains(&(row, col))
                    && (col == cols - 1 || !region.contains(&(row, col + 1))) {
                    sides_count += 1;
                    while row < rend
                        && region.contains(&(row, col))
                        && (col == cols - 1 || !region.contains(&(row, col + 1))) { row += 1 }
                }
            }
        }
        result += region.len() * sides_count;
    }
    println!("{}", result);
}
