use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


pub fn main() {
    part_1();
    part_2();
}

fn part_2() {
    let mut total = 0;
    
    if let Ok(lines) = read_lines("src/inputs/day_1") {
        for line in lines.flatten() {
            if let Some(new_line) = convert_ascii_to_digit(&line) {
                if let Some((first_digit, last_digit)) = extract_digits(&new_line) {
                    let calibration_value = combine_digits(first_digit, last_digit);
                    total += calibration_value;
                }
            }
        }
    }

    println!("Answer: {}", total);
}

fn part_1() {
    let mut total = 0;

    if let Ok(lines) = read_lines("src/inputs/day_1") {
        for line in lines.flatten() {
            if let Some((first_digit, last_digit)) = extract_digits(&line) {
                let calibration_value = combine_digits(first_digit, last_digit);
                total += calibration_value;
            }
        }
    }

    println!("Answer: {}", total);
}


fn convert_ascii_to_digit(line: &str) -> Option<String> {
    let words_to_digits: HashMap<&str, &str> = HashMap::from([
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let mut converted_line = String::from("");
    let mut current_word = String::from("");

    for c in line.chars() {
        current_word.push(c);
        if words_to_digits.contains_key(current_word.as_str()) {
            if let Some(digit) = words_to_digits.get(current_word.as_str()) {
                converted_line.push_str(&digit);
            }
        }
    }

    Some(converted_line)
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn extract_digits(line: &str) -> Option<(u32, u32)> {
    let first_digit = line.chars().find(|c| c.is_ascii_digit())?;
    let last_digit = line.chars().rev().find(|c| c.is_ascii_digit())?;

    Some((first_digit.to_digit(10)?, last_digit.to_digit(10)?))
}


fn combine_digits(first_digit: u32, last_digit: u32) -> u32 {
    first_digit * 10 + last_digit
}
