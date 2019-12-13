extern crate num;

use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Moon {
    coords: [(i64, i64); 3],
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon {
            coords: [(x, 0), (y, 0), (z, 0)],
        }
    }
}

fn print_moons(moons: &Vec<Moon>) {
    for moon in moons {
        eprintln!("{:?}", moon);
    }
}

fn apply_gravity(m1: &mut Moon, m2: &mut Moon) {
    let changes = m1
        .coords
        .iter()
        .zip(m2.coords.iter())
        .map(|pair| (pair.0).0 - (pair.1).0)
        .map(|change| num::clamp(change, -1, 1))
        .collect::<Vec<i64>>();
    for i in 0..changes.len() {
        m1.coords[i].1 -= changes[i];
        m2.coords[i].1 += changes[i];
    }
}

fn apply_velocity(moon: &mut Moon) {
    for coord in &mut moon.coords {
        coord.0 += coord.1;
    }
}

fn do_step(moons: &mut Vec<Moon>, combos: &Vec<Vec<usize>>) {
    for c in combos {
        let mut m1 = moons[c[0]];
        let mut m2 = moons[c[1]];
        apply_gravity(&mut m1, &mut m2);
        moons[c[0]] = m1;
        moons[c[1]] = m2;
    }
    for mut moon in moons {
        apply_velocity(&mut moon);
    }
}

fn main() {
    let moons: Vec<Moon> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let mut coords: HashMap<String, i64> = HashMap::new();
            line.unwrap()
                .trim_start_matches("<")
                .trim_end_matches(">")
                .split(',')
                .for_each(|s| {
                    let pair = s.trim_start().trim_end().split('=').collect::<Vec<&str>>();
                    coords.insert(pair[0].to_string(), pair[1].parse::<i64>().unwrap());
                });
            Moon::new(
                *coords.get("x").unwrap(),
                *coords.get("y").unwrap(),
                *coords.get("z").unwrap(),
            )
        })
        .collect();
    let combos = (0..moons.len()).combinations(2).collect();

    let mut cycles: Vec<Option<u64>> = vec![None, None, None];
    let mut steps = 0;
    let mut simulated_moons = moons.to_vec();
    let mut found = 0;

    loop {
        steps += 1;
        do_step(&mut simulated_moons, &combos);

        for i in 0..3 {
            if cycles[i].is_none() {
                if simulated_moons.iter().zip(moons.iter()).
                    map(|pair| pair.0.coords[i] == pair.1.coords[i])
                        .fold(true, |acc, x| acc && x) {
                    cycles[i] = Some(steps);
                    found += 1;
                }
            }
        }
        if found == 3 {
            break;
        }
    }
    println!(
        "{}",
        cycles.iter().map(|o| o.unwrap()).fold(1, |acc, x| num::integer::lcm(acc, x)));
}
