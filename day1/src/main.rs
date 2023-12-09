// AOC Day 1 in Rust!

use std::fs;

fn main() {
    let file_path = "./input.txt";
    let mut sum = 0;

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file.");

    let list: Vec<&str> = contents.split("\n").collect();

    for item in &list {
        let mut double_digit = String::new();

        for c in item.chars() {
            if c.is_digit(10) {
                double_digit.push(c);
            }
        }

        if !double_digit.is_empty() {
            let first = double_digit.chars().nth(0).unwrap();
            let last = double_digit.chars().rev().nth(0).unwrap();
            
            let combined = first.to_string() + &last.to_string();
            let combined_number = combined.parse::<i32>().unwrap();

            sum += combined_number
        }
    }

    println!("{}", sum);
}
