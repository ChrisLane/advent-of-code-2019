#[macro_use]
extern crate clap;

use std::io::prelude::*;
use std::io::stdin;
use std::fs::File;
use clap::{App, Arg};

mod day_01;

fn main() -> std::io::Result<()> {
    let possible_days: Vec<String> = (1..24).map(|x| x.to_string()).collect();
    let possible_days: Vec<&str> = possible_days.iter().map(|x| x.as_str()).collect();
    let matches = App::new("Advent of Code 2019 Solutions")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Runs solutions for the Advent of Code 2019 puzzles")
        .arg(Arg::with_name("DAY")
            .help("Sets the day to solve puzzles for")
            .required(true)
            .possible_values(&possible_days)
            .index(1))
        .arg(Arg::with_name("PART")
            .help("Sets the part of the puzzle to solve")
            .required(true)
            .possible_values(&["1", "2"])
            .index(2))
        .arg(Arg::with_name("FILE")
            .help("The input file for the puzzle")
            .index(3))
        .get_matches();

    let day = matches.value_of("DAY").unwrap().parse().expect("Invalid DAY");
    let part = matches.value_of("PART").unwrap().parse().expect("Invalid PART");
    let mut input = String::new();

    if let Some(path) = matches.value_of("FILE") {
        let mut file = File::open(path)?;
        file.read_to_string(&mut input)?;
    } else {
        stdin().read_to_string(&mut input)?;
    }

    if let Some(result) = solve(day, part, input) {
        println!("{}", result)
    } else {
        println!("A solution for day {} part {} has not been implemented yet", day, part)
    }
    Ok(())
}

fn solve(day: u8, part: u8, input: String) -> Option<String> {
    if let Some(solver) = get_solver(day, part) {
        Some(solver(&input))
    } else {
        None
    }
}

fn get_solver(day: u8, part: u8) -> Option<fn(&str) -> String> {
    let parts = match day {
        1 => (day_01::part1, day_01::part2),
        _ => return None
    };

    match part {
        1 => Some(parts.0),
        2 => Some(parts.1),
        _ => None
    }
}
