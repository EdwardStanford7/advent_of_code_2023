use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn run() {
    let mut result = 0;
    let mut num_lines = 0;

    for mut line in BufReader::new(File::open("inputs/day_04_input.txt").unwrap())
        .lines()
        .map(|result| result.unwrap())
    {
        num_lines += 1;

        line = line.split_off(line.find(':').unwrap() + 2);
        let mut parts = line.split('|');

        let winning_numbers: HashSet<&str> = parts.next().unwrap().split_whitespace().collect();
        let my_numbers: Vec<&str> = parts.next().unwrap().split_whitespace().collect();

        let mut num_winning = 0;
        for number in my_numbers {
            if winning_numbers.contains(number) {
                num_winning += 1;
            }
        }

        let card_value = {
            if num_winning > 0 {
                2i32.pow(num_winning - 1)
            } else {
                0
            }
        };

        result += card_value;
    }
    println!("Part 1: {}", result);

    let mut num_copies = vec![1; num_lines];

    for (index, mut line) in BufReader::new(File::open("inputs/day_04_input.txt").unwrap())
        .lines()
        .map(|result| result.unwrap())
        .enumerate()
    {
        line = line.split_off(line.find(':').unwrap() + 2);
        let mut parts = line.split('|');

        let winning_numbers: HashSet<&str> = parts.next().unwrap().split_whitespace().collect();
        let my_numbers: Vec<&str> = parts.next().unwrap().split_whitespace().collect();

        let mut num_winning = 0;
        for number in my_numbers {
            if winning_numbers.contains(number) {
                num_winning += 1;
            }
        }

        for _ in 0..num_copies[index] {
            for i in 1..=num_winning {
                num_copies[index + i] += 1;
            }
        }
    }

    println!("Part 2: {}", num_copies.iter().sum::<i32>());
}
