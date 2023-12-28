use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use regex::Regex;

struct MapDetails {
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

enum Maps {
    None,
    SeedToSoil,
    SoilToFert,
    FertToWater,
    WaterToLight,
    LightToTemp,
    TempToHumid,
    HumidToLoc,
}

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = std::env::args().collect();

    let input_file = &args[1];
    let mut lowest_location_number: u64 = 0;
    let mut vec_of_seeds: Vec<u64> = Vec::new();

    let mut seed_to_soil_maps: Vec<MapDetails> = Vec::new();
    let mut soil_to_fert_maps: Vec<MapDetails> = Vec::new();
    let mut fert_to_water_maps: Vec<MapDetails> = Vec::new();
    let mut water_to_light_maps: Vec<MapDetails> = Vec::new();
    let mut light_to_temp_maps: Vec<MapDetails> = Vec::new();
    let mut temp_to_humid_maps: Vec<MapDetails> = Vec::new();
    let mut humid_to_loc_maps: Vec<MapDetails> = Vec::new();

    if let Ok(lines) = read_lines(input_file) {
        let mut curr_map_being_parsed: Maps = Maps::None;
        for line in lines {
            if let Ok(str) = line {

                let seeds_regex = Regex::new(r"seeds: (?<seeds>.*)").unwrap();
                let category_regex = Regex::new(r"(?<category_name>\w+-\w+-\w+) map:").unwrap();
                let map_regex = Regex::new(r"(?<destination_start>\d+) (?<source_start>\d+) (?<range_length>\d+)").unwrap();

                for cap in seeds_regex.captures_iter(&str) {
                    let seeds_str = &cap["seeds"];
                    let seed_regex = Regex::new(r"(?<seed>\d+)").unwrap();
                    for cap in seed_regex.captures_iter(&seeds_str) {
                        let seed = &cap["seed"];
                        vec_of_seeds.push(seed.parse::<u64>().unwrap());
                    }
                }

                for cap in category_regex.captures_iter(&str) {
                    let category = &cap["category_name"];
                    match category {
                        "seed-to-soil" => curr_map_being_parsed = Maps::SeedToSoil,
                        "soil-to-fertilizer" => curr_map_being_parsed = Maps::SoilToFert,
                        "fertilizer-to-water" => curr_map_being_parsed = Maps::FertToWater,
                        "water-to-light" => curr_map_being_parsed = Maps::WaterToLight,
                        "light-to-temperature" => curr_map_being_parsed = Maps::LightToTemp,
                        "temperature-to-humidity" => curr_map_being_parsed = Maps::TempToHumid,
                        "humidity-to-location" => curr_map_being_parsed = Maps::HumidToLoc,
                        _ => curr_map_being_parsed = Maps::None,
                    }
                }

                for cap in map_regex.captures_iter(&str) {
                    let destination_start = &cap["destination_start"];
                    let source_start = &cap["source_start"];
                    let range_length = &cap["range_length"];
                    
                    let map_details = MapDetails {
                        destination_start: destination_start.parse::<u64>().unwrap(),
                        source_start: source_start.parse::<u64>().unwrap(),
                        range_length: range_length.parse::<u64>().unwrap(),
                    };

                    match curr_map_being_parsed {
                        Maps::SeedToSoil => seed_to_soil_maps.push(map_details),
                        Maps::SoilToFert => soil_to_fert_maps.push(map_details),
                        Maps::FertToWater => fert_to_water_maps.push(map_details),
                        Maps::WaterToLight => water_to_light_maps.push(map_details),
                        Maps::LightToTemp => light_to_temp_maps.push(map_details),
                        Maps::TempToHumid => temp_to_humid_maps.push(map_details),
                        Maps::HumidToLoc => humid_to_loc_maps.push(map_details),
                        Maps::None => (),
                    }
                }
            }
        }
    }

    for seed in vec_of_seeds.iter() {

        let mut soil: u64 = 0;
        for map in seed_to_soil_maps.iter() {
            if *seed >= map.source_start && *seed < map.source_start + map.range_length {
                soil = map.destination_start + (*seed - map.source_start);
                break;
            } else {
                soil = *seed;
            }
        }

        let mut fertilizer: u64 = 0;
        for map in soil_to_fert_maps.iter() {
            if soil >= map.source_start && soil < map.source_start + map.range_length {
                fertilizer = map.destination_start + (soil - map.source_start);
                break;
            } else {
                fertilizer = soil;
            }
        }

        let mut water: u64 = 0;
        for map in fert_to_water_maps.iter() {
            if fertilizer >= map.source_start && fertilizer < map.source_start + map.range_length {
                water = map.destination_start + (fertilizer - map.source_start);
                break;
            } else {
                water = fertilizer;
            }
        }

        let mut light: u64 = 0;
        for map in water_to_light_maps.iter() {
            if water >= map.source_start && water < map.source_start + map.range_length {
                light = map.destination_start + (water - map.source_start);
                break;
            } else {
                light = water;
            }
        }

        let mut temperature: u64 = 0;
        for map in light_to_temp_maps.iter() {
            if light >= map.source_start && light < map.source_start + map.range_length {
                temperature = map.destination_start + (light - map.source_start);
                break;
            } else {
                temperature = light;
            }
        }

        let mut humidity: u64 = 0;
        for map in temp_to_humid_maps.iter() {
            if temperature >= map.source_start && temperature < map.source_start + map.range_length {
                humidity = map.destination_start + (temperature - map.source_start);
                break;
            } else {
                humidity = temperature;
            }
        }

        let mut location: u64 = 0;
        for map in humid_to_loc_maps.iter() {
            if humidity >= map.source_start && humidity < map.source_start + map.range_length {
                location = map.destination_start + (humidity - map.source_start);
                break;
            } else {
                location = humidity;
            }
        }
       

        if lowest_location_number == 0 {
            lowest_location_number = location;
        } else if location < lowest_location_number {
            lowest_location_number = location;
        }
    }

    println!("Lowest Location Number: {}", lowest_location_number);
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