use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn run() {
    // Read input
    let mut games: Vec<Game> = Vec::new();
    for (index, mut line) in BufReader::new(File::open("inputs/day_02_input.txt").unwrap())
        .lines()
        .map(|result| result.unwrap())
        .enumerate()
    {
        line = line.split_off(line.find(':').unwrap() + 2);

        let mut game = Game::new((index + 1).try_into().unwrap());
        for reveal in line.split(';') {
            let mut num_colors = [0, 0, 0];

            for cubes in reveal.split(", ") {
                let parts: Vec<&str> = cubes.split_whitespace().collect();

                match parts[1] {
                    "red" => {
                        num_colors[0] = parts[0].parse::<u8>().unwrap();
                    }
                    "green" => {
                        num_colors[1] = parts[0].parse::<u8>().unwrap();
                    }
                    "blue" => {
                        num_colors[2] = parts[0].parse::<u8>().unwrap();
                    }
                    _ => {}
                }
            }

            game.reveals.push(num_colors.into());
        }
        games.push(game);
    }

    // Part 1
    let mut result = 0;
    'outer: for game in &games {
        for reveal in &game.reveals {
            if reveal.0 > 12 || reveal.1 > 13 || reveal.2 > 14 {
                continue 'outer;
            }
        }
        result += game.id as u32;
    }
    println!("Part 1: {result}");

    // Part 2
    result = 0;
    for game in &games {
        let mut min_reds = 0;
        let mut min_greens = 0;
        let mut min_blues = 0;

        for reveal in &game.reveals {
            if reveal.0 > min_reds {
                min_reds = reveal.0;
            }
            if reveal.1 > min_greens {
                min_greens = reveal.1;
            }
            if reveal.2 > min_blues {
                min_blues = reveal.2;
            }
        }

        result += min_reds as u32 * min_greens as u32 * min_blues as u32;
    }
    println!("Part 2: {}", result);
}

#[derive(Debug)]
struct Game {
    id: u8,
    reveals: Vec<(u8, u8, u8)>, // Number of red, green, blue cubes.
}

impl Game {
    fn new(id: u8) -> Game {
        Game {
            id: (id),
            reveals: (Vec::new()),
        }
    }
}
