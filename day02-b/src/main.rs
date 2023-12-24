use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use regex::Regex;

/**
 * Holds information about the number of cubes and the colour of the cubes.
 */
struct CubeAssignment {
    number_of_cubes: u32,
    cube_colour: String,
}

/**
 * Holds the cube assignments for the rounds in the game.
 */
struct RoundInformation {
    round_number: u32,
    cube_assignments: Vec<CubeAssignment>,
}

/**
 * Holds information about the game.
 */
struct GameInfo {
    game_number: u32,
    rounds: Vec<RoundInformation>,
}

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = std::env::args().collect();

    let input_file = &args[1];
    let mut sum_of_powers: u32 = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(str) = line {
                let game_info = split_line_into_usable_information(String::from(str));


                let mut highest_blue_cube_count = 0;
                let mut highest_green_cube_count = 0;
                let mut highest_red_cube_count = 0;

                for round in game_info.rounds {
                    for cube_assignment in round.cube_assignments {
                        if cube_assignment.cube_colour == "red" {
                            if cube_assignment.number_of_cubes > highest_red_cube_count {
                                highest_red_cube_count = cube_assignment.number_of_cubes;
                            }
                        } else if cube_assignment.cube_colour == "green" {
                            if cube_assignment.number_of_cubes > highest_green_cube_count {
                                highest_green_cube_count = cube_assignment.number_of_cubes;
                            }
                        } else if cube_assignment.cube_colour == "blue" {
                            if cube_assignment.number_of_cubes > highest_blue_cube_count {
                                highest_blue_cube_count = cube_assignment.number_of_cubes;
                            }
                        }
                    }
                }

                let power_value = highest_blue_cube_count * highest_green_cube_count * highest_red_cube_count;

                println!("Game: {}, Power: {}", game_info.game_number, power_value);

                sum_of_powers += power_value;
            }
        }
    }
    println!("Sum of powers: {}", sum_of_powers);
    println!("Program finished executing in : {:?}", start_time.elapsed());
}

/**
 * Reads the lines from a file and returns an iterator over the lines.
 */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


/**
 * parses the line information into a Struct with usable information.
 */
fn split_line_into_usable_information(line: String)-> GameInfo {
    let mut game_info = GameInfo {
        game_number: 0,
        rounds: Vec::new(),
    };

    let game_number_regex = Regex::new(r"Game (?<game_number>\d+):").unwrap();
    let round_information_regex = Regex::new(r"(?<number_of_cubes>\d+) (?<cube_colour>\w+)").unwrap();

    for cap in game_number_regex.captures_iter(&line) {
        game_info.game_number = cap["game_number"].parse::<u32>().unwrap();
    }

    let game_split: Vec<&str> = line.split(":").collect();

    if let Some(rounds_str) = game_split.get(1) {
    

        let rounds: Vec<&str> = rounds_str.split(";").collect();

        let mut round_number = 1;

        for round in rounds {
            let mut round_information = RoundInformation {
                round_number: round_number,
                cube_assignments: Vec::new(),
            };

            round_number += 1;

            for cap in round_information_regex.captures_iter(round) {
                let cube_assignment = CubeAssignment {
                    number_of_cubes: cap["number_of_cubes"].parse::<u32>().unwrap(),
                    cube_colour: String::from(&cap["cube_colour"]),
                };
                round_information.cube_assignments.push(cube_assignment);
            }
            game_info.rounds.push(round_information);
        }
    }

    game_info
} 
