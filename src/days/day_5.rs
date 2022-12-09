use crate::utils;
use std::collections::VecDeque;

pub fn exec() {
    let data = utils::read_input("5");

    let mut parts = data.split("\n\n");

    let mut raw_crates = parts.nth(0).unwrap().split("\n").collect::<Vec<&str>>();
    let instructions = parts.nth(0).unwrap().split("\n");

    let mut crates_a: Vec<VecDeque<char>> = Vec::with_capacity(9);

    let columns = raw_crates
        .pop()
        .unwrap()
        .trim()
        .split("   ")
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    for _ in 0..columns {
        crates_a.push(VecDeque::with_capacity(10))
    }

    for line in raw_crates {
        for i in (0..line.len()).step_by(4) {
            let box_letter = char::from(line.as_bytes()[i + 1]);
            if box_letter != ' ' {
                crates_a[i / 4].push_back(box_letter)
            }
        }
    }

    let mut crates_b = crates_a.clone();
    let mut crane: VecDeque<char> = VecDeque::new();

    for line in instructions {
        if line == "" {
            continue;
        }
        let (amount, from, to) = get_instructions(line);

        for _ in 0..amount {
            let x = crates_a[from - 1].pop_front().unwrap();
            crates_a[to - 1].push_front(x)
        }

        for _ in 0..amount {
            let x = crates_b[from - 1].pop_front().unwrap();
            crane.push_front(x)
        }

        while let Some(item) = crane.pop_front() {
            crates_b[to - 1].push_front(item);
        }
    }

    let mut part_a = String::with_capacity(9);
    let mut part_b = String::with_capacity(9);

    for column in crates_a {
        part_a += &column.front().unwrap().to_string();
    }
    for column in crates_b {
        part_b += &column.front().unwrap().to_string();
    }

    println!("{part_a} {part_b}");
}

fn get_instructions(line: &str) -> (usize, usize, usize) {
    let parts = line.split(" ").collect::<Vec<&str>>();

    let a = parts[1].parse::<usize>().unwrap();
    let b = parts[3].parse::<usize>().unwrap();
    let c = parts[5].parse::<usize>().unwrap();

    (a, b, c)
}
