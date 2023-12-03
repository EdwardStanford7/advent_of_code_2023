use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn run() {
    // Read input
    let mut engine_schematic: Vec<Vec<char>> = Vec::new();
    for line in BufReader::new(File::open("inputs/day_03_input.txt").unwrap())
        .lines()
        .map(|result| result.unwrap())
    {
        engine_schematic.push(line.chars().collect());
    }

    let mut part_2_schematic = engine_schematic.clone();

    // Part 1
    let mut result = 0;
    for row in 0..engine_schematic.len() {
        for column in 0..engine_schematic[row].len() {
            if !engine_schematic[row][column].is_numeric() && engine_schematic[row][column] != '.' {
                result += check_surrounding(&mut engine_schematic, row, column);
            }
        }
    }
    println!("Part 1: {}", result);

    // Part 2
    result = 0;
    for row in 0..part_2_schematic.len() {
        for column in 0..part_2_schematic[row].len() {
            if part_2_schematic[row][column] == '*' {
                result += check_gear(&mut part_2_schematic, row, column);
            }
        }
    }
    println!("Part 2: {}", result);
}

fn check_surrounding(engine_schematic: &mut [Vec<char>], y: usize, x: usize) -> u32 {
    let mut sum = 0;

    for y in (y - 1)..=(y + 1) {
        for x in (x - 1)..=(x + 1) {
            if engine_schematic[y][x].is_numeric() {
                sum += get_number(engine_schematic, y, x);
            }
        }
    }

    sum
}

fn check_gear(engine_schematic: &mut [Vec<char>], y: usize, x: usize) -> u32 {
    let mut numbers_found: Vec<u32> = Vec::new();

    for y in (y - 1)..=(y + 1) {
        for x in (x - 1)..=(x + 1) {
            if engine_schematic[y][x].is_numeric() {
                numbers_found.push(get_number(engine_schematic, y, x));
            }
        }
    }

    if numbers_found.len() == 2 {
        numbers_found[0] * numbers_found[1]
    } else {
        0
    }
}

fn get_number(engine_schematic: &mut [Vec<char>], y: usize, x: usize) -> u32 {
    let mut digits: Vec<char> = [engine_schematic[y][x]].to_vec();
    engine_schematic[y][x] = '.';

    // Check left for digits
    let mut current_column = x - 1;
    while engine_schematic[y][current_column].is_numeric() {
        digits.insert(0, engine_schematic[y][current_column]);
        engine_schematic[y][current_column] = '.';

        if current_column == 0 {
            break;
        }

        current_column -= 1;
    }

    // Check right for digits
    current_column = x + 1;
    while engine_schematic[y][current_column].is_numeric() {
        digits.push(engine_schematic[y][current_column]);
        engine_schematic[y][current_column] = '.';

        current_column += 1;

        if current_column == engine_schematic[y].len() {
            break;
        }
    }

    digits.into_iter().collect::<String>().parse().unwrap()
}
