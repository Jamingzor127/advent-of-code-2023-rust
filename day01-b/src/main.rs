use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

struct NumericWord<'a> {
    word: &'a str,
    value: u32,
}

const NUMERIC_WORDS: [NumericWord; 9] = [
    NumericWord {
        word: "one",
        value: 1,
    },
    NumericWord {
        word: "two",
        value: 2,
    },
    NumericWord {
        word: "three",
        value: 3,
    },
    NumericWord {
        word: "four",
        value: 4,
    },
    NumericWord {
        word: "five",
        value: 5,
    },
    NumericWord {
        word: "six",
        value: 6,
    },
    NumericWord {
        word: "seven",
        value: 7,
    },
    NumericWord {
        word: "eight",
        value: 8,
    },
    NumericWord {
        word: "nine",
        value: 9,
    }];


fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = std::env::args().collect();

    let input_file = &args[1];
    let mut sum_of_calibration_values: u32 = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(str) = line {
                let updated_line = convert_numeric_words_in_line_to_digits(String::from(str));
                println!("Updated line: {}", updated_line);
                let mut first_int: u32 = 0;
                let mut second_int: u32 = 0;
                
                for char_value in updated_line.chars() {
                    if char_value.is_digit(10) {
                        let char_as_int = char_value.to_digit(10);

                        match char_as_int {
                            Some(x) => {
                                if first_int == 0 {
                                    first_int = x;
                                }
                                second_int = x;
                            },
                            None => println!("Error converting char to int"),
                        }
            
                    }
                }
                match format!("{}{}", first_int, second_int).parse::<u32>() {
                    Ok(calibration_value) => { 
                        println!("Calibration Value {}", calibration_value); 
                        sum_of_calibration_values += calibration_value;
                    }
                    Err(e) => {
                        println!("Error parsing string to int: {}", e);
                    }
                }
            }
        }
    }
    println!("Sum of calibration values: {}", sum_of_calibration_values);
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

fn convert_numeric_words_in_line_to_digits(line: String) -> String {
    let mut updated_line = line;
    for word in NUMERIC_WORDS.iter() {
        updated_line = updated_line.replace(word.word, word.value.to_string().as_str());
    }
    String::from(updated_line)
}