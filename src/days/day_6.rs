use crate::utils;
use std::collections::HashSet;

pub fn exec() {
    let data = utils::read_input("6");

    println!("{} {}", find_marker(&data, 4), find_marker(&data, 14),)
}

fn find_marker(contents: &String, size: usize) -> usize {
    let data = contents.trim();
    let mut part_set: HashSet<u8> = HashSet::with_capacity(size);

    for i in size..data.len() {
        let part = &data[i - size..i];
        part_set.extend(part.as_bytes());
        if part_set.len() == size {
            return i;
        }
        part_set.clear()
    }

    0
}
