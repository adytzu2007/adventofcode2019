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
    dbg!(sequence.len());
    let offset: usize = dbg!(sequence[0..7].iter().fold(0, |acc, x| acc * 10 + *x as usize));
    sequence = repeat_n(sequence, 10000).flatten().collect::<Vec<i32>>()[offset..].to_vec();
    for _phase in 0..100 {
        let mut sum = sequence.iter().fold(0, |acc, x| acc + x);
        sequence = (0..sequence.len())
            .map(|i| { sum -= sequence[i]; sum + sequence[i] })
            .map(|x| x.abs() % 10)
            .collect::<Vec<i32>>();
    }
    for i in &sequence[0..8] {
        print!("{}", i);
    }
    println!("");
}
