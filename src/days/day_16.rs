use crate::utils;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

type Cache = HashMap<(usize, u64, usize), (usize, HashSet<usize>)>;

#[derive(Debug, Clone)]
struct ValveTemp {
    name: String,
    flow_rate: usize,
    neighbours: Vec<String>,
}

struct Valve {
    flow_rate: usize,
    neighbours: Vec<usize>,
}

type Graph = HashMap<usize, Valve>;

pub fn exec() {
    // Read the input
    let data = utils::read_input("16_sample");
    let data: Vec<&str> = data.trim_end().split("\n").collect();

    // Parse data
    let mut valves: Vec<ValveTemp> = Vec::new();

    for line in data {
        let valve = parse_line(line);
        valves.push(valve);
    }

    let index_map: HashMap<&str, usize> = valves
        .iter()
        .enumerate()
        .map(|(i, valve)| (valve.name.as_str(), i))
        .collect();

    let reversed = index_map.iter().map(|(a, b)| (*b, *a)).collect();

    let valves: Graph = valves
        .iter()
        .map(|valve| {
            (
                *index_map.get(valve.name.as_str()).unwrap(),
                Valve {
                    flow_rate: valve.flow_rate,
                    neighbours: valve
                        .neighbours
                        .iter()
                        .map(|n| *index_map.get(n.as_str()).unwrap())
                        .collect::<Vec<_>>(),
                },
            )
        })
        .collect();

    let part_a = dfs(&valves, *index_map.get("AA").unwrap(), 30, false, &reversed);
    let part_b = dfs(&valves, *index_map.get("AA").unwrap(), 26, true, &reversed);

    println!("{} {}", part_a, part_b);
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

fn dfs(
    graph: &Graph,
    start: usize,
    total_mins: usize,
    elephant: bool,
    lookup: &HashMap<usize, &str>,
) -> usize {
    fn walk(
        node: usize,
        graph: &Graph,
        opened: &HashSet<usize>,
        mins_left: usize,
        cache: &mut Cache,
    ) -> (usize, HashSet<usize>) {
        if mins_left <= 0 {
            return (0, opened.clone());
        }

        let opened_key = calc_hash(opened.iter());
        let key = (node, opened_key, mins_left);

        match cache.get(&key) {
            None => {}
            Some((best_release, best_opened)) => {
                return (*best_release, best_opened.clone());
            }
        }

        let mut best_released = 0;

        let mut loc_opened = opened.clone();
        loc_opened.insert(node);
        let mut released = 0;
        let valve = graph.get(&node).unwrap();
        if !opened.contains(&node) {
            released += (mins_left - 1) * valve.flow_rate;
        }

        let mut new_opened: HashSet<usize> = HashSet::new();

        for n in &valve.neighbours {
            if released > 0 {
                let (a_release, a_opened) = walk(*n, &graph, &loc_opened, mins_left - 2, cache);
                if (released + a_release) > best_released {
                    best_released = released + a_release;
                    new_opened = a_opened;
                }
            }
            let (b_release, b_opened) = walk(*n, &graph, &opened, mins_left - 1, cache);
            if b_release > best_released {
                best_released = b_release;
                new_opened = b_opened;
            }
        }

        let mut opened = opened.clone();
        opened.extend(new_opened);

        cache.insert(key, (best_released, opened.clone()));

        (best_released, opened)
    }

    let mut cache = Cache::new();

    let total_mins = total_mins - if elephant { 4 } else { 0 };

    let walk1 = walk(start, &graph, &HashSet::new(), total_mins, &mut cache);

    if elephant {
        print_opened(&walk1.1, lookup);
        let walk2 = walk(start, &graph, &walk1.1, total_mins, &mut cache);
        print_opened(&walk2.1, lookup);

        return walk1.0 + walk2.0;
    }
    walk1.0
}

fn print_opened(opened: &HashSet<usize>, lookup: &HashMap<usize, &str>) {
    println!(
        "{:?}",
        opened
            .iter()
            .map(|a| lookup.get(a).unwrap())
            .collect::<HashSet<&&str>>()
    );
}

fn parse_line(line: &str) -> ValveTemp {
    let parts: Vec<&str> = line.split(" ").collect();

    let rate = parts[4];
    let rate_l = rate.find("=").unwrap() as usize;
    let rate_r = rate.find(";").unwrap() as usize;
    let rate = rate.chars().collect::<Vec<char>>()[rate_l + 1..rate_r]
        .into_iter()
        .collect::<String>();
    let flow_rate: usize = rate.parse().unwrap();

    let neighbours: Vec<String> = parts[9..]
        .join(" ")
        .split(", ")
        .map(|valve| valve.to_string())
        .collect();

    ValveTemp {
        name: parts[1].to_string(),
        flow_rate,
        neighbours,
    }
}
