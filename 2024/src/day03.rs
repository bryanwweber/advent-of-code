use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn find_mul_expressions(text: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(text)
        .map(|capture| {
            let n1 = capture[1].parse::<i32>().unwrap();
            let n2 = capture[2].parse::<i32>().unwrap();
            (n1, n2)
        })
        .collect()
}

fn get_valid_parts(text: &str) -> Vec<String> {
    let re = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let mut valid_parts = Vec::new();
    let mut current_part = String::new();
    let mut is_active = true; // Start in "on" state (do)
    let mut last_pos = 0;

    for mat in re.find_iter(text) {
        let between_text = &text[last_pos..mat.start()];
        if is_active {
            current_part.push_str(between_text);
        }
        is_active = mat.as_str() == "do()";
        if !is_active && !current_part.is_empty() {
            valid_parts.push(current_part.clone());
            current_part.clear();
        }

        last_pos = mat.end();
    }

    if is_active && last_pos < text.len() {
        current_part.push_str(&text[last_pos..]);
        valid_parts.push(current_part);
    }

    valid_parts
}

fn read_input_text(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(filepath);
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut value = String::new();
    reader.read_to_string(&mut value)?;
    Ok(value)
}

pub fn solve_part1() {
    match read_input_text("data/03/input.txt") {
        Ok(text) => {
            let matches = find_mul_expressions(&text);
            let mul_sum: i32 = matches.iter().map(|&(x, y)| x * y).sum();
            println!("The sum is {}", mul_sum)
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
pub fn solve_part2() {
    match read_input_text("data/03/input.txt") {
        Ok(text) => {
            let mut mul_sum: i32 = 0;
            let valid_parts = get_valid_parts(&text);
            for part in valid_parts.iter() {
                let matches = find_mul_expressions(&part);
                mul_sum += matches.iter().map(|&(x, y)| x * y).sum::<i32>();
            }
            println!("The sum is {}", mul_sum)
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
