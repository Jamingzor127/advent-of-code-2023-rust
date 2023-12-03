use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input_file = &args[1];
    let mut sum_of_calibration_values: u32 = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(str) = line {
                let mut first_int: u32 = 0;
                let mut second_int: u32 = 0;
                
                for char_value in str.chars() {
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
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
