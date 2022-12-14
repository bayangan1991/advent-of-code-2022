extern crate core;

use std::env;

mod days;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let run_all = args.len() == 1;
    if run_all | args.contains(&"1".to_string()) {
        days::day_1::exec();
    }
    if run_all | args.contains(&"2".to_string()) {
        days::day_2::exec();
    }
    if run_all | args.contains(&"3".to_string()) {
        days::day_3::exec();
    }
    if run_all | args.contains(&"4".to_string()) {
        days::day_4::exec();
    }
    if run_all | args.contains(&"5".to_string()) {
        days::day_5::exec();
    }
    if run_all | args.contains(&"6".to_string()) {
        days::day_6::exec();
    }
    if run_all | args.contains(&"7".to_string()) {
        days::day_7::exec();
    }
    if run_all | args.contains(&"8".to_string()) {
        days::day_8::exec();
    }
    if run_all | args.contains(&"9".to_string()) {
        days::day_9::exec();
    }
    if run_all | args.contains(&"10".to_string()) {
        days::day_10::exec();
    }
    if run_all | args.contains(&"11".to_string()) {
        days::day_11::exec();
    }
    if run_all | args.contains(&"12".to_string()) {
        days::day_12::exec();
    }
}
