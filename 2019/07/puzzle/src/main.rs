extern crate permutator;

use permutator::Permutation;
use std::io;
use std::io::prelude::*;

fn get_operand(memory: &[i32], mode: i32, i: usize) -> i32 {
    if mode == 1 {
        memory[i]
    } else {
        memory[memory[i] as usize]
    }
}

fn get_output_position(memory: &[i32], mode: i32, i: usize) -> usize {
    assert!(mode == 0);
    memory[i] as usize
}

#[derive(PartialEq)]
enum Output {
    Halt(i32),
    NeedsInput,
    Signal(i32),
}

struct State {
    memory: Vec<i32>,
    input: Option<i32>,
    ip: usize,
}

impl State {
    fn run(&mut self) -> Output {
        loop {
            match (self.memory[self.ip] / 100, self.memory[self.ip] % 100) {
                (0, 99) => return Output::Halt(self.memory[0]),
                (mode, op) => {
                    let modes: [i32; 3] = [mode % 10, mode % 100 / 10, mode / 100];
                    modes
                        .iter()
                        .for_each(|mode| assert!(*mode == 0 || *mode == 1));
                    match op {
                        1 | 2 => {
                            let op1 = get_operand(&self.memory, modes[0], self.ip + 1);
                            let op2 = get_operand(&self.memory, modes[1], self.ip + 2);
                            let to = get_output_position(&self.memory, modes[2], self.ip + 3);
                            if op == 1 {
                                self.memory[to] = op1 + op2;
                            } else {
                                self.memory[to] = op1 * op2;
                            }
                            self.ip += 4;
                        }
                        3 => {
                            if self.input.is_none() {
                                return Output::NeedsInput;
                            }
                            let to = get_output_position(&self.memory, modes[0], self.ip + 1);
                            self.memory[to] = self.input.unwrap();
                            self.input = None;
                            self.ip += 2;
                        }
                        4 => {
                            let to = get_operand(&self.memory, modes[0], self.ip + 1);
                            self.ip += 2;
                            return Output::Signal(to);
                        }
                        5 | 6 => {
                            let cond = get_operand(&self.memory, modes[0], self.ip + 1);
                            let destination = get_operand(&self.memory, modes[1], self.ip + 2);
                            if (op == 5 && cond != 0) || (op == 6 && cond == 0) {
                                self.ip = destination as usize;
                            } else {
                                self.ip += 3;
                            }
                        }
                        7 | 8 => {
                            let op1 = get_operand(&self.memory, modes[0], self.ip + 1);
                            let op2 = get_operand(&self.memory, modes[1], self.ip + 2);
                            let to = get_output_position(&self.memory, modes[2], self.ip + 3);
                            if (op == 7 && op1 < op2) || (op == 8 && op1 == op2) {
                                self.memory[to] = 1;
                            } else {
                                self.memory[to] = 0;
                            }
                            self.ip += 4;
                        }
                        _ => panic!(),
                    };
                }
            };
        }
    }
}

fn main() {
    let memory: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .flatten()
        .collect();
    let phase_settings = &mut [0, 1, 2, 3, 4];
    let mut max_signal = 0;
    for phase_setting in phase_settings.permutation() {
        let mut amplifiers = phase_setting
            .iter()
            .map(|ps| State { memory: memory.to_vec(), input: Some(*ps), ip: 0 })
            .collect::<Vec<State>>();
        let result = amplifiers[0].run();
        assert!(result == Output::NeedsInput);
        amplifiers[0].input = Some(0);
        let signal = match amplifiers[0].run() {
            Output::Signal(s) => s,
            _ => panic!()
        };
        let result = amplifiers[1].run();
        assert!(result == Output::NeedsInput);
        amplifiers[1].input = Some(signal);
        let signal = match amplifiers[1].run() {
            Output::Signal(s) => s,
            _ => panic!()
        };
        let result = amplifiers[2].run();
        assert!(result == Output::NeedsInput);
        amplifiers[2].input = Some(signal);
        let signal = match amplifiers[2].run() {
            Output::Signal(s) => s,
            _ => panic!()
        };
        let result = amplifiers[3].run();
        assert!(result == Output::NeedsInput);
        amplifiers[3].input = Some(signal);
        let signal = match amplifiers[3].run() {
            Output::Signal(s) => s,
            _ => panic!()
        };
        let result = amplifiers[4].run();
        assert!(result == Output::NeedsInput);
        amplifiers[4].input = Some(signal);
        match amplifiers[4].run() {
            Output::Signal(s) => max_signal = std::cmp::max(max_signal, s),
            _ => panic!()
        };

    }
    println!("{}", max_signal);
}
