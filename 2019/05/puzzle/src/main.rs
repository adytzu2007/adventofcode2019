use std::io;
use std::io::prelude::*;

fn run(mut operations: Vec<usize>) -> usize
{
    let mut i = 0;
    loop {
        match operations[i] {
            99 => return operations[0],
            op => {
                let op1 = operations[operations[i + 1]];
                let op2 = operations[operations[i + 2]];
                let output = operations[i + 3];
                let result = match op {
                    1 => op1 + op2,
                    2 => op1 * op2,
                    _ => panic!()
                };
                operations[output] = result;
            }
        }
        i += 4;
    }
}

fn main() {
    let mut operations: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect();
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            operations[1] = noun;
            operations[2] = verb;
            if run(operations.to_vec()) == 19690720 {
                println!("{}", noun * 100 + verb);
                break 'outer;
            }
        }
    }
}
