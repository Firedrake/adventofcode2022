use regex::Regex;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Square,
}

#[derive(Debug)]
pub struct Monkey {
    stack: VecDeque<u32>,
    operator: Operator,
    operand: u32,
    test: u32,
    iftrue: usize,
    iffalse: usize,
}

impl Monkey {
    pub fn new() -> Monkey {
        Monkey {
            stack: VecDeque::new(),
            operator: Operator::Add,
            operand: 0,
            test: 1,
            iftrue: 0,
            iffalse: 0,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let f = File::open(fname).unwrap();
    let fr = BufReader::new(f);
    let mut monkey: Vec<Monkey> = Vec::new();
    let mut active: Monkey = Monkey::new();
    let rn: Regex = Regex::new("^([0-9]+)").unwrap();
    for line in fr.lines() {
        let line = line.unwrap();
        if line.len() > 0 {
            let le = line
                .split(' ')
                .filter(|x| x.len() > 0)
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if le[0] == "Monkey" {
                if le[1] != "0:" {
                    monkey.push(active);
                }
                active = Monkey::new();
            } else if le[0] == "Starting" {
                for i in le.iter().skip(2) {
                    if rn.is_match(&i) {
                        let m = rn.captures(&i).unwrap();
                        active.stack.push_back(
                            m.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                        );
                    }
                }
            } else if le[0] == "Operation:" {
                active.operator = match le[4].as_str() {
                    "+" => Operator::Add,
                    "*" => Operator::Multiply,
                    _ => Operator::Add,
                };
                if le[5] == "old" {
                    active.operator = Operator::Square;
                } else {
                    active.operand = le[5].parse::<u32>().unwrap();
                }
            } else if le[0] == "Test:" {
                active.test = le[3].parse::<u32>().unwrap();
            } else if le[1] == "true:" {
                active.iftrue = le[5].parse::<usize>().unwrap();
            } else if le[1] == "false:" {
                active.iffalse = le[5].parse::<usize>().unwrap();
            }
        }
    }
    monkey.push(active);
    let mut monkeycount: Vec<usize> = vec![0; monkey.len()];
    for cycle in 1..=20 {
        for m in 0..=monkey.len() - 1 {
            monkeycount[m] += monkey[m].stack.len();
            loop {
                if monkey[m].stack.len() > 0 {
                    let mut worry = monkey[m].stack.pop_front().unwrap();
                    worry = match monkey[m].operator {
                        Operator::Add => worry + monkey[m].operand,
                        Operator::Multiply => worry * monkey[m].operand,
                        Operator::Square => worry * worry,
                    };
                    worry /= 3;
                    let mut target = monkey[m].iffalse;
                    if worry % monkey[m].test == 0 {
                        target = monkey[m].iftrue;
                    }
                    monkey[target].stack.push_back(worry);
                } else {
                    break;
                }
            }
            println!("End monkey {}", m);
        }
        println!("End cycle {}", cycle);
    }
    println!("{:?}", monkeycount);
    monkeycount.sort();
    monkeycount.reverse();
    println!("{}", monkeycount[0] * monkeycount[1]);
}
