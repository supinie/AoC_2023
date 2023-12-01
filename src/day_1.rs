use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
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
