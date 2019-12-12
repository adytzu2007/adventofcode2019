extern crate itertools;
extern crate num;

use itertools::Itertools;
use std::collections::BTreeMap;
use std::io;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,
    dx: i64,
    dy: i64,
    dz: i64,
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon {
            x: x,
            y: y,
            z: z,
            dx: 0,
            dy: 0,
            dz: 0,
        }
    }
}

fn print_moons(moons: &Vec<Moon>) {
    for moon in moons {
        eprintln!("{:?}", moon);
    }
}

fn apply_gravity(m1: &mut Moon, m2: &mut Moon) {
    let changes = [m1.x - m2.x, m1.y - m2.y, m1.z - m2.z]
        .iter()
        .map(|d| num::clamp(*d, -1, 1))
        .collect::<Vec<i64>>();
    m1.dx -= changes[0];
    m2.dx += changes[0];
    m1.dy -= changes[1];
    m2.dy += changes[1];
    m1.dz -= changes[2];
    m2.dz += changes[2];
}

fn apply_velocity(moon: &mut Moon) {
    moon.x += moon.dx;
    moon.y += moon.dy;
    moon.z += moon.dz;
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
    let mut moons: Vec<Moon> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let mut coords: BTreeMap<String, i64> = BTreeMap::new();
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
    for step in 0..1000 {
        do_step(&mut moons);
    }
    println!("{}", moons.iter()
        .map(|moon| {
            (moon.x.abs() + moon.y.abs() + moon.z.abs())
                * (moon.dx.abs() + moon.dy.abs() + moon.dz.abs())
        })
        .fold(0, |acc, x| acc + x));
}
