extern crate itertools;
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

fn do_step(moons: &mut Vec<Moon>) {
    for c in (0..moons.len()).combinations(2) {
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
    let mut cycles: HashMap<usize, (u64, u64)> = HashMap::new();
    let mut steps = 0;
    let mut simulated_moons = moons.to_vec();
    let mut positions = moons[0]
        .coords
        .iter()
        .map(|_| HashMap::new())
        .collect::<Vec<HashMap<Vec<(i64, i64)>, u64>>>();
    let mut single_coords = (0..moons[0].coords.len())
        .map(|i| {
            simulated_moons
                .iter()
                .map(|moon| moon.coords[i])
                .collect::<Vec<(i64, i64)>>()
        })
        .collect::<Vec<Vec<(i64, i64)>>>();

    loop {
        for (i, single_coord) in single_coords.iter().enumerate() {
            positions[i].insert(single_coord.to_vec(), steps);
        }

        steps += 1;
        do_step(&mut simulated_moons);
        single_coords = (0..moons[0].coords.len())
            .map(|i| {
                simulated_moons
                    .iter()
                    .map(|moon| moon.coords[i])
                    .collect::<Vec<(i64, i64)>>()
            })
            .collect();

        for (i, single_coord) in single_coords.iter().enumerate() {
            if cycles.get(&i).is_none() {
                if let Some(previous_steps) = positions[i].get(single_coord) {
                    cycles.insert(i, (*previous_steps, steps - previous_steps));
                }
            }
        }
        if cycles.len() == 3 {
            break;
        }
    }
    println!(
        "{}",
        num::integer::lcm(num::integer::lcm(cycles[&0].1, cycles[&1].1), cycles[&2].1)
    );
}
