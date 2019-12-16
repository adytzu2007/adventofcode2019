extern crate itertools;

use itertools::{repeat_n, Itertools};
use std::io;
use std::io::prelude::*;

fn main() {
    let mut sequence: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            return line
                .unwrap()
                .as_bytes()
                .iter()
                .map(|d| (d - '0' as u8) as i32)
                .collect::<Vec<i32>>();
        })
        .flatten()
        .collect();
    for _phase in 0..100 {
        sequence = (0..sequence.len())
            .map(|i| {
                let mut sum = 0;
                let mut start = 0;
                loop {
                    let mut range = &sequence[start..];
                    if range.len() == 0 {
                        break;
                    }
                    repeat_n(0, i + 1)
                        .chain(
                            repeat_n(1, i + 1).chain(repeat_n(0, i + 1).chain(repeat_n(-1, i + 1))),
                        ).skip(if start == 0 { 1 } else { 0 })
                        .zip(range.iter())
                        .for_each(|(p, d)| {
                            sum += d * p;
                            start += 1;
                        });
                }
                sum.abs() % 10
            })
            .collect::<Vec<i32>>();
    }
    for i in sequence {
        print!("{}", i);
    }
    println!("");
}
