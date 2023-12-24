use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use regex::Regex;
use std::collections::HashMap;

struct PendingPartNumber {
    part_number: u32,
    gear_line: u32,
    gear_index: u32,
}

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = std::env::args().collect();

    let input_file = &args[1];
    let mut sum_of_part_numbers: u32 = 0;


    let mut prev_line_symbols_map: HashMap<u32, String> = HashMap::new();
    let mut prev_line_part_numbers_map: HashMap<u32, u32> = HashMap::new();

    let mut pending_part_numbers: Vec<PendingPartNumber> = Vec::new();

    let mut line_number = 0;
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
                        if let Some(symbol) = symbols_for_line.get(&i) {
                            if *symbol == "*" {
                                let add_to_sum = check_pending_symbols(
                                    &mut pending_part_numbers,
                                    line_number,
                                    *part_number,
                                    i,
                                );
                                sum_of_part_numbers = sum_of_part_numbers + add_to_sum;
                            }
                            break;
                        }
                    }
                }

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
                        if let Some(symbol) = prev_line_symbols_map.get(&i) {
                            if *symbol == "*" {
                                let add_to_sum = check_pending_symbols(
                                    &mut pending_part_numbers,
                                    line_number - 1,
                                    *part_number,
                                    i,
                                );
                                sum_of_part_numbers = sum_of_part_numbers + add_to_sum;
                            }
                            found_symbol = true;
                            break;

                        }
                    }
                    if !found_symbol {
                        let indexes_to_check = [starting_index, index + part_number_len];
                        for i in indexes_to_check {
                            if let Some(symbol) = symbols_for_line.get(&i) {
                                if *symbol == "*" {
                                    let add_to_sum = check_pending_symbols(
                                        &mut pending_part_numbers,
                                        line_number,
                                        *part_number,
                                        i,
                                    );
                                    sum_of_part_numbers = sum_of_part_numbers + add_to_sum;
                                }
                            }
                        }
                    }
                }

                prev_line_symbols_map = symbols_for_line;
                prev_line_part_numbers_map = part_numbers_for_line;

            }
            line_number = line_number + 1;
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

/**
 * Checks if a pending gear exists for the current symbol. If so, we multiply the pending part number with the new one. If not, we add our new one
 * as a pending part number. The returned value should be added to our part number sum.
 */
fn check_pending_symbols(
    pending_part_numbers: &mut Vec<PendingPartNumber>,
    line_number: i32,
    part_number: u32,
    i: u32
) -> u32 {
     // Check if a pending part number exists for this symbol.
     let mut found_pending_part_number: Option<u32> = None;
     let mut pending_part_number_index: Option<u32> = None;
     for (index, pending_part_number) in pending_part_numbers.iter().enumerate() {
         if pending_part_number.gear_line == line_number.try_into().unwrap() && pending_part_number.gear_index == i {
             found_pending_part_number = Some(pending_part_number.part_number);
             pending_part_number_index = Some(index as u32);
             break;
         }
     }

     if pending_part_number_index == None {
         pending_part_numbers.push(PendingPartNumber {
             part_number: part_number,
             gear_line: line_number.try_into().unwrap(),
             gear_index: i,
         });
     } else {
        if let Some(part_number_prev) = found_pending_part_number {
            pending_part_numbers.remove(pending_part_number_index.unwrap() as usize);

            return part_number * part_number_prev;
        }
     }
     return 0;
}