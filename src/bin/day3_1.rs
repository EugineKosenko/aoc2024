use std::{fs, env, io::{self, BufRead}};



fn main() {
    let mut result = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let lines = io::BufReader::new(file).lines().collect::<Vec<_>>();
    for line in lines {
        let mut line = line.unwrap();
        lazy_static::lazy_static! {
            static ref RE_CODE: regex::Regex = regex::Regex::new(r"^mul\((?P<n1>\d{1,3}),(?P<n2>\d{1,3})\)(?P<rest>.*)$").unwrap();
        }
        while !line.is_empty() {
            match RE_CODE.captures(&line) {
                None => { line = line[1..].to_string(); },
                Some(cps) => {
                    let n1 = cps.name("n1").unwrap().as_str().parse::<usize>().unwrap();
                    let n2 = cps.name("n2").unwrap().as_str().parse::<usize>().unwrap();
                    line = cps.name("rest").unwrap().as_str().to_owned();
                    result += n1 * n2;
                }
            }
        }
    }
    println!("{}", result);
}
