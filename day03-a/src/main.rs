use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = std::env::args().collect();

    let input_file = &args[1];
    let mut sum_of_part_numbers: u32 = 0;

    let mut prev_line_symbols_map: HashMap<u32, String> = HashMap::new();
    let mut prev_line_part_numbers_map: HashMap<u32, u32> = HashMap::new();
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(str) = line {
                let mut symbols_for_line: HashMap<u32, String> = HashMap::new();
                let mut part_numbers_for_line: HashMap<u32, u32> = HashMap::new();

                let symbols_regex = Regex::new(r"(?<symbol>[^\d.])").unwrap();
                let part_number_regex = Regex::new(r"(?<part_number>\d+)").unwrap();

                for cap in symbols_regex.captures_iter(&str) {
                    let symbol = &cap["symbol"];
                    let index = cap.get(0).unwrap().start() as u32;
                    symbols_for_line.insert(index, String::from(symbol));
                }

                for cap in part_number_regex.captures_iter(&str) {
                    let part_number = &cap["part_number"];
                    let index = cap.get(0).unwrap().start() as u32;
                    part_numbers_for_line.insert(index, part_number.parse::<u32>().unwrap());
                }

                let mut indexes_for_prev_line_valid_parts: Vec<u32> = Vec::new();

                // Check prev line's parts for any that might now be valid. Any valid part number is added to the sum and removed from the hashmap.
                for (index, part_number) in prev_line_part_numbers_map.iter() {
                    let part_number_len = part_number.to_string().len() as u32;
                    let starting_index: u32;
                    if index == &(0 as u32) {
                        starting_index = 0;
                    } else {
                        starting_index = index - 1;
                    }
                    for i in starting_index..=index+part_number_len { //checks all values under the part number for a symbol
                        if let Some(_symbol) = symbols_for_line.get(&i) {
                            sum_of_part_numbers = sum_of_part_numbers + part_number;
                            indexes_for_prev_line_valid_parts.push(*index);
                            break;
                        }
                    }
                }

                // remove all part numbers that were previously foudn
                for index in indexes_for_prev_line_valid_parts {
                    prev_line_part_numbers_map.remove(&index);
                }

                let mut indexes_for_curr_line_valid_parts: Vec<u32> = Vec::new();

                // check current line's parts for any that are valid. Any valid part number is added to the total and removed from the hashmap. 
                for (index, part_number) in part_numbers_for_line.iter() {
                    let mut found_symbol = false;
                    let part_number_len = part_number.to_string().len() as u32;
                    let starting_index: u32;
                    if index == &(0 as u32) {
                        starting_index = 0;
                    } else {
                        starting_index = index - 1;
                    }

                    for i in starting_index..=index+part_number_len { //checks all values above the part number for a symbol
                        if let Some(_symbol) = prev_line_symbols_map.get(&i) {
                            sum_of_part_numbers = sum_of_part_numbers + part_number;
                            indexes_for_curr_line_valid_parts.push(*index);
                            found_symbol = true;
                            break;
                        }
                    }
                    if !found_symbol {
                        if let Some(_symbol) = symbols_for_line.get(&(starting_index)).or(symbols_for_line.get(&(index + (part_number.to_string().len() as u32)))) {
                            sum_of_part_numbers = sum_of_part_numbers + part_number;
                            indexes_for_curr_line_valid_parts.push(*index);
                        }
                    }
                }

                for index in indexes_for_curr_line_valid_parts {
                    part_numbers_for_line.remove(&index);
                }

                prev_line_symbols_map = symbols_for_line;
                prev_line_part_numbers_map = part_numbers_for_line;

            }
        }
    }

    println!("Sum of Part Numbers: {}", sum_of_part_numbers);
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