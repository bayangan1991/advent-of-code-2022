use crate::utils;
use std::collections::{HashMap, HashSet};

pub fn exec() {
    // Read the input
    let data = utils::read_input("9");
    let data: Vec<&str> = data.trim_end().split("\n").collect();

    let part_a = calculate_path(&data, 2);
    let part_b = calculate_path(&data, 10);

    println!("{part_a} {part_b}");
}

fn calculate_path(data: &Vec<&str>, length: usize) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0i32, 0i32));
    let mut rope = vec![(0, 0); length];

    for line in data {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let amount = parts[1].parse::<u32>().unwrap();
        let last_index = rope.len() - 1;

        for _ in 0..amount {
            match parts[0] {
                "L" => {
                    rope[last_index] = (rope[last_index].0 - 1, rope[last_index].1);
                }
                "R" => {
                    rope[last_index] = (rope[last_index].0 + 1, rope[last_index].1);
                }
                "U" => {
                    rope[last_index] = (rope[last_index].0, rope[last_index].1 - 1);
                }
                "D" => {
                    rope[last_index] = (rope[last_index].0, rope[last_index].1 + 1);
                }
                _ => {}
            }
            for i in (0..rope.len() - 1).rev() {
                rope[i] = calculate_offset(rope[i + 1], rope[i]);

                if i == 0 {
                    visited.insert(rope[i]);
                }
            }
            //visualise(&rope, &length);
        }
    }
    visited.len()
}

fn calculate_offset(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let mut tail = tail;
    let x_diff: i32 = head.0 - tail.0;
    let x_diff_abs = x_diff.abs();
    let y_diff: i32 = tail.1 - head.1;
    let y_diff_abs = y_diff.abs();

    if (x_diff_abs > 1) & (y_diff_abs > 1) {
        if x_diff < 0 {
            tail = (tail.0 - 1, tail.1)
        } else {
            tail = (tail.0 + 1, tail.1)
        }
        if y_diff < 0 {
            tail = (tail.0, tail.1 + 1)
        } else {
            tail = (tail.0, tail.1 - 1)
        }
    } else if x_diff_abs > 1 {
        if x_diff < 0 {
            tail = (tail.0 - 1, head.1)
        } else if x_diff > 0 {
            tail = (tail.0 + 1, head.1)
        }
    } else if y_diff_abs > 1 {
        if y_diff < 0 {
            tail = (head.0, tail.1 + 1)
        } else if y_diff > 0 {
            tail = (head.0, tail.1 - 1)
        }
    }
    tail
}

#[allow(dead_code)]
fn visualise(rope: &Vec<(i32, i32)>, length: &usize) {
    let size: i32 = 10;
    let rope_lookup: HashMap<&(i32, i32), usize> = rope
        .iter()
        .enumerate()
        .map(|(k, v)| (v, length - k))
        .collect();
    for y in -size..size {
        for x in -size..size {
            match rope_lookup.get(&(x, y)) {
                None => {
                    print!(".")
                }
                Some(index) => match index {
                    1 => print!("H"),
                    other => print!("{}", other - 1),
                },
            }
        }
        print!("\n")
    }
    print!("\n")
}
