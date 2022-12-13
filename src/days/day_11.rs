use crate::utils;
use std::collections::HashMap;

#[derive(Clone)]
struct Monkey {
    items: Vec<i128>,
    op: String,
    op_amount: String,
    test: i128,
    true_target: usize,
    false_target: usize,
    inspections: u64,
}

impl Monkey {
    fn inspect_items(&mut self, divisor: i128, part_a: bool) -> HashMap<usize, Vec<i128>> {
        let mut result: HashMap<usize, Vec<i128>> = HashMap::new();

        let op = match self.op.as_str() {
            "*" => |a: i128, b: i128| a * b,
            "+" => |a, b| a + b,
            _ => panic!("Invalid op!"),
        };

        loop {
            let mut item = match self.items.pop() {
                None => break,
                Some(item) => match self.op_amount.parse::<i128>() {
                    Ok(amount) => op(item, amount),
                    Err(_) => op(item, item),
                },
            };

            if part_a {
                item = item / 3;
            }

            item %= divisor;

            self.inspections += 1;

            if item % self.test == 0 {
                match result.get_mut(&self.true_target) {
                    None => {
                        result.insert(self.true_target, vec![item]);
                    }
                    Some(bucket) => {
                        bucket.push(item);
                    }
                }
            } else {
                match result.get_mut(&self.false_target) {
                    None => {
                        result.insert(self.false_target, vec![item]);
                    }
                    Some(bucket) => {
                        bucket.push(item);
                    }
                }
            }
        }

        result
    }
}

pub fn exec() {
    // Read the input
    let data = utils::read_input("11");
    let raw_monkeys: Vec<&str> = data.trim_end().split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = Vec::with_capacity(8);

    for monkey in raw_monkeys {
        monkeys.push(parse_monkey(monkey))
    }

    let mut monkeys2 = monkeys.clone();

    let divisor: i128 = monkeys.iter().map(|monkey| monkey.test).product();

    let inspections_a = monkey_business(&mut monkeys, divisor, true);
    let part_a = inspections_a[0] * inspections_a[1];

    let inspections_b = monkey_business(&mut monkeys2, divisor, false);
    let part_b = inspections_b[0] * inspections_b[1];

    println!("{part_a} {part_b}")
}

fn monkey_business(monkeys: &mut Vec<Monkey>, divisor: i128, part_a: bool) -> Vec<u64> {
    for _ in 0..(if part_a { 20 } else { 10000 }) {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            let inspections = monkey.inspect_items(divisor, part_a);

            for (index, items) in inspections {
                monkeys.get_mut(index).unwrap().items.extend(items);
            }
        }
    }

    let mut inspection_counts: Vec<u64> = monkeys.iter().map(|monkey| monkey.inspections).collect();

    inspection_counts.sort();
    inspection_counts.reverse();
    inspection_counts
}

fn parse_monkey(monkey: &str) -> Monkey {
    let mut parts = monkey.split("\n");

    let _ = parts.nth(0).unwrap().to_string();

    let items = parts.nth(0).unwrap();
    let mut items = items.split(": ");
    let items = items.nth(1).unwrap();
    let items: Vec<i128> = items
        .split(", ")
        .into_iter()
        .map(|i| i.parse::<i128>().unwrap())
        .collect();

    let ops = parts.nth(0).unwrap();
    let mut ops: Vec<&str> = ops.split(" ").collect();
    ops.reverse();
    let op = ops[1].to_string();
    let op_amount = ops[0].to_string();

    let test = parts.nth(0).unwrap().split(" ");
    let test: i128 = test.last().unwrap().parse().unwrap();

    let true_target = parts.nth(0).unwrap().split(" ");
    let true_target: usize = true_target.last().unwrap().parse().unwrap();

    let false_target = parts.nth(0).unwrap().split(" ");
    let false_target: usize = false_target.last().unwrap().parse().unwrap();

    Monkey {
        items,
        op,
        op_amount,
        test,
        true_target,
        false_target,
        inspections: 0,
    }
}
