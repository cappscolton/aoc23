use clap::Parser;
use std::collections::HashMap;
use std::cmp::{min, max};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let mut total: i32 = 0;
    let substrings = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for line in content.lines() {
        let mut index_number_map: HashMap<usize, i32> = HashMap::new();
        for (index, substring) in substrings.iter().enumerate() {
            if let Some(start_index) = line.find(substring) {
                index_number_map.insert(start_index, (index + 1) as i32);
            }
            if let Some(last_occurrence) = line.rfind(substring) {
                index_number_map.insert(last_occurrence, (index+1) as i32);
            }

        }

        for (index, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if let Some(digit) = c.to_digit(10) {
                    index_number_map.insert(index, digit as i32);
                }
            }
        }

        //println!("HashMap: {:?}", index_number_map);
        let min_pos = index_number_map.keys().min().copied().unwrap(); // safe to unwrap because we know there's at least one digit
        let max_pos = index_number_map.keys().max().copied().unwrap(); // same reason as above

        let min_value = *index_number_map.get(&min_pos).unwrap(); // safe to unwrap because we have the key
        let max_value = *index_number_map.get(&max_pos).unwrap(); // same reason as above

        let concatenated = format!("{}{}", min_value, max_value);
        match concatenated.parse::<i32>() {
            Ok(num) => total += num,
            Err(_) => println!("Could not parse the string as an i32"),
        }
        //println!("Concatenated string of values at min and max keys: {}", concatenated);
    }
    println!("{}", total);
}

