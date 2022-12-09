use crate::utils;

pub fn exec() {
    let data = utils::read_input("4");

    let lines = data.split("\n");

    let mut part_a: u32 = 0;
    let mut part_b: u32 = 0;

    for line in lines {
        let mut pair = line.split(",");

        let left = &get_range(pair.nth(0).unwrap());
        let right = &get_range(pair.nth(0).unwrap());

        let left_contains = right.iter().any(|i| left.contains(i));
        let right_contains = left.iter().any(|i| right.contains(i));

        if left_contains | right_contains {
            part_b += 1;
        }

        if is_subset(left, right) | is_subset(right, left) {
            part_a += 1;
        }
    }

    println!("{part_a} {part_b}");
}

fn get_range(elf: &str) -> Vec<u32> {
    let mut ranges = elf.split("-");
    let lower = ranges.nth(0).unwrap().parse::<u32>().unwrap();
    let upper = ranges.nth(0).unwrap().parse::<u32>().unwrap();
    return (lower..=upper).collect::<Vec<u32>>();
}

fn is_subset(a: &Vec<u32>, b: &Vec<u32>) -> bool {
    if a.iter().min() >= b.iter().min() {
        if a.iter().max() <= b.iter().max() {
            return true;
        }
    }
    return false;
}
