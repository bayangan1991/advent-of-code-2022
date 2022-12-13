use crate::utils;
use std::collections::{HashMap, HashSet};

pub fn exec() {
    // Read the input
    let data = utils::read_input("10");
    let data: Vec<&str> = data.trim_end().split("\n").collect();
    let mut readings: HashMap<i32, i32> = HashMap::with_capacity(240);
    let mut lit_pixels: HashSet<i32> = HashSet::with_capacity(240);
    let mut part_a = 0;
    let mut signal = 1i32;
    let mut cycle = 0i32;

    for line in data {
        let mut parts = line.split(" ");

        let inst = parts.nth(0).unwrap();
        let amount: i32 = parts.nth(0).unwrap_or(&"0").parse().unwrap();

        match (inst, amount) {
            ("noop", _) => {
                if valid_signal(cycle, signal) {
                    lit_pixels.insert(cycle + 1);
                }
                cycle += 1;
                readings.insert(cycle, signal * cycle);
            }
            ("addx", amount) => {
                if valid_signal(cycle, signal) {
                    lit_pixels.insert(cycle + 1);
                }
                cycle += 1;
                readings.insert(cycle, signal * cycle);
                if valid_signal(cycle, signal) {
                    lit_pixels.insert(cycle + 1);
                }
                cycle += 1;
                readings.insert(cycle, signal * cycle);
                signal += amount;
            }
            _ => {}
        }
    }

    for i in (20i32..=220i32).step_by(40) {
        part_a += readings.get(&i).unwrap();
    }

    println!("A: {part_a}\nB:");

    for y in 0..6 {
        for x in 1..=40 {
            let index = x + (40 * y);
            match lit_pixels.get(&index) {
                None => print!("  "),
                Some(_) => print!("██"),
            };
        }
        print!("\n");
    }
}

fn valid_signal(cycle: i32, signal: i32) -> bool {
    let pos = cycle % 40;
    if signal == pos {
        return true;
    }
    if signal - 1 == pos {
        return true;
    }
    if signal + 1 == pos {
        return true;
    }

    false
}
