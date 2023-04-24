extern crate core;

use std::env;

mod challenges;

fn main() {
    println!("Welcome to the advent of code 2022 !");
    let day = parse_args();
    match day.as_str() {
        "day1" => challenges::day1::run(),
        "day2" => challenges::day2::run(),
        _ => println!("Module not found, are you sure you are running a valid challenge?")
    }
}

fn parse_args() -> String {
    let err = "You must provide the day argument. Example: ./advent-of-code-2022 day1";
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("{}", err)
    }
    let day: String = args[1].clone();
    if !day.contains("day") {
        panic!("{}", err);
    }

    day
}