use crate::utils;

pub fn exec() {
    let data = utils::read_input("3");

    let lines = data.split("\n");

    let mut part_a = 0;
    let mut part_b = 0;

    let mut group: Vec<&str> = Vec::new();

    for line in lines {
        if group.len() == 2 {
            part_b += get_badge(line, group);
            group = Vec::new();
        } else {
            group.push(line);
        }

        let line_len = line.len() / 2;

        let left = &line[..line_len];
        let right = &line[line_len..];

        for item in left.chars() {
            if right.contains(&item.to_string()) {
                let score = get_score(item);
                part_a += score;
                break;
            }
        }
    }

    println!("{part_a} {part_b}");
}

fn get_score(item: char) -> u32 {
    let mut value: u32 = u32::from(item.to_ascii_lowercase()) - 96;
    if item.is_uppercase() {
        value += 26;
    }
    return value;
}

fn get_badge(line: &str, others: Vec<&str>) -> u32 {
    for char in line.chars() {
        let check = others.iter().all(|line| line.contains(&char.to_string()));
        if check {
            return get_score(char);
        }
    }

    return 0;
}
