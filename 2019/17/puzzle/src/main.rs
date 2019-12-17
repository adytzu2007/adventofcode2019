use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::io::prelude::*;
use std::{thread, time};

#[derive(Debug, PartialEq)]
enum Output {
    Halt(i64),
    NeedsInput,
    Value(i64),
}

#[derive(Debug)]
struct State {
    memory: BTreeMap<usize, i64>,
    input: Option<i64>,
    ip: usize,
    rb: usize,
}

impl State {
    fn new(memory: &Vec<i64>) -> State {
        let mut s = State {
            memory: BTreeMap::new(),
            input: None,
            ip: 0,
            rb: 0,
        };
        for (i, v) in memory.iter().enumerate() {
            s.memory.insert(i, *v);
        }
        s
    }

    fn get_operand(&self, mode: i64, i: usize) -> i64 {
        match mode {
            1 => self.get(i),
            _ => self.get(self.get_address(mode, i)),
        }
    }

    fn get_address(&self, mode: i64, i: usize) -> usize {
        let immediate = self.get(i);
        let address = match mode {
            0 => immediate,
            2 => immediate + self.rb as i64,
            _ => panic!(),
        };
        assert!(address >= 0);
        address as usize
    }

    fn get(&self, i: usize) -> i64 {
        match self.memory.get(&i) {
            Some(v) => *v,
            None => 0,
        }
    }

    fn set(&mut self, i: usize, v: i64) {
        *self.memory.entry(i).or_insert(0) = v;
    }

    fn run(&mut self) -> Output {
        loop {
            let opcode = self.get(self.ip);
            match (opcode / 100, opcode % 100) {
                (0, 99) => return Output::Halt(self.get(0)),
                (mode, op) => {
                    let modes: [i64; 3] = [mode % 10, mode % 100 / 10, mode / 100];
                    match op {
                        1 | 2 => {
                            let op1 = self.get_operand(modes[0], self.ip + 1);
                            let op2 = self.get_operand(modes[1], self.ip + 2);
                            let to = self.get_address(modes[2], self.ip + 3);
                            if op == 1 {
                                self.set(to, op1 + op2);
                            } else {
                                self.set(to, op1 * op2);
                            }
                            self.ip += 4;
                        }
                        3 => {
                            if self.input.is_none() {
                                return Output::NeedsInput;
                            }
                            let to = self.get_address(modes[0], self.ip + 1);
                            self.set(to, self.input.unwrap());
                            self.input = None;
                            self.ip += 2;
                        }
                        4 => {
                            let to = self.get_operand(modes[0], self.ip + 1);
                            self.ip += 2;
                            return Output::Value(to);
                        }
                        5 | 6 => {
                            let cond = self.get_operand(modes[0], self.ip + 1);
                            let destination = self.get_operand(modes[1], self.ip + 2);
                            if (op == 5 && cond != 0) || (op == 6 && cond == 0) {
                                self.ip = destination as usize;
                            } else {
                                self.ip += 3;
                            }
                        }
                        7 | 8 => {
                            let op1 = self.get_operand(modes[0], self.ip + 1);
                            let op2 = self.get_operand(modes[1], self.ip + 2);
                            let to = self.get_address(modes[2], self.ip + 3);
                            if (op == 7 && op1 < op2) || (op == 8 && op1 == op2) {
                                self.set(to, 1);
                            } else {
                                self.set(to, 0);
                            }
                            self.ip += 4;
                        }
                        9 => {
                            let op1 = self.get_operand(modes[0], self.ip + 1);
                            self.rb += op1 as usize;
                            self.ip += 2;
                        }
                        op => panic!(dbg!(op)),
                    };
                }
            };
        }
    }
}

fn print_map(map: &BTreeMap<(i64, i64), char>, lower_left: (i64, i64), upper_right: (i64, i64)) {
    for j in lower_left.0 - 4..upper_right.0 + 2 {
        eprint!("{}", j.abs() / 10);
    }
    eprintln!("");
    for j in lower_left.0 - 4..upper_right.0 + 2 {
        eprint!("{}", j.abs() % 10);
    }
    eprintln!("");
    for _ in lower_left.0 - 4..upper_right.0 + 2 {
        eprint!("=");
    }
    eprintln!("");
    for i in (lower_left.1..upper_right.1 + 1).rev() {
        eprint!("{:3}|", i);
        for j in lower_left.0..upper_right.0 + 1 {
            match map.get(&(j, i)) {
                None => eprint!(" "),
                Some(ch) => eprint!("{}", ch),
            }
        }
        eprint!("|");
        eprintln!("");
    }
    for _ in lower_left.0 - 4..upper_right.0 + 2 {
        eprint!("=");
    }
    eprintln!("");
    thread::sleep(time::Duration::from_millis(100));
}

fn right(d: (i64, i64)) -> (i64, i64) {
    match d {
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        _ => panic!(),
    }
}

fn left(d: (i64, i64)) -> (i64, i64) {
    match d {
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        _ => panic!(),
    }
}

fn main() {
    let memory: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .flatten()
        .collect();

    let mut camera = State::new(&memory);
    let mut pos = (0, 0);
    let mut map: BTreeMap<(i64, i64), char> = BTreeMap::new();
    let mut lower_left = (0, 0);
    let mut upper_right = (0, 0);
    let mut robot = None;
    loop {
        match camera.run() {
            Output::Value(v) => {
                pos = match v {
                    10 => (0, pos.1 - 1),
                    ch => {
                        let ch = ch as u8 as char;
                        match ch {
                            '^' | 'v' | '<' | '>' => robot = Some(pos),
                            _ => {}
                        };
                        lower_left.0 = std::cmp::min(lower_left.0, pos.0);
                        lower_left.1 = std::cmp::min(lower_left.1, pos.1);
                        upper_right.0 = std::cmp::max(upper_right.0, pos.0);
                        upper_right.1 = std::cmp::max(upper_right.1, pos.1);
                        map.insert(pos, ch);
                        (pos.0 + 1, pos.1)
                    }
                };
            }
            Output::Halt(v) => {
                println!("Halt {}", v);
                break;
            }
            Output::NeedsInput => panic!(),
        }
    }

    let mut positions = vec![robot.unwrap()];
    let mut visited: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut intersections = vec![];
    while positions.len() > 0 {
        let mut current_pos = positions.remove(0);
        if visited.contains(&current_pos) {
            continue;
        }
        visited.insert(current_pos);
        let mut directions = 0;
        let mut d = (0, 1);
        loop {
            let next_pos = (current_pos.0 + d.0, current_pos.1 + d.1);
            match map.get(&next_pos) {
                Some('#') => {
                    positions.push(next_pos);
                    directions += 1;
                }
                _ => {}
            }
            d = right(d);
            if d == (0, 1) {
                break;
            }
        }
        if directions >= 3 {
            intersections.push(current_pos);
        }
    }
    println!(
        "{}",
        intersections
            .iter()
            .fold(0, |acc, x| acc + x.0.abs() * x.1.abs())
    );

    pos = robot.unwrap();
    let mut d = match map.get(&pos) {
        None => panic!(),
        Some('^') => (0, 1),
        Some('v') => (0, -1),
        Some('<') => (-1, 0),
        Some('>') => (1, 0),
        _ => panic!(),
    };
    let mut dir = 'L';
    loop {
        match map.get(&(pos.0 + left(d).0, pos.1 + left(d).1)) {
            Some('#') | Some('L') | Some('R') => {
                d = left(d);
                dir = 'L';
                print!("L,");
            }
            _ => match map.get(&(pos.0 + right(d).0, pos.1 + right(d).1)) {
                Some('#') | Some('L') | Some('R') => {
                    d = right(d);
                    dir = 'R';
                    print!("R,");
                }
                _ => break,
            },
        }
        let mut distance = 0;
        loop {
            map.insert(pos, dir);
            distance += 1;
            match map.get(&(pos.0 + d.0, pos.1 + d.1)) {
                Some('#') | Some('R') | Some('L') => {}
                _ => break,
            }
            pos = (pos.0 + d.0, pos.1 + d.1);
        }
        print!("{},", distance);
    }
    println!("");

    let mut robot = State::new(&memory);
    robot.memory.insert(0, 2);
    let sequences = [
        "A,A,B,C,B,C,B,A,C,A\n".to_string(),
        "R,8,L,12,R,8\n".to_string(),
        "L,10,L,10,R,8\n".to_string(),
        "L,12,L,12,L,10,R,10\n".to_string(),
        "n\n".to_string(),
    ];
    sequences.iter().for_each(|x| assert!(x.len() <= 20));
    let mut sequence = sequences.join("");
    let mut it = sequence.as_bytes().iter();
    let mut pos = (0, 0);
    let mut map: BTreeMap<(i64, i64), char> = BTreeMap::new();
    let mut lower_left = (0, 0);
    let mut upper_right = (0, 0);

    loop {
        match robot.run() {
            Output::Value(v) => {
                if v >= 128 {
                    println!("{}", v);
                } else {
                    pos = match v {
                        10 => (0, pos.1 - 1),
                        ch => {
                            let ch = ch as u8 as char;
                            lower_left.0 = std::cmp::min(lower_left.0, pos.0);
                            lower_left.1 = std::cmp::min(lower_left.1, pos.1);
                            upper_right.0 = std::cmp::max(upper_right.0, pos.0);
                            upper_right.1 = std::cmp::max(upper_right.1, pos.1);
                            map.insert(pos, ch);
                            (pos.0 + 1, pos.1)
                        }
                    };
                }
            }
            Output::Halt(v) => {
                println!("{:?} {}", it, v);
                break;
            }
            Output::NeedsInput => match it.next() {
                Some(ch) => robot.input = Some(*ch as i64),
                None => panic!(),
            },
        }
    }
    print_map(&map, lower_left, upper_right);
}
