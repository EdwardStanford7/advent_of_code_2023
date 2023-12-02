use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn run() {
    for line in BufReader::new(File::open("inputs/day_03_input.txt").unwrap())
        .lines()
        .map(|result| result.unwrap())
    {}

    println!("Part 1:");

    for line in BufReader::new(File::open("inputs/day_03_input.txt").unwrap())
        .lines()
        .map(|result| result.unwrap())
    {}

    println!("Part 2:");
}
