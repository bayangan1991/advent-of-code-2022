use crate::utils;
use std::collections::{HashMap, VecDeque};

pub fn exec() {
    let data = utils::read_input("7");
    let data = data.trim().split("\n");

    let mut cwd = VecDeque::<&str>::new();
    let mut sizes = HashMap::<String, u32>::new();

    cwd.push_back("/");

    for line in data {
        let parts: Vec<&str> = line.split(" ").collect();

        if parts[1] == "cd" {
            match parts[2] {
                ".." => {
                    cwd.pop_back();
                }
                "/" => {
                    cwd.clear();
                    cwd.push_back("/");
                }
                folder => {
                    cwd.push_back(folder);
                }
            }
        } else {
            match parts[0].parse::<u32>() {
                Ok(size) => {
                    let mut folder = String::new();
                    for loc in 0..cwd.len() {
                        folder += cwd[loc];
                        sizes
                            .entry(folder.to_string())
                            .and_modify(|current| *current += size)
                            .or_insert(size);
                    }
                }
                _ => {}
            }
        }
    }

    let mut part_a = 0u32;
    let mut part_b = 0u32;
    let total_size = 70000000u32;
    let required_size = 30000000u32;

    let total_used = sizes.get("/").unwrap();
    let total_available = total_size - total_used;

    for (_, size) in &sizes {
        if *size < 100_000 {
            part_a += size;
        }

        if total_available + size > required_size {
            if (part_b == 0) | (*size < part_b) {
                part_b = *size;
            }
        }
    }

    println!("{part_a} {part_b}");
}
