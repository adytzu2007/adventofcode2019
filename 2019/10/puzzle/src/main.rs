use std::io;
use std::io::prelude::*;
use std::collections::BTreeSet;
use std::f64;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn slope(x: usize, y: usize, c: usize, l: usize) -> (usize, i8, usize, i8) {
    let d = (if c < x { -1 } else { 1 }, if l < y { -1 } else { 1 });
    let s = (if c < x { x - c } else { c - x }, if l < y { y - l } else { l - y });
    let s = if s.0 < s.1 {
        (s.0 / gcd(s.1, s.0), s.1 / gcd(s.1, s.0))
    } else {
        (s.0 / gcd(s.0, s.1), s.1 / gcd(s.1, s.0))
    };
    (s.0, d.0, s.1, d.1)
}

fn angle(x: usize, y: usize, c: usize, l: usize) -> (u32, (usize, i8, usize, i8)) {
    let s = slope(x, y, c, l);
    ((if s.1 > 0 && s.3 < 0 {
        // upper-right quadrant
        (s.0 as f64).atan2(s.2 as f64)
    } else if s.1 > 0 && s.3 > 0 {
        // lower-right quadrant
        f64::consts::PI - (s.0 as f64).atan2(s.2 as f64)
    } else if s.1 < 0 && s.3 > 0 {
        // lower-left quadrant
        f64::consts::PI + (s.0 as f64).atan2(s.2 as f64)
    } else {
        // upper-left quadrant
        2.0 * f64::consts::PI - (s.0 as f64).atan2(s.2 as f64)
    } * 10000.0) as u32, s)
}

fn visible_asteroids(x: usize, y: usize, mut map: Vec<Vec<u8>>) -> u32 {
    let mut vas = 0;
    for l in 0..map.len() {
        for c in 0..map[l].len() {
            if map[l][c] == 0 || (x == c && y == l) {
                continue;
            }

            vas += 1;

            let s = slope(x, y, c, l);
            let mut xp = x + s.0 * s.1 as usize;
            let mut yp = y + s.2 * s.3 as usize;
            while xp < map[l].len() && yp < map.len() {
                map[yp][xp] = 0;
                xp += s.0 * s.1 as usize;
                yp += s.2 * s.3 as usize;
            }
        }
    }
    vas
}

fn pretty_print(map: &Vec<Vec<u8>>) {
    eprint!("    ");
    for c in 0..map[0].len() {
        eprint!(" {:03} ", c);
    }
    eprintln!("");
    for l in 0..map.len() {
        eprint!("{:2}  ", l);
        for c in 0..map[l].len() {
            eprint!("  {}  ", match map[l][c] { 0 => '.', 1 => '#', 2 => 'X', 3 => 'O',_ => panic!() });
        }
        eprintln!("");
    }
}

fn main() {
    let mut map: Vec<Vec<u8>> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|b| match b {
                    '#' => 1,
                    _ => 0,
                })
                .collect::<Vec<u8>>()
        })
        .collect();
    let mut max = 0;
    let mut coords = (0, 0);
    for l in 0..map.len() {
        for c in 0..map[l].len() {
            if map[l][c] == 1 {
                let vas = visible_asteroids(c, l, map.to_vec());
                if vas > max {
                    coords = (c, l);
                    max = vas;
                }
            }
        }
    }
    map[coords.1][coords.0] = 3;
    println!("{}", max);
    let mut slopes = BTreeSet::new();
    for l in 0..map.len() {
        for c in 0..map[l].len() {
            if map[l][c] == 1{
                slopes.insert(angle(coords.0, coords.1, c, l));
            }
        }
    }
    let mut vaporized = 0;
    let mut it = slopes.iter();
    'outer: loop {
        match it.next() {
            None => it = slopes.iter(),
            Some((_, s)) => {
                let mut xp = coords.0 + s.0 * s.1 as usize;
                let mut yp = coords.1 + s.2 * s.3 as usize;
                while xp < map[0].len() && yp < map.len() {
                    if map[yp][xp] == 1 {
                        vaporized += 1;
                        map[yp][xp] = 2;
                        if vaporized == 200 {
                            println!("{}", xp * 100 + yp);
                            break 'outer;
                        }
                        break;
                    }
                    xp += s.0 * s.1 as usize;
                    yp += s.2 * s.3 as usize;
                }
            }
        }
    }
}
