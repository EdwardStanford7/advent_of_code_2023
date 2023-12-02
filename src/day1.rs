use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn run() {
    let mut sum: u16 = 0;
    for line in BufReader::new(File::open("inputs/day1_input.txt").unwrap())
        .lines()
        .map(|result| result.unwrap())
    {
        let mut first_digit = "".to_string();
        let mut second_digit = " ".to_string();

        for char in line.chars() {
            if char.is_numeric() {
                if first_digit.is_empty() {
                    first_digit = char.to_string();
                }

                second_digit = char.to_string();
            }
        }

        sum += (first_digit + &second_digit).parse::<u16>().unwrap();
    }

    println!("Part 1: {}", sum);

    // Part 2
    sum = 0;
    for line in BufReader::new(File::open("inputs/day1_input.txt").unwrap())
        .lines()
        .map(|result| result.unwrap())
    {
        // Find substrings
        let substrings = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let mut substring_indices = [(-1, -1); 10];
        for (index, substring) in substrings.iter().enumerate() {
            substring_indices[index] = {
                let first = {
                    if let Some(index) = line.find(substring) {
                        index as i8
                    } else {
                        i8::MAX
                    }
                };

                let second = {
                    if let Some(index) = line.rfind(substring) {
                        index as i8
                    } else {
                        -1
                    }
                };

                (first, second)
            }
        }

        // Find digits
        let digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let mut digit_indices = [(-1, -1); 10];
        for (index, digit) in digits.iter().enumerate() {
            digit_indices[index] = {
                let first = {
                    if let Some(index) = line.find(digit) {
                        index as i8
                    } else {
                        i8::MAX
                    }
                };

                let second = {
                    if let Some(index) = line.rfind(digit) {
                        index as i8
                    } else {
                        -1
                    }
                };

                (first, second)
            }
        }

        let mut first_digit = "".to_string();
        let mut second_digit = "".to_string();

        let mut min_index = i8::MAX;
        let mut max_index = -1;

        // Figure out what the first and last numbers were.
        for i in 0..10 {
            // Substrings
            if substring_indices[i].0 < min_index {
                first_digit = i.to_string();
                min_index = substring_indices[i].0;
            }
            if substring_indices[i].1 > max_index {
                second_digit = i.to_string();
                max_index = substring_indices[i].1;
            }

            // Digits
            if digit_indices[i].0 < min_index {
                first_digit = i.to_string();
                min_index = digit_indices[i].0;
            }
            if digit_indices[i].1 > max_index {
                second_digit = i.to_string();
                max_index = digit_indices[i].1;
            }
        }

        sum += (first_digit + &second_digit).parse::<u16>().unwrap();
    }

    println!("Part 2: {}", sum);
}
