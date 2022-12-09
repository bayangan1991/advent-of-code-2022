use crate::utils;

pub fn exec() {
    let data = utils::read_input("1");

    let mut elves: Vec<i32> = Vec::new();

    let lines = data.split("\n");

    let mut current_elf: i32 = 0;
    let mut elf_max: i32 = 0;

    for line in lines {
        if line == "" {
            elves.push(current_elf);
            if current_elf > elf_max {
                elf_max = current_elf;
            }
            current_elf = 0;
        } else {
            current_elf += line.parse::<i32>().unwrap();
        }
    }

    elves.sort();

    let last = elves.pop().unwrap();
    let last2 = elves.pop().unwrap();
    let last3 = elves.pop().unwrap();

    println!("{} {}", last, last + last2 + last3);
}
