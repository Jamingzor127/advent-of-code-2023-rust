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

    //Holds the rules that the game must follow. These represent the maximum number of cubes any given colour
    let rules: [CubeAssignment; 3] = [
        CubeAssignment {
            number_of_cubes: 12,
            cube_colour: String::from("red"),
        },
        CubeAssignment {
            number_of_cubes: 13,
            cube_colour: String::from("green"),
        },
        CubeAssignment {
            number_of_cubes: 14,
            cube_colour: String::from("blue"),
        },
    ];

    let input_file = &args[1];
    let mut sum_of_correct_games: u32 = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(str) = line {
                let game_info = split_line_into_usable_information(String::from(str));

                let mut game_is_valid = true;
                for round in game_info.rounds {
                    for cube_assignment in round.cube_assignments {
                        if let Some(rule) = rules.iter().find(|rule| rule.cube_colour == cube_assignment.cube_colour) {
                            if rule.number_of_cubes < cube_assignment.number_of_cubes {
                                game_is_valid = false;
                                println!("Game {} is invalid because there are too many {} cubes in round {}", game_info.game_number, cube_assignment.cube_colour, round.round_number);
                                break;
                            }
                        }
                    }
                    if !game_is_valid {
                        break;
                    }
                }


                if game_is_valid {
                    sum_of_correct_games += game_info.game_number;
                }
            }
        }
    }
    println!("Sum of valid games: {}", sum_of_correct_games);
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
