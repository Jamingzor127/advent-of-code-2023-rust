use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = std::env::args().collect();

    let input_file = &args[1];
    let mut sum_of_scratchcards: u32 = 0;
    let mut hash_of_pending_won_scratchcards: HashMap<u32, u32> = HashMap::new();

    let mut curr_card_number: u32 = 1;

    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(str) = line {

                let line_regex = Regex::new(r".*:(?<winning_numbers>.*)\|(?<chosen_numbers>.*)").unwrap();

                let numbers_regex = Regex::new(r"(?<number>\d+)").unwrap();

                let mut set_of_winning_numbers: HashSet<u32> = HashSet::new();
                let mut vec_of_chosen_numbers: Vec<u32> = Vec::new();

                for cap in line_regex.captures_iter(&str) {
                    let winning_numbers = &cap["winning_numbers"];
                    let chosen_numbers = &cap["chosen_numbers"];
                    for cap in numbers_regex.captures_iter(&winning_numbers) {
                        let winning_number = &cap["number"];
                        set_of_winning_numbers.insert(winning_number.parse::<u32>().unwrap());
                    }
                    for cap in numbers_regex.captures_iter(&chosen_numbers) {
                        let chosen_number = &cap["number"];
                        vec_of_chosen_numbers.push(chosen_number.parse::<u32>().unwrap());
                    }
                }

                let mut number_of_matches = 0;

                for chosen_number in vec_of_chosen_numbers {
                    if set_of_winning_numbers.contains(&chosen_number) {
                        number_of_matches += 1;
                    }
                }



                let pending_scratchcards: u32;
                if let Some(pending_value) = hash_of_pending_won_scratchcards.get(&curr_card_number) {
                    pending_scratchcards = *pending_value;
                } else {
                    pending_scratchcards = 0;
                }

                let total_cards_for_this_round = 1 + pending_scratchcards;

                if number_of_matches > 0 {
                    for i in 1..=number_of_matches {
                        if let Some(value_of_hash) = hash_of_pending_won_scratchcards.get(&(curr_card_number + i)) {
                            hash_of_pending_won_scratchcards.insert(curr_card_number + i , value_of_hash + total_cards_for_this_round);
                        } else {
                            hash_of_pending_won_scratchcards.insert(curr_card_number + i, total_cards_for_this_round);
                        }
                    }
                    
                }



                sum_of_scratchcards += total_cards_for_this_round;


            }

            curr_card_number += 1;
        }
    }

    println!("Sum of Scratchcards: {}", sum_of_scratchcards);
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