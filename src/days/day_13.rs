use crate::utils;
use serde_json::{json, Value};
use std::cmp::{max, Ordering};

#[derive(Eq, Debug)]
struct Packet {
    value: Vec<Value>,
}

impl Packet {
    fn in_order(&self, other: Vec<Value>) -> bool {
        compare(self.value.clone(), other).0
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.in_order(other.value.clone()) {
            return Ordering::Less;
        }
        Ordering::Greater
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compare(left: Vec<Value>, right: Vec<Value>) -> (bool, bool) {
    for i in 0..max(left.len(), right.len()) {
        match (left.get(i), right.get(i)) {
            (Some(Value::Number(l)), Some(Value::Number(r))) => {
                //println!("Comparing => {l},{r}");
                if l.as_u64() < r.as_u64() {
                    return (true, false);
                } else if l.as_u64() > r.as_u64() {
                    return (false, false);
                }
            }
            (Some(Value::Array(l)), Some(Value::Number(r))) => {
                //println!("Morphing RIGHT: {l:?} | {r}");
                let r = vec![Value::from(r.clone())];
                let (result, do_continue) = compare(l.clone(), r);
                if do_continue {
                    continue;
                }
                return (result, false);
            }
            (Some(Value::Number(l)), Some(Value::Array(r))) => {
                //println!("Morphing LEFT: {l} | {r:?}");
                let l = vec![Value::from(l.clone())];
                let (result, do_continue) = compare(l, r.clone());
                if do_continue {
                    continue;
                }
                return (result, do_continue);
            }
            (Some(Value::Array(l)), Some(Value::Array(r))) => {
                //println!("Checking elements in array: {l:?} | {r:?}");
                let (result, do_continue) = compare(l.clone(), r.clone());
                if do_continue {
                    continue;
                }
                return (result, do_continue);
            }
            (None, Some(_)) => {
                //println!("Left side ran out: returning true");
                return (true, false);
            }
            (Some(_), None) => {
                //println!("Right side ran out: returning false");
                return (false, false);
            }
            (None, None) => {
                //println!("No elements left in each?");
                return (false, false);
            }
            (l, r) => println!("Missing pattern: {l:?}, {r:?}"),
        }
    }
    (false, true)
}

pub fn exec() {
    // Read the input
    let data = utils::read_input("13");
    let data: Vec<&str> = data.trim_end().split("\n\n").collect();
    let mut in_order: Vec<bool> = vec![false; data.len()];

    let mut all_packets: Vec<Packet> = vec![
        Packet {
            value: vec![json!([[2]])],
        },
        Packet {
            value: vec![json!([[6]])],
        },
    ];

    for (i, pair) in data.iter().enumerate() {
        let mut parts = pair.split("\n");
        let a = parts.nth(0).unwrap();
        let b = parts.nth(0).unwrap();

        let a: Value = serde_json::from_str(a).unwrap();
        let b: Value = serde_json::from_str(b).unwrap();

        let left = match a {
            Value::Number(val) => vec![Value::from(val)],
            Value::Array(val) => val,
            _ => panic!(),
        };
        let right = match b {
            Value::Number(val) => vec![Value::from(val)],
            Value::Array(val) => val,
            _ => panic!(),
        };

        all_packets.push(Packet {
            value: left.clone(),
        });
        all_packets.push(Packet {
            value: right.clone(),
        });

        let pair = Packet { value: left };

        let result = pair.in_order(right);
        //println!("{i} in order: {result}\n");
        in_order[i] = result;
    }

    all_packets.sort();

    let index_1 = all_packets
        .iter()
        .position(|p| {
            *p == Packet {
                value: vec![json!([[2]])],
            }
        })
        .unwrap()
        + 1;

    let index_2 = all_packets
        .iter()
        .position(|p| {
            *p == Packet {
                value: vec![json!([[6]])],
            }
        })
        .unwrap()
        + 1;

    let part_a: usize = in_order
        .iter()
        .enumerate()
        .map(|(i, val)| if *val { i + 1 } else { 0 })
        .sum();

    println!("{part_a} {}", index_1 * index_2)
}
