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
    fn new(memory: &Vec<i64>, input: i64) -> State {
        let mut s = State {
            memory: BTreeMap::new(),
            input: Some(input),
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

enum Action {
    Move,
}

fn print_map(
    map: &BTreeMap<(i64, i64), (u8, u8)>,
    lower_left: (i64, i64),
    upper_right: (i64, i64),
    pos: (i64, i64),
) {
    for j in lower_left.0 - 3..upper_right.0 + 2 {
        eprint!("{}", j.abs() / 10);
    }
    eprintln!("");
    for j in lower_left.0 - 3..upper_right.0 + 2 {
        eprint!("{}", j.abs() % 10);
    }
    eprintln!("");
    for _ in lower_left.0 - 3..upper_right.0 + 2 {
        eprint!("=");
    }
    eprintln!("");
    for i in (lower_left.1..upper_right.1 + 1).rev() {
        eprint!("{:3}|", i);
        for j in lower_left.0..upper_right.0 + 1 {
            if (j, i) == pos {
                eprint!("D")
            } else {
                match map.get(&(j, i)) {
                    None => eprint!(" "),
                    Some((1, _)) => eprint!("."),
                    //Some((1, 2)) => eprint!("E"),
                    Some((0, _)) => eprint!("#"),
                    Some((2, _)) => eprint!("o"),
                    Some((3, _)) => eprint!("P"),
                    Some(v) => panic!(dbg!(*v)),
                }
            }
        }
        eprint!("|");
        eprintln!("");
    }
    for _ in lower_left.0 - 3..upper_right.0 + 2 {
        eprint!("=");
    }
    eprintln!("");
    thread::sleep(time::Duration::from_millis(100));
}

fn get_next(d: (i64, i64, i64)) -> (i64, i64, i64) {
    match d {
        (0, 1, 1) => {
            // North
            (0, -1, 2) // South
        }
        (0, -1, 2) => {
            // South
            (-1, 0, 3) // West
        }
        (-1, 0, 3) => {
            // West
            (1, 0, 4) // East
        }
        (1, 0, 4) => {
            // East
            (0, 1, 1) // North
        }
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
    let mut robot = State::new(&memory, 1);
    let mut turn = Action::Move;
    let mut pos = (0, 0);
    let mut d = (0, 1, 1);
    let mut oxygen_system = None;
    let mut map: BTreeMap<(i64, i64), (u8, u8)> = BTreeMap::new();
    let mut lower_left = (-1, -1);
    let mut upper_right = (1, 1);
    map.insert(pos, (1, 1));

    'outer: loop {
        match robot.run() {
            Output::Value(v) => {
                turn = match (turn, v) {
                    (Action::Move, found) => {
                        match found {
                            0 => {
                                lower_left.0 = std::cmp::min(lower_left.0, pos.0 + d.0);
                                lower_left.1 = std::cmp::min(lower_left.1, pos.1 + d.1);
                                upper_right.0 = std::cmp::max(upper_right.0, pos.0 + d.0);
                                upper_right.1 = std::cmp::max(upper_right.1, pos.1 + d.1);
                                map.insert(((pos.0 + d.0), (pos.1 + d.1)), (0, 2));
                            }
                            1 | 2 => {
                                pos.0 += d.0;
                                pos.1 += d.1;
                                lower_left.0 = std::cmp::min(lower_left.0, pos.0);
                                lower_left.1 = std::cmp::min(lower_left.1, pos.1);
                                upper_right.0 = std::cmp::max(upper_right.0, pos.0);
                                upper_right.1 = std::cmp::max(upper_right.1, pos.1);
                                if !map.contains_key(&pos) {
                                    map.insert(pos, (found as u8, 1));
                                }
                                if found == 2 {
                                    oxygen_system = Some(pos);
                                }
                            }
                            _ => panic!(),
                        }
                        let mut directions = vec![];
                        let mut new_d = d;
                        loop {
                            new_d = get_next(new_d);
                            match map.get(&((pos.0 + new_d.0), (pos.1 + new_d.1))) {
                                None => directions.insert(0, new_d),
                                Some((_, 2)) => {}
                                Some((_, 1)) => directions.push(new_d),
                                Some((_, 0)) => directions.insert(0, new_d),
                                _ => panic!(),
                            };
                            if d == new_d {
                                break;
                            }
                        }
                        if directions.len() == 0 {
                            break 'outer;
                        } else if directions.len() == 1 {
                            map.insert(pos, (map.get(&pos).unwrap().0, 2));
                        }
                        d = directions[0];
                        //print_map(&map, lower_left, upper_right, pos);
                        Action::Move
                    }
                };
            }
            Output::Halt(v) => {
                println!("Halt {}", v);
                break;
            }
            Output::NeedsInput => {
                robot.input = Some(d.2);
            }
        }
    }
    let mut positions = vec![((0, 0), 0)];

    let mut visited: BTreeSet<(i64, i64)> = BTreeSet::new();
    while positions.len() > 0 {
        let (current_pos, distance) = positions.remove(0);
        visited.insert(current_pos);
        if map.get(&current_pos).unwrap().0 == 2 {
            println!("{}", distance);
            break;
        }
        map.insert(current_pos, (3, 0));
        let mut new_d = d;
        loop {
            new_d = get_next(new_d);
            let new_position = ((current_pos.0 + new_d.0), (current_pos.1 + new_d.1));
            if !visited.contains(&new_position) {
                match map.get(&new_position) {
                    Some((0, _)) => {},
                    Some((_, _)) => {
                        positions.push((new_position, distance + 1));
                    },
                    _ => panic!(),
                }
            }
            if d == new_d {
                break;
            }
        }
    }

    let mut positions = vec![((oxygen_system.unwrap().0, oxygen_system.unwrap().1), 0)];

    let mut visited: BTreeSet<(i64, i64)> = BTreeSet::new();
    while positions.len() > 0 {
        let (current_pos, distance) = positions.remove(0);
        visited.insert(current_pos);
        map.insert(current_pos, (3, 0));
        let mut new_d = d;
        loop {
            new_d = get_next(new_d);
            let new_position = ((current_pos.0 + new_d.0), (current_pos.1 + new_d.1));
            if !visited.contains(&new_position) {
                match map.get(&new_position) {
                    Some((0, _)) => {},
                    Some((_, _)) => {
                        positions.push((new_position, distance + 1));
                    },
                    _ => panic!(),
                }
            }
            if d == new_d {
                break;
            }
        }
        if positions.len() == 0 {
            println!("{}", distance);
        }
    }
}
