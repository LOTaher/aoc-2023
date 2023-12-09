// AOC Day 1 in Rust!

use std::fs;

fn main() {
    let file_path = "./input.txt";
    let mut total = 0;

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

        if let Ok(num) = double_digit.parse::<i32>() {
            total += num;
        }
    }

    println!("{}", total);
}
