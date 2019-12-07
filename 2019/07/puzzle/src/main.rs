extern crate permutator;

use permutator::Permutation;
use std::io;
use std::io::prelude::*;

fn get_operand(operations: &[i32], mode: i32, i: usize) -> i32 {
    if mode == 1 {
        operations[i]
    } else {
        operations[operations[i] as usize]
    }
}

fn get_output_position(operations: &[i32], mode: i32, i: usize) -> usize {
    assert!(mode == 0);
    operations[i] as usize
}

enum InputType {
    PhaseSetting,
    InputSignal,
}

fn run<I: Fn() -> i32, O: FnMut(i32) -> ()>(
    mut operations: Vec<i32>,
    phase_setting: i32,
    input: I,
    mut output: O,
) -> i32
{
    let mut i: usize = 0;
    let mut input_type = InputType::PhaseSetting;
    loop {
        match (operations[i] / 100, operations[i] % 100) {
            (0, 99) => return operations[0],
            (mode, op) => {
                let modes: [i32; 3] = [mode % 10, mode % 100 / 10, mode / 100];
                modes
                    .iter()
                    .for_each(|mode| assert!(*mode == 0 || *mode == 1));
                match op {
                    1 | 2 => {
                        let op1 = get_operand(&operations, modes[0], i + 1);
                        let op2 = get_operand(&operations, modes[1], i + 2);
                        let to = get_output_position(&operations, modes[2], i + 3);
                        if op == 1 {
                            operations[to] = op1 + op2;
                        } else {
                            operations[to] = op1 * op2;
                        }
                        i += 4;
                    }
                    3 => {
                        let to = get_output_position(&operations, modes[0], i + 1);
                        operations[to] = match input_type {
                            InputType::PhaseSetting => {
                                input_type = InputType::InputSignal;
                                phase_setting
                            }
                            InputType::InputSignal => input(),
                        };
                        i += 2;
                    }
                    4 => {
                        let to = get_operand(&operations, modes[0], i + 1);
                        output(to);
                        i += 2;
                    }
                    5 | 6 => {
                        let cond = get_operand(&operations, modes[0], i + 1);
                        let destination = get_operand(&operations, modes[1], i + 2);
                        if (op == 5 && cond != 0) || (op == 6 && cond == 0) {
                            i = destination as usize;
                        } else {
                            i += 3;
                        }
                    }
                    7 | 8 => {
                        let op1 = get_operand(&operations, modes[0], i + 1);
                        let op2 = get_operand(&operations, modes[1], i + 2);
                        let to = get_output_position(&operations, modes[2], i + 3);
                        if (op == 7 && op1 < op2) || (op == 8 && op1 == op2) {
                            operations[to] = 1;
                        } else {
                            operations[to] = 0;
                        }
                        i += 4;
                    }
                    _ => panic!(),
                };
            }
        };
    }
}

fn main() {
    let operations: Vec<i32> = io::stdin()
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
    let mut max_signal: i32 = 0;
    for phase_setting in phase_settings.permutation() {
        run(operations.to_vec(), phase_setting[0], || 0, |signal: i32| {
            run(operations.to_vec(), phase_setting[1], || signal, |signal: i32| {
                run(operations.to_vec(), phase_setting[2], || signal, |signal: i32| {
                    run(operations.to_vec(), phase_setting[3], || signal, |signal: i32| {
                        run(operations.to_vec(), phase_setting[4], || signal, |signal: i32| {
                            max_signal = std::cmp::max(signal, max_signal);
                        });
                    });
                });
            });
        });
    }
    println!("{}", max_signal);
}
