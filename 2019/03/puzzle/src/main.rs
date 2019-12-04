extern crate num;

use std::cmp::min;
use std::io;
use std::io::prelude::*;

fn distance_between_points((x1, y1): (i16, i16), (x2, y2): (i16, i16)) -> u32 {
    (num::abs(x2 - x1) + num::abs(y2 - y1)) as u32
}

fn intersection(l1: &Line, l2: &Line) -> Option<(u32, u32)> {
    let (l1, l2) = if l1.x1 > l2.x1 { (l2, l1) } else { (l1, l2) };

    if l2.x1 >= l1.x1 && l2.x1 <= l1.x2 {
        // X projections overlap
        let x1 = l2.x1;
        let x2 = min(l2.x2, l1.x2);
        let x = if num::abs(x1) < num::abs(x2) { x1 } else { x2 };

        let (l1, l2) = if l1.y1 > l2.y1 { (l2, l1) } else { (l1, l2) };

        if l2.y1 >= l1.y1 && l2.y1 <= l1.y2 {
            // Y projections overlap
            let y1 = l2.y1;
            let y2 = min(l2.y2, l1.y2);

            let y = if num::abs(y1) < num::abs(y2) { y1 } else { y2 };

            if x == 0 && y == 0 {
                return None;
            };

            let mut steps = l1.steps + l2.steps;
            steps = steps
                + distance_between_points(
                    (x, y),
                    match l1.d {
                        Direction::L => (l1.x2, l1.y2),
                        Direction::R => (l1.x1, l1.y1),
                        Direction::U => (l1.x1, l1.y1),
                        Direction::D => (l1.x2, l1.y2),
                    },
                );
            steps = steps
                + distance_between_points(
                    (x, y),
                    match l2.d {
                        Direction::L => (l2.x2, l2.y2),
                        Direction::R => (l2.x1, l2.y1),
                        Direction::U => (l2.x1, l2.y1),
                        Direction::D => (l2.x2, l2.y2),
                    },
                );
            Some((steps, distance_between_points((0, 0), (x, y))))
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Debug)]
enum Direction {
    R,
    L,
    D,
    U,
}

#[derive(Debug)]
struct Line {
    x1: i16,
    y1: i16,
    x2: i16,
    y2: i16,
    d: Direction,
    steps: u32,
}

fn main() {
    let wires: Vec<Vec<Line>> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let mut x: i16 = 0;
            let mut y: i16 = 0;
            let mut total_steps: u32 = 0;
            line.unwrap()
                .split(',')
                .map(|s| {
                    let steps = s[1..].parse::<u16>().unwrap();
                    let (new_x, new_y, line) = match s.chars().nth(0).unwrap() {
                        'R' => (
                            x + steps as i16,
                            y,
                            Line {
                                x1: x,
                                y1: y,
                                x2: x + steps as i16,
                                y2: y,
                                d: Direction::R,
                                steps: total_steps,
                            },
                        ),
                        'L' => (
                            x - steps as i16,
                            y,
                            Line {
                                x1: x - steps as i16,
                                y1: y,
                                x2: x,
                                y2: y,
                                d: Direction::L,
                                steps: total_steps,
                            },
                        ),
                        'D' => (
                            x,
                            y - steps as i16,
                            Line {
                                x1: x,
                                y1: y - steps as i16,
                                x2: x,
                                y2: y,
                                d: Direction::D,
                                steps: total_steps,
                            },
                        ),
                        'U' => (
                            x,
                            y + steps as i16,
                            Line {
                                x1: x,
                                y1: y,
                                x2: x,
                                y2: y + steps as i16,
                                d: Direction::U,
                                steps: total_steps,
                            },
                        ),
                        _ => panic!(),
                    };
                    x = new_x;
                    y = new_y;
                    total_steps = total_steps + steps as u32;
                    line
                })
                .collect::<Vec<Line>>()
        })
        .collect();

    let mut minimum_distance: Option<u32> = None;
    let mut minimum_steps: Option<u32> = None;
    for l1 in wires[0].iter() {
        for l2 in wires[1].iter() {
            match intersection(&l1, &l2) {
                Some((steps, distance)) => {
                    minimum_steps = Some(min(minimum_steps.unwrap_or(steps), steps));
                    minimum_distance = Some(min(minimum_distance.unwrap_or(distance), distance));
                }
                None => {}
            }
        }
    }

    println!(
        "Minimum steps: {:?} minimum distance: {:?}",
        minimum_steps, minimum_distance
    );
}
