use crate::utils;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point(i32, i32);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={}, y={}>", self.0, self.1)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Point {
    fn distance(&self, other: &Point) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }

    fn in_range(&self, loc: &Point, other: &Point) -> bool {
        self.distance(loc) <= self.distance(other)
    }

    fn edges_at_row(&self, range: i32, y: i32) -> (Point, Point) {
        (Point(self.0 - range, y), Point(self.0 + range, y))
    }

    fn points_in_row(&self, range: u32, y: i32) -> HashSet<Point> {
        let mut points: HashSet<_> = HashSet::new();

        let top_y = self.1 + range as i32;
        let bottom_y = self.1 - range as i32;

        if (bottom_y <= y) & (y <= top_y) {
            let diff = range - self.1.abs_diff(y);
            for x in self.0 - diff as i32..=self.0 + diff as i32 {
                points.insert(Point(x, y));
            }
        }
        points
    }
}

#[derive(Eq, Debug, PartialEq, Clone, Hash)]
struct Sensor {
    loc: Point,
    beacon: Point,
}

impl Sensor {
    fn points_in_row(&self, y: i32, range_bonus: Option<u32>) -> HashSet<Point> {
        let mut points = self.loc.points_in_row(
            self.loc.distance(&self.beacon) + range_bonus.unwrap_or(0),
            y,
        );
        points.remove(&self.beacon);
        points
    }

    fn range(&self) -> u32 {
        self.loc.distance(&self.beacon)
    }

    fn edges_at_row(&self, y: i32, range_bonus: Option<i32>) -> (Point, Point) {
        let range = self.range() as i32 + range_bonus.unwrap_or(0);
        let range_offset = range - self.loc.1.abs_diff(y) as i32;

        self.loc.edges_at_row(range_offset, y)
    }

    fn in_range(&self, loc: &Point) -> bool {
        self.loc.in_range(loc, &self.beacon)
    }

    fn edges(&self) -> HashSet<Point> {
        let mut edges = HashSet::new();

        let range = self.range();
        let lower = self.loc.1 - range as i32;
        let upper = self.loc.1 + range as i32;

        for y in lower - 1..=upper + 1 {
            let (l, r) = self.edges_at_row(y, Some(1));
            edges.insert(l);
            edges.insert(r);
        }

        edges
    }
}

pub fn exec() {
    let sample_part_a = part_a("15_sample", 10);
    println!("{}", sample_part_a);

    let sample_part_b = part_b("15_sample", 20);
    println!("{}", sample_part_b);

    // let part_a = part_a("15", 2000000);
    // println!("{}", part_a);
    //
    let part_b = part_b("15", 4000000);
    println!("{}", part_b);
}

fn part_a(filename: &str, target_row: i32) -> usize {
    // Read the input

    let sensors = parse_points(filename);

    let mut points_in_row: HashSet<Point> = HashSet::new();

    for sensor in sensors.values() {
        let set = sensor.points_in_row(target_row, None);
        //println!("{} = {:?}", sensor.loc, &set.len());
        points_in_row.extend(set);
    }
    visualise(&sensors, None); //, Some(&Point(8, 7)));
    points_in_row.len()
}

fn part_b(filename: &str, max_edge: i32) -> u64 {
    let sensors = parse_points(filename);

    let edges: HashSet<Point> = sensors.values().fold(HashSet::new(), |mut set, sensor| {
        set.extend(sensor.edges());
        set
    });

    let mut result_edge = Point(0, 0);

    for edge in edges.iter() {
        if (edge.0 < 0) | (edge.0 > max_edge) {
            continue;
        } else if (edge.1 < 0) | (edge.1 > max_edge) {
            continue;
        } else if sensors.values().any(|sensor| sensor.in_range(edge)) {
            continue;
        } else {
            result_edge = edge.clone();
            println!("{}", edge)
        }
    }

    let mut result = 0u64;

    match result_edge {
        Point(x, y) => {
            result += x as u64 * 4000000;
            result += y as u64;
        }
    }

    result
}

fn parse_points(filename: &str) -> HashMap<Point, Sensor> {
    let data = utils::read_input(filename);
    let data: Vec<&str> = data.trim_end().split("\n").collect();

    let mut sensors: HashMap<Point, Sensor> = HashMap::new();

    for line in data {
        let sensor = parse_line(line);
        sensors.insert(sensor.loc.clone(), sensor);
    }
    sensors
}

fn parse_line(line: &str) -> Sensor {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"x=(-?[0-9]+), y=(-?[0-9]+)").unwrap();
    }

    let mut parts: Vec<i32> = Vec::with_capacity(4);

    for part in RE.captures_iter(line) {
        parts.push(part[1].parse::<i32>().unwrap());
        parts.push(part[2].parse::<i32>().unwrap());
    }

    Sensor {
        loc: Point(parts[0], parts[1]),
        beacon: Point(parts[2], parts[3]),
    }
}

fn visualise(sensors: &HashMap<Point, Sensor>, active: Option<&Point>) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    let mut beacons: HashSet<Point> = HashSet::with_capacity(sensors.len());

    for sensor in sensors.values() {
        min_x = *[min_x, sensor.loc.0, sensor.beacon.0].iter().min().unwrap();
        min_y = *[min_y, sensor.loc.1, sensor.beacon.1].iter().min().unwrap();

        max_x = *[max_x, sensor.loc.0, sensor.beacon.0].iter().max().unwrap();
        max_y = *[max_y, sensor.loc.1, sensor.beacon.1].iter().max().unwrap();
        beacons.insert(sensor.beacon.clone());
    }

    let sensor = match active {
        None => None,
        Some(point) => sensors.get(point),
    };

    for y in (0..=9).step_by(9) {
        print!("   ");
        for x in min_x - 2..max_x + 2 {
            print!("{}", (x / (10 - y)).abs() % 10);
        }
        println!();
    }

    for y in min_y - 5..max_y + 5 {
        print!("{: >2} ", y);
        for x in min_x - 2..max_x + 2 {
            let point = Point(x, y);
            match (sensors.get(&point), beacons.get(&point)) {
                (None, None) => {
                    if sensor.is_some() {
                        let sensor = sensor.unwrap();
                        let edges = sensor.edges();
                        if sensor.in_range(&point) {
                            print!("*");
                        } else if edges.contains(&point) {
                            print!("%");
                        } else {
                            print!(".");
                        }
                    } else if sensors.values().any(|sensor| sensor.in_range(&point)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                (Some(_), None) => print!("S"),
                (None, Some(_)) => print!("B"),
                _ => {}
            }
        }
        println!();
    }
}
