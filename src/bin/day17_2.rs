use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeSet;

fn decode(op: usize, ra: usize, rb: usize, rc: usize) -> usize {
    match op {
        op @ 0..=3 => op,
        4 => ra,
        5 => rb,
        6 => rc,
        op => panic!("Unexpected combo operand {}", op)
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    lazy_static::lazy_static! {
        static ref RE_REGISTER: regex::Regex = regex::Regex::new(r"^Register [ABC]: (?P<value>\d+)$").unwrap();
        static ref RE_PROGRAM: regex::Regex = regex::Regex::new(r"^Program: (?P<program>[\d,]+)$").unwrap();
    }
    let line = lines.next().unwrap();
    let cps = RE_REGISTER.captures(&line).unwrap();
    let ra = cps.name("value").unwrap().as_str().parse::<usize>().unwrap();
    let line = lines.next().unwrap();
    let cps = RE_REGISTER.captures(&line).unwrap();
    let rb = cps.name("value").unwrap().as_str().parse::<usize>().unwrap();
    let line = lines.next().unwrap();
    let cps = RE_REGISTER.captures(&line).unwrap();
    let rc = cps.name("value").unwrap().as_str().parse::<usize>().unwrap();
    lines.next().unwrap();
    let line = lines.next().unwrap();
    let cps = RE_PROGRAM.captures(&line).unwrap();
    let program = cps.name("program").unwrap().as_str()
        .split(',')
        .map(|command| command.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut cnds = BTreeSet::from([0]);
    for n in 1..program.len() {
        cnds = cnds.iter().map(|cnd| cnd % (1 << 3 * n)).collect();
        cnds = cnds.iter()
            .flat_map(|c| {
                (0..1024)
                    .map(move |i| c + (i << 3 * (n - 1)))
                    .filter_map(|ta| {
                        let ra = ta;
                        let (mut ra, mut rb, mut rc) = (ra, rb, rc);
                        let mut ip = 0;
                        let mut out = vec![];
                        while ip < program.len() {
                            match program[ip] {
                                0 => {
                                    let op = decode(program[ip + 1], ra, rb, rc);
                                    ra /= 1 << op;
                                    ip += 2;
                                },
                                1 => {
                                    let op = program[ip + 1];
                                    rb ^= op;
                                    ip += 2;
                                }, 
                                2 => {
                                    let op = decode(program[ip + 1], ra, rb, rc);
                                    rb = op % 8;
                                    ip += 2;
                                },
                                3 => {
                                    let op = program[ip + 1];
                                    if ra == 0 { ip += 2 } else { ip = op; }
                                },
                                4 => {
                                    rb ^= rc;
                                    ip += 2;
                                },
                                5 => {
                                    let op = decode(program[ip + 1], ra, rb, rc);
                                    out.push(op % 8);
                                    ip += 2;
                                },
                                6 => {
                                    let op = decode(program[ip + 1], ra, rb, rc);
                                    rb = ra / (1 << op);
                                    ip += 2;
                                },
                                7 => {
                                    let op = decode(program[ip + 1], ra, rb, rc);
                                    rc = ra / (1 << op);
                                    ip += 2;
                                }, 
                                c => panic!("Unexpected command {}", c)
                            }
                        }
                        if n <= out.len() && out[0..n] == program[0..n] { Some(ta) } else { None }
                    })
            })
            .collect();
    }
    let result = cnds.into_iter()
        .filter_map(|ta| {
            let ra = ta;
            let (mut ra, mut rb, mut rc) = (ra, rb, rc);
            let mut ip = 0;
            let mut out = vec![];
            while ip < program.len() {
                match program[ip] {
                    0 => {
                        let op = decode(program[ip + 1], ra, rb, rc);
                        ra /= 1 << op;
                        ip += 2;
                    },
                    1 => {
                        let op = program[ip + 1];
                        rb ^= op;
                        ip += 2;
                    }, 
                    2 => {
                        let op = decode(program[ip + 1], ra, rb, rc);
                        rb = op % 8;
                        ip += 2;
                    },
                    3 => {
                        let op = program[ip + 1];
                        if ra == 0 { ip += 2 } else { ip = op; }
                    },
                    4 => {
                        rb ^= rc;
                        ip += 2;
                    },
                    5 => {
                        let op = decode(program[ip + 1], ra, rb, rc);
                        out.push(op % 8);
                        ip += 2;
                    },
                    6 => {
                        let op = decode(program[ip + 1], ra, rb, rc);
                        rb = ra / (1 << op);
                        ip += 2;
                    },
                    7 => {
                        let op = decode(program[ip + 1], ra, rb, rc);
                        rc = ra / (1 << op);
                        ip += 2;
                    }, 
                    c => panic!("Unexpected command {}", c)
                }
            }
            if out == program { Some(ta) } else { None }
        })
        .next().unwrap();
    println!("{}", result);
}
