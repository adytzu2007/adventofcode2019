extern crate clap;

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

fn run<I: FnOnce() -> i32, O: FnOnce(i32) -> ()>(operations: &mut Vec<i32>, input: I, output: O) -> i32
where I: Copy, O: Copy
{
    let mut i: usize = 0;
    loop {
        match (operations[i] / 100, operations[i] % 100) {
            (0, 99) => return operations[0],
            (mode, op) => {
                let modes: [i32; 3] = [mode % 10, mode % 100 / 10, mode / 100];
                modes.iter().for_each(|mode| assert!(*mode == 0 || *mode == 1));
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
                    },
                    3 => {
                        let to = get_output_position(&operations, modes[0], i + 1);
                        operations[to] = input();
                        i += 2;
                    },
                    4 => {
                        let to = get_operand(&operations, modes[0], i + 1);
                        output(to);
                        i += 2;
                    },
                    5 | 6 => {
                        let cond = get_operand(&operations, modes[0], i + 1);
                        let destination = get_operand(&operations, modes[1], i + 2);
                        if (op == 5 && cond != 0) || (op == 6 && cond == 0) {
                            i = destination as usize;
                        } else {
                            i += 3;
                        }
                    },
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
                    },
                    _ => panic!()
                };
            },
        };
    }
}

fn main() {
    let mut operations: Vec<i32> = io::stdin()
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
    let matches = clap::App::new("INTCODE machine")
        .version("1.0")
        .arg_from_usage("--input [INT] 'input to give to the machine'")
        .get_matches();
    let input = matches.value_of("input").map_or(5, |s| s.parse::<i32>().unwrap());
    println!("{}", run(&mut operations, || input, |i| println!("{}", i)));
}
