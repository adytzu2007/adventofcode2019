extern crate num;

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
    X,
    Y,
    Tile,
    YScore,
    Score,
}

fn print_screen(screen: &BTreeMap<(i64, i64), i64>, width: i64, height: i64) {
    for i in 0..height {
        for j in 0..width {
            match screen.get(&(j, i)) {
                None => eprint!(" "),
                Some(0) => eprint!(" "),
                Some(1) => eprint!("+"),
                Some(2) => eprint!("#"),
                Some(3) => eprint!("="),
                Some(4) => eprint!("o"),
                Some(v) => panic!(dbg!(*v))
            }
        }
        eprintln!("");
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
    let mut arcade = State::new(&memory, 1);
    arcade.memory.insert(0, 2); // free play
    let mut screen: BTreeMap<(i64, i64), i64> = BTreeMap::new();
    let mut turn = Action::X;
    let mut coords: Vec<i64> = vec![];
    let mut score = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;

    loop {
        match arcade.run() {
            Output::Value(v) => {
                turn = match (turn, v) {
                    (Action::X, -1) => {
                        Action::YScore
                    },
                    (Action::YScore, 0) => {
                        Action::Score
                    },
                    (Action::Score, v) => {
                        score = v;
                        Action::X
                    }
                    (Action::X, v) => {
                        max_x = std::cmp::max(max_x, v);
                        coords.push(v);
                        Action::Y
                    },
                    (Action::Y, v) => {
                        max_y = std::cmp::max(max_y, v);
                        coords.push(v);
                        Action::Tile
                    },
                    (Action::Tile, v) => {
                        screen.insert((coords[0], coords[1]), v);
                        if v == 4 {
                            ball_x = coords[0];
                        } else if v == 3 {
                            paddle_x = coords[0];
                        }
                        coords.clear();
                        Action::X
                    },
                    _ => panic!()
                }
            }
            Output::Halt(v) => {
                println!("Halt {}", v);
                break;
            },
            Output::NeedsInput => {
                //print_screen(&screen, max_x + 1, max_y + 1);
                arcade.input = Some(num::clamp(ball_x - paddle_x, -1, 1));
            }
        }
    }
    println!("{}", score);
}
