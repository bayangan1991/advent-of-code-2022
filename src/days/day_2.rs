use crate::utils;

pub fn exec() {
    let data = utils::read_input("2");

    let mut part_a: i32 = 0;
    let mut part_b: i32 = 0;
    let wins = ["A Y", "B Z", "C X"];
    let draws = ["A X", "B Y", "C Z"];

    let lines = data.split("\n");

    for line in lines {
        let win1 = wins.contains(&line);
        let draw1 = draws.contains(&line);
        let opponent = line.chars().nth(0).unwrap();
        let thrown = line.chars().nth(2).unwrap();

        if win1 {
            part_a += 6;
        }

        if draw1 {
            part_a += 3;
        }

        if thrown == 'X' {
            part_a += 1;
        } else if thrown == 'Y' {
            part_a += 2;
        } else if thrown == 'Z' {
            part_a += 3;
        }

        let win2 = thrown == 'Z';
        let draw2 = thrown == 'Y';

        if win2 {
            part_b += 6;
        }

        if draw2 {
            part_b += 3;
        }

        if opponent == 'A' {
            // rock > paper
            if draw2 {
                part_b += 1;
            } else if win2 {
                part_b += 2;
            } else {
                part_b += 3;
            }
        } else if opponent == 'B' {
            // paper > scissors
            if draw2 {
                part_b += 2;
            } else if win2 {
                part_b += 3;
            } else {
                part_b += 1;
            }
        } else if opponent == 'C' {
            // scissors > rock
            if draw2 {
                part_b += 3;
            } else if win2 {
                part_b += 1;
            } else {
                part_b += 2;
            }
        }
    }

    println!("{part_a} {part_b}");
}
