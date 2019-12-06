#![feature(option_expect_none)]

extern crate clap;

use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;

fn get_parents(mut object: String, parents: &HashMap<String, String>) -> HashSet<String> {
    let mut parent_set: HashSet<String> = HashSet::new();
    while parents.contains_key(&object) {
        object = parents.get(&object).unwrap().to_string();
        parent_set.insert(object.to_string());
    }
    parent_set
}

fn get_orbits(
    object: String,
    parents: &HashMap<String, String>,
    orbits: &mut HashMap<String, u32>,
) -> u32 {
    if orbits.contains_key(&object) {
        *orbits.get(&object).unwrap()
    } else {
        let parent_orbits = if parents.contains_key(&object) {
            1 + get_orbits(parents.get(&object).unwrap().to_string(), parents, orbits)
        } else {
            0
        };
        *orbits.entry(object).or_insert(parent_orbits)
    }
}

fn add_orbit(objects: Vec<String>, parents: &mut HashMap<String, String>) -> () {
    parents
        .insert(objects[1].clone(), objects[0].clone())
        .expect_none("Key already exists");
}

fn main() {
    let mut parents: HashMap<String, String> = HashMap::new();
    let mut orbits: HashMap<String, u32> = HashMap::new();
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(")")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|objects| add_orbit(objects, &mut parents))
        .count();
    println!(
        "{}",
        parents
            .keys()
            .map(|k| get_orbits(k.to_string(), &parents, &mut orbits))
            .fold(0, |acc, x| acc + x)
    );

    let my_parents: HashSet<String> = get_parents("YOU".to_string(), &parents);
    let santa_parents: HashSet<String> = get_parents("SAN".to_string(), &parents);
    let maximum_jumps =
        orbits.get(&"YOU".to_string()).unwrap() + orbits.get(&"SAN".to_string()).unwrap() - 2;
    let minimum_jumps = my_parents
        .intersection(&santa_parents)
        .map(|common_ancestor| maximum_jumps - 2 * orbits.get(common_ancestor).unwrap())
        .min().unwrap();
    println!("{}", minimum_jumps);
}
