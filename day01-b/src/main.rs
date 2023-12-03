use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

struct NumericWord<'a> {
    word: &'a str,
    replacement_word: &'a str,
}

const NUMERIC_WORDS: [NumericWord; 9] = [
    NumericWord {
        word: "one",
        replacement_word: "o1e",
    },
    NumericWord {
        word: "two",
        replacement_word: "t2o",
    },
    NumericWord {
        word: "three",
        replacement_word: "t3hree",
    },
    NumericWord {
        word: "four",
        replacement_word: "f4our",
    },
    NumericWord {
        word: "five",
        replacement_word: "f5ive",
    },
    NumericWord {
        word: "six",
        replacement_word: "s6ix",
    },
    NumericWord {
        word: "seven",
        replacement_word: "s7even",
    },
    NumericWord {
        word: "eight",
        replacement_word: "e8ight",
    },
    NumericWord {
        word: "nine",
        replacement_word: "n9ine",
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
                //println!("Updated line: {}", updated_line);
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
                        //println!("Calibration Value {}", calibration_value); 
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

/**
 * Converts all numeric words in the line to digits.
 */
fn convert_numeric_words_in_line_to_digits(line: String) -> String {
    let mut updated_line = line;
    for word in NUMERIC_WORDS.iter() {
        updated_line = updated_line.replace(word.word, word.replacement_word);
    }
    updated_line
}