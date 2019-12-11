use std::collections::BTreeMap;
use std::io;
use std::io::prelude::*;

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
                            let op = self.get_operand(modes[0], self.ip + 1);
                            self.rb += op as usize;
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
    Paint,
    Move,
}

fn turn_left(d: (i8, i8)) -> (i8, i8) {
    // (0, 1) => (-1, 0) => (0, -1) => (1, 0) => (0, 1)
    if d.1 == 1 {
        (-1, 0)
    } else if d.0 == -1 {
        (0, -1)
    } else if d.1 == -1 {
        (1, 0)
    } else {
        (0, 1)
    }
}

fn turn_right(d: (i8, i8)) -> (i8, i8) {
    // (0, 1) => (1, 0) => (0, -1) => (-1, 0) => (0, 1)
    if d.1 == 1 {
        (1, 0)
    } else if d.0 == 1 {
        (0, -1)
    } else if d.1 == -1 {
        (-1, 0)
    } else {
        (0, 1)
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
    let mut d: (i8, i8) = (0, 1);
    let mut position: (i64, i64) = (0, 0);
    let mut panels: BTreeMap<(i64, i64), i64> = BTreeMap::new();
    panels.insert(position, 1);
    let mut min: (i64, i64) = (100, 100);
    let mut max: (i64, i64) = (0, 0);
    let mut turn = Action::Paint;
    loop {
        match robot.run() {
            Output::Value(v) => {
                turn = match (turn, v) {
                    (Action::Paint, v) => {
                        min = (std::cmp::min(min.0, position.0), std::cmp::min(min.1, position.1));
                        max = (std::cmp::max(max.0, position.0), std::cmp::max(max.1, position.1));
                        panels.insert(position, v);
                        Action::Move
                    }
                    (Action::Move, v) => {
                        if v == 0 {
                            d = turn_left(d);
                        } else {
                            d = turn_right(d);
                        }
                        position = (position.0 + d.0 as i64, position.1 + d.1 as i64);
                        Action::Paint
                    }
                }
            }
            Output::Halt(v) => {
                println!("Halt {}", v);
                break;
            }
            Output::NeedsInput => {
                robot.input = Some(panels.get(&position).map(|v| *v).unwrap_or(0))
            }
        }
    }
    for y in (min.1..max.1+1).rev() {
        for x in min.0..max.0+1 {
            print!(
                "{}",
                panels
                    .get(&(x, y))
                    .map(|v| if *v == 1 { '#' } else { ' ' })
                    .unwrap_or(' ')
            );
        }
        println!("");
    }
}
