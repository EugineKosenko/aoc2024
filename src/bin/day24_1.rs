use std::{fs, env, io::{self, BufRead}};
use std::collections::BTreeMap;
#[derive(Debug)]
enum Func { AND, OR, XOR }

#[derive(Debug)]
struct Gate {
    input1: String,
    input2: String,
    output: String,
    func: Func
}

impl Gate {
    fn process(&self, wires: &mut BTreeMap<String, Option<usize>>) {
        if let Some(input1) = *wires.get(&self.input1).unwrap() {
            if let Some(input2) = *wires.get(&self.input2).unwrap() {
                *wires.get_mut(&self.output).unwrap() = Some(match self.func {
                    Func::AND => input1 & input2,
                    Func::OR => input1 | input2,
                    Func::XOR => input1 ^ input2
                });
            }
        }
    }
}





fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut wires = BTreeMap::new();
    lazy_static::lazy_static! {
        static ref RE_WIRE: regex::Regex = regex::Regex::new(r"^(?P<name>\w+): (?P<value>[01])$").unwrap();
    }
    for line in lines.by_ref() {
        if line.is_empty() { break; }
        let cps = RE_WIRE.captures(&line).unwrap();
        let name = cps.name("name").unwrap().as_str().to_string();
        let value = cps.name("value").unwrap().as_str().parse::<usize>().unwrap();
        wires.insert(name, Some(value));
    }
    let mut gates = Vec::new();
    lazy_static::lazy_static! {
        static ref RE_GATE: regex::Regex = regex::Regex::new(r"^(?P<input1>\w+) (?P<func>AND|OR|XOR) (?P<input2>\w+) -> (?P<output>\w+)$").unwrap();
    }
    for line in lines {
        let cps = RE_GATE.captures(&line).unwrap();
        let func = match cps.name("func").unwrap().as_str() {
            "AND" => Func::AND,
            "OR" => Func::OR,
            "XOR" => Func::XOR,
            f => panic!("Unexpected func {}", f)
        };
        let input1 = cps.name("input1").unwrap().as_str().to_string();
        wires.entry(input1.clone()).or_default();
        let input2 = cps.name("input2").unwrap().as_str().to_string();
        wires.entry(input2.clone()).or_default();
        let output = cps.name("output").unwrap().as_str().to_string();
        wires.entry(output.clone()).or_default();
        gates.push(Gate { input1, input2, output, func });
    }
    while wires.iter()
        .filter(|(name, _)| name.starts_with('z'))
        .any(|(_, value)| value.is_none()) {
            for gate in gates.iter() {
                gate.process(&mut wires);
            }
        }
    let result = wires.iter()
        .filter(|(name, _)| name.starts_with('z'))
        .rev()
        .map(|(_, value)| value.unwrap())
        .reduce(|a, v| 2 * a + v).unwrap();
    println!("{}", result);
}
