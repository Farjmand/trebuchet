use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./map.txt") {
    let mut numbers = Vec::new();
    let mut result = 0;
        for line in lines.flatten() {
            if let Some(combined_number) = combine_digits_from_each_line(&line) {
                numbers.push(combined_number)
            } else {
                println!("No digits found in line: {}", line);
            }
        }
        let total_sum = sum_combined_numbers(&numbers);
        println!("Added all numbers: {:?}", total_sum);
    }
}

fn sum_combined_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}
fn combine_digits_from_each_line(line: &str)-> Option<i32> {

    let digits: Vec<_> = line.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() >= 2 {
        let first = digits[0] as i32;
        let last = digits[digits.len() - 1] as i32;
        Some(first * 10 + last)
    } else if digits.len() == 1 {

        let first = digits[0] as i32;
        Some(first * 10 + first)
    } else {
        None
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
