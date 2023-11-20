use crate::utils;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Hash)]
enum Space {
    F, // Falling
    S, // Settled
    A, // Air
}

type Room = Vec<Vec<Space>>;
type Gases = VecDeque<i32>;

const PIECES: &'static [[[Space; 4]; 4]; 5] = &[
    [
        [Space::A, Space::A, Space::A, Space::A],
        [Space::A, Space::A, Space::A, Space::A],
        [Space::A, Space::A, Space::A, Space::A],
        [Space::F, Space::F, Space::F, Space::F],
    ],
    [
        [Space::A, Space::A, Space::A, Space::A],
        [Space::A, Space::F, Space::A, Space::A],
        [Space::F, Space::F, Space::F, Space::A],
        [Space::A, Space::F, Space::A, Space::A],
    ],
    [
        [Space::A, Space::A, Space::A, Space::A],
        [Space::A, Space::A, Space::F, Space::A],
        [Space::A, Space::A, Space::F, Space::A],
        [Space::F, Space::F, Space::F, Space::A],
    ],
    [
        [Space::F, Space::A, Space::A, Space::A],
        [Space::F, Space::A, Space::A, Space::A],
        [Space::F, Space::A, Space::A, Space::A],
        [Space::F, Space::A, Space::A, Space::A],
    ],
    [
        [Space::A, Space::A, Space::A, Space::A],
        [Space::A, Space::A, Space::A, Space::A],
        [Space::F, Space::F, Space::A, Space::A],
        [Space::F, Space::F, Space::A, Space::A],
    ],
];

pub fn exec() {
    let mut gases = parse_input("17");

    let mut room: Room = vec![
        vec![Space::A; 7],
        vec![Space::A; 7],
        vec![Space::A; 7],
        vec![Space::S; 7],
    ];

    let mut trimmed: u128 = 0;

    //let size: u128 = 1_000_000_000_000;
    let size: u128 = 2022;

    let mut room_states: HashMap<u64, (u128, u128)> = HashMap::new();

    let mut result = (0, 0, 0, 0);

    for i in 0..size {
        add_block(&mut room, (i % 5) as usize);
        simulate_block(&mut room, &mut gases);
        trimmed += trim_room(&mut room);
        let room_hash = hash_room(&room, i % 5);
        match room_states.get(&room_hash) {
            None => {
                room_states.insert(room_hash, (i, trimmed));
            }
            Some((j, height)) => {
                println!("Repeat {j} - {height}");
                println!("at {i} - {trimmed}");
                result = (i, *j, trimmed, *height);
                visualise(&room);
                break;
            }
        }
    }

    let block_count_per_loop = result.0 - result.1;
    let height_growth_per_loop = result.2 - result.3;

    let mut start = result.0 + 1;

    while start + block_count_per_loop < size {
        trimmed += height_growth_per_loop;
        start += block_count_per_loop;
    }

    for i in start..size {
        add_block(&mut room, (i % 5) as usize);
        simulate_block(&mut room, &mut gases);
        trimmed += trim_room(&mut room);
    }

    let part_a = room.len() as u128 + trimmed - 4;

    println!("{part_a}")
}

fn trim_room(room: &mut Room) -> u128 {
    let rows = room.len();
    let cols = room[0].len();
    let mut top = 0;
    let mut highest = vec![rows; cols];

    for y in (0..rows).rev() {
        for x in 0..cols {
            match room[y][x] {
                Space::S => highest[x] = y,
                _ => {}
            }
        }
        if room[y].iter().all(|col| match col {
            Space::A => true,
            _ => false,
        }) {
            top = y;
            break;
        }
    }

    let highest = *highest.iter().max().unwrap() + 1;

    for _ in highest..rows {
        room.remove(highest);
    }

    for _ in 0..top {
        room.remove(0);
    }

    for _ in 0..2 {
        room.insert(0, vec![Space::A; 7])
    }

    (rows - highest) as u128
}

fn add_block(room: &mut Room, block_index: usize) {
    let block = PIECES.get(block_index % 5).expect("Not a valid index!");

    for block_row in block.iter().rev() {
        let mut row = vec![Space::A; 2];
        block_row.iter().for_each(|b| row.push(*b));
        row.push(Space::A);
        if row.iter().any(|s| match s {
            Space::F => true,
            Space::S => true,
            Space::A => false,
        }) {
            room.insert(0, row);
        }
    }
}

fn simulate_block(room: &mut Room, gases: &mut Gases) {
    let rows = room.len();
    let cols = room[0].len();
    loop {
        let x_mod = gases.pop_front().expect("Ran out of elements?");
        gases.push_back(x_mod);

        if can_move(&room, 0, x_mod) {
            let mut x_range: Vec<usize> = (0..cols).collect();
            if x_mod > 0 {
                x_range.reverse();
            }
            for y in 0..rows {
                for x in &x_range {
                    match room[y][*x] {
                        Space::F => {
                            room[y][(*x as i32 + x_mod) as usize] = Space::F;
                            room[y][*x] = Space::A;
                        }
                        _ => {}
                    }
                }
            }
        }

        if can_move(&room, 1, 0) {
            for y in (1..rows).rev() {
                for x in 0..cols {
                    match room[y - 1][x] {
                        Space::F => {
                            room[y - 1][x] = Space::A;
                            room[y][x] = Space::F;
                        }
                        _ => {}
                    }
                }
            }
        } else {
            break;
        }
    }

    for y in 0..rows {
        for x in 0..cols {
            match room[y][x] {
                Space::F => room[y][x] = Space::S,
                _ => {}
            }
        }
    }
}

fn can_move(room: &Room, y_mod: i32, x_mod: i32) -> bool {
    let rows = room.len();
    let cols = room[0].len();

    for y in 0..rows {
        for x in 0..cols {
            match room[y][x] {
                Space::F => {
                    let y = (y as i32 + y_mod) as usize;
                    let x = (x as i32 + x_mod) as usize;
                    match room.get(y) {
                        None => return false,
                        Some(row) => match row.get(x) {
                            None => return false,
                            Some(space) => match space {
                                Space::S => return false,
                                _ => {}
                            },
                        },
                    }
                }
                _ => {}
            }
        }
    }

    true
}

fn hash_room(room: &Room, last_block_index: u128) -> u64 {
    let mut hasher = DefaultHasher::new();

    last_block_index.hash(&mut hasher);

    for row in room {
        let row_hash = calc_hash(row.iter());
        row_hash.hash(&mut hasher);
    }

    hasher.finish()
}

fn calc_hash<I, J>(items: I) -> u64
where
    I: Iterator<Item = J>,
    J: Hash,
{
    let mut hasher = DefaultHasher::new();
    for item in items {
        item.hash(&mut hasher);
    }

    hasher.finish()
}

fn parse_input(filename: &str) -> Gases {
    // Read the input
    let data = utils::read_input(filename);
    data.trim_end()
        .chars()
        .map(|b| if b == '<' { -1 } else { 1 })
        .collect()
}

fn visualise(room: &Room) {
    for row in room {
        print!("|");
        for col in row {
            print!(
                "{}",
                match col {
                    Space::F => "@",
                    Space::S => "#",
                    Space::A => "·",
                }
            );
        }
        println!("|");
    }
    println!("+-------+");
}
