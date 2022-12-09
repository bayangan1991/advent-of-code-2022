use crate::utils;

pub fn exec() {
    // Read the input
    let data = utils::read_input("9");
    let data: Vec<&str> = data.trim_end().split("\n").collect();

    // Setup result vars
    let (mut part_a, mut part_b) = (0u32, 0u32);

    for line in data {}

    println!("{part_a} {part_b}");
}
