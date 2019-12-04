use std::io;
use std::io::prelude::*;

fn get_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel < 0 {
        0
    } else {
        fuel + get_fuel(fuel)
    }
}

fn main() {
    println!(
        "{}",
        io::stdin()
            .lock()
            .lines()
            .map(|s| s.unwrap().parse::<i32>().unwrap())
            .map(|i| get_fuel(i))
            .fold(0, |acc, x| acc + x)
    );
}
