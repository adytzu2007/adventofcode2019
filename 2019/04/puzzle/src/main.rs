extern crate num;

use std::cmp::min;
use std::io;
use std::io::prelude::*;

fn valid(mut code: u32) -> u32 {
    let mut last_digit = code % 10;
    let mut repeating = 0;
    let mut double = 0;
    loop {
        code = code / 10;
        if code == 0 {
            if repeating == 1 {
                double = 1;
            }
            break;
        }

        if last_digit < code % 10 {
            return 0;
        }
        if last_digit == code % 10 {
            repeating = repeating + 1;
        } else {
            if repeating == 1 {
                double = 1;
            }
            repeating = 0;
        }

        last_digit = code % 10;
    }
    double
}

fn main() {
    let ranges: Vec<Vec<u32>> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    for range in ranges {
        let mut valid_codes = 0;
        for i in range[0]..range[1] + 1 {
            valid_codes = valid_codes + valid(i);
        }
        println!("{:?} {}", range, valid_codes)
    }
}
