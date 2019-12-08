use std::io;
use std::io::prelude::*;

fn count_digits(digits: &[u8]) -> [u32; 10] {
    let mut count: [u32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for d in digits {
        let digit = d - '0' as u8;
        count[digit as usize] += 1
    }
    count
}

fn main() {
    io::stdin().lock().lines().for_each(|line| {
        line.map(|line| {
            let mut pixels = line.as_bytes()[..25 * 6].to_vec();
            line.as_bytes().chunks(25 * 6).skip(1).for_each(|layer| {
                for (i, pixel) in layer.iter().enumerate() {
                    if pixels[i] == '2' as u8 {
                        pixels[i] = *pixel
                    }
                    if pixels[i] == '0' as u8 {
                        pixels[i] = ' ' as u8;
                    }
                }
            });
            pixels[..]
                .chunks(25)
                .for_each(|line| println!("{}", std::str::from_utf8(&line).unwrap()));
        });
    });
}
