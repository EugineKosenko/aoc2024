use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeMap;


fn size(value: usize, count: usize, cache: &mut BTreeMap<(usize, usize), usize>) -> usize {
    cache.get(&(value, count))
        .copied()
        .unwrap_or_else(|| {
            let result = if count == 0 {
                1
            } else if value == 0 {
                size(1, count - 1, cache)
            } else {
                let len = (value as f32).log10().floor() as u32 + 1;
                if len % 2 == 0 {
                    let p = 10_usize.pow(len / 2);
                    size(value / p, count - 1, cache) + size(value % p, count - 1, cache)
                } else {
                    size(value.checked_mul(2024).unwrap(), count - 1, cache)
                }
            };
            cache.insert((value, count), result);
            result
        })
}


fn main() {
    let mut result: usize = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let values = io::BufReader::new(file)
        .lines().next().unwrap().unwrap()
        .split(' ')
        .map(|value| value.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut cache = BTreeMap::new();
    for value in values {
        result += size(value, 25, &mut cache);
    }
    println!("{}", result);
}
