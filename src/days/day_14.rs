use crate::utils;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point(i32, i32);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{},{}>", self.0, self.1)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

enum Space {
    Rock,
    Sand,
}

pub fn exec() {
    // Read the input
    let data = utils::read_input("14");
    let data: Vec<&str> = data.trim_end().split("\n").collect();

    let mut my_map: HashMap<Point, Space> = HashMap::new();

    for line in data {
        let vec = parse_line(line);
        let mut rocks: Vec<Point> = Vec::new();
        for (l, r) in vec.iter().tuple_windows::<(&Point, &Point)>() {
            rocks.extend(eval_rocks(l, r));
        }
        for rock in &rocks {
            my_map.insert(rock.clone(), Space::Rock);
        }
    }

    let mut y: Vec<i32> = my_map.keys().map(|point| point.1).collect();
    y.sort();
    let max_y = y.last().unwrap();

    let mut part_a = 0u32;

    loop {
        if !simulate_sand(&mut my_map, *max_y, false) {
            break;
        } else {
            part_a += 1;
        }
    }

    let mut part_b = part_a;

    loop {
        if !simulate_sand(&mut my_map, *max_y, true) {
            break;
        } else {
            part_b += 1;
            if my_map.contains_key(&Point(500, 0)) {
                break;
            }
        }
    }

    //visualise(&my_map);

    println!("{part_a} {part_b}")
}

fn simulate_sand(map: &mut HashMap<Point, Space>, max_y: i32, part_b: bool) -> bool {
    // BOOL for came to rest
    let mut current_pos = Point(500, 0);
    loop {
        if part_b {
            if current_pos.1 > max_y {
                map.insert(current_pos.clone(), Space::Sand);
                map.insert(Point(current_pos.0, current_pos.1 + 1), Space::Rock);
                map.insert(Point(current_pos.0 - 1, current_pos.1 + 1), Space::Rock);
                map.insert(Point(current_pos.0 + 1, current_pos.1 + 1), Space::Rock);
                return true;
            }
        }
        match map.get(&Point(current_pos.0, current_pos.1 + 1)) {
            None => {
                current_pos = Point(current_pos.0, current_pos.1 + 1);
                if !part_b & (current_pos.1 > max_y) {
                    return false;
                }
            }
            Some(_) => match map.get(&Point(current_pos.0 - 1, current_pos.1 + 1)) {
                None => current_pos = Point(current_pos.0 - 1, current_pos.1 + 1),
                Some(_) => match map.get(&Point(current_pos.0 + 1, current_pos.1 + 1)) {
                    None => current_pos = Point(current_pos.0 + 1, current_pos.1 + 1),
                    Some(_) => {
                        map.insert(current_pos.clone(), Space::Sand);
                        return true;
                    }
                },
            },
        }
    }
}

fn parse_line(line: &str) -> Vec<Point> {
    let mut result = Vec::new();

    let mut parts = line.split(" -> ");

    loop {
        match parts.nth(0) {
            None => break,
            Some(part) => {
                let vec: Vec<&str> = part.split(",").collect();
                result.push(Point(vec[0].parse().unwrap(), vec[1].parse().unwrap()))
            }
        }
    }

    result
}

fn eval_rocks(left: &Point, right: &Point) -> Vec<Point> {
    let mut result = Vec::new();

    let mut x_range: Vec<i32> = (min(left.0, right.0)..=max(left.0, right.0)).collect();

    if left.0 > right.0 {
        x_range.reverse();
    }

    let mut y_range: Vec<i32> = (min(left.1, right.1)..=max(left.1, right.1)).collect();

    if left.1 > right.1 {
        y_range.reverse();
    }

    for x in &x_range {
        for y in &y_range {
            let point = Point(*x, *y);
            if !result.contains(&point) {
                result.push(point)
            }
        }
    }

    result
}

#[allow(dead_code)]
fn visualise(map: &HashMap<Point, Space>) {
    let mut x: Vec<i32> = map.keys().map(|point| point.0).collect();
    x.sort();
    let min_x = x.first().unwrap();
    let max_x = x.last().unwrap();
    let mut y: Vec<i32> = map.keys().map(|point| point.1).collect();
    y.sort();
    let min_y = y.first().unwrap();
    let max_y = y.last().unwrap();

    println!("x_range: {min_x},{max_x}");
    println!("y_range: {min_y},{max_y}");

    for y in 0..=*max_y {
        for x in *min_x..=*max_x {
            print!(
                "{}",
                match map.get(&Point(x, y)) {
                    None => "..",
                    Some(value) => match value {
                        Space::Rock => "##",
                        Space::Sand => "<>",
                    },
                }
            );
        }
        println!();
    }
}
