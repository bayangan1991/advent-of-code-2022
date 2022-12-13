use crate::utils;
use std::cmp::min;
use std::collections::{HashMap, HashSet, VecDeque};

type Map = Vec<Vec<u8>>;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Node {
    loc: Point,
    neighbours: Vec<Point>,
}

pub fn exec() {
    // Read the input
    let data = utils::read_input("12");
    let data: Vec<&str> = data.trim_end().split("\n").collect();

    let mut data: Map = data.iter().map(|&row| row.bytes().collect()).collect();

    let mut nodes: HashMap<Point, Node> = HashMap::new();

    let mut other_starts: Vec<Point> = Vec::new();

    let mut end = Point { x: 0, y: 0 };
    let mut start = Point { x: 0, y: 0 };

    for (y, row) in data.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            match tile {
                b'S' => {
                    start = Point { x, y };
                }
                b'E' => {
                    end = Point { x, y };
                }
                b'a' => other_starts.push(Point { x, y }),
                &_ => {}
            }
        }
    }

    data[start.y][start.x] = b'a';
    data[end.y][end.x] = b'z';

    for (y, row) in data.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            nodes.insert(Point { x, y }, build_graph(&data, x, y));
        }
    }

    let graph = solve(&nodes, &end);

    let part_a = find_path_length(&graph, &start);

    let mut part_b = 9999u32;

    for other_start in other_starts {
        let new_len = find_path_length(&graph, &other_start);
        if new_len > 0 {
            part_b = min(part_b, new_len);
        }
    }

    println!("{part_a} {part_b}")
}

fn find_path_length(graph: &HashMap<Point, Point>, end: &Point) -> u32 {
    let mut length = 0u32;

    let mut current = end.clone();

    loop {
        match graph.get(&current) {
            None => break,
            Some(point) => {
                length += 1;
                current = point.clone();
            }
        }
    }

    length
}

fn solve(nodes: &HashMap<Point, Node>, start: &Point) -> HashMap<Point, Point> {
    let mut queue: VecDeque<Box<&Node>> = VecDeque::new();
    let mut visited: HashSet<&Point> = HashSet::new();
    let mut previous: HashMap<Point, Point> = HashMap::new();

    queue.push_front(Box::from(nodes.get(&start).unwrap()));
    visited.insert(&start);

    while let Some(node) = queue.pop_front() {
        for neighbour in node.neighbours.iter() {
            if !visited.contains(&neighbour) {
                queue.push_back(Box::from(nodes.get(&neighbour).unwrap()));
                visited.insert(&neighbour);
                previous.insert(neighbour.clone(), node.loc.clone());
            }
        }
    }

    previous
}

fn build_graph(map: &Map, x: usize, y: usize) -> Node {
    let height = map.len();
    let width = map[0].len();

    let tile = map[y][x];

    let mut node = Node {
        loc: Point { x, y },
        neighbours: Vec::new(),
    };

    if x > 0 {
        let diff = map[y][x - 1] as i32 - tile as i32;
        if diff >= -1 {
            node.neighbours.push(Point { x: x - 1, y })
        }
    }
    if x < width - 1 {
        let diff = map[y][x + 1] as i32 - tile as i32;
        if diff >= -1 {
            node.neighbours.push(Point { x: x + 1, y })
        }
    }
    if y > 0 {
        let diff = map[y - 1][x] as i32 - tile as i32;
        if diff >= -1 {
            node.neighbours.push(Point { x, y: y - 1 })
        }
    }
    if y < height - 1 {
        let diff = map[y + 1][x] as i32 - tile as i32;
        if diff >= -1 {
            node.neighbours.push(Point { x, y: y + 1 })
        }
    }

    node
}
