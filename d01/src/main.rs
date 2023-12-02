use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn first_and_last(s: &str) -> (Option<char>, Option<char>) {
    let first = s.chars().next();
    let last = s.chars().last();
    (first, last)
}

fn option_chars_to_string_to_number(chars: (Option<char>, Option<char>)) -> Result<i32, std::num::ParseIntError> {
    let mut result = String::new();
    if let Some(c) = chars.0 {
        result.push(c);
    }
    if let Some(c) = chars.1 {
        result.push(c);
    }
    result.parse::<i32>()
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let mut total = 0;
    for line in content.lines() {
        let mut temp = String::new();
        for c in line.chars() {
            if c.is_digit(10) {
                temp.push(c);
            }
        }
        let two_digits_string = first_and_last(&temp);
        match option_chars_to_string_to_number(two_digits_string) {
            Ok(number) => { total += number; }
        
            Err(_) => println!("Failed to parse number"),
        }
    }
    println!("{}", total);
}

