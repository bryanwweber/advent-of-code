use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn is_monotonic(numbers: &[i32]) -> bool {
    if numbers.len() <= 1 {
        return true;
    }

    let increasing = numbers.windows(2).all(|w| w[0] < w[1]);
    let decreasing = numbers.windows(2).all(|w| w[0] > w[1]);
    increasing || decreasing
}

fn is_gradual(numbers: &[i32]) -> bool {
    if numbers.len() <= 1 {
        return true;
    }

    let gradual = numbers.windows(2).all(|w| (w[0] - w[1]).abs() <= 3);
    gradual
}

fn get_values_from_line(
    line: Result<String, std::io::Error>,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let line = line?;
    let values: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;
    Ok(values)
}

fn check_values(values: &[i32]) -> bool {
    if is_monotonic(&values) && is_gradual(&values) {
        return true;
    }
    false
}

fn open_file(filepath: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let path = Path::new(filepath);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

pub fn solve_part1() {
    match open_file("data/02/sample.txt") {
        Ok(reader) => {
            let mut results: Vec<bool> = Vec::new();
            for (line_num, line) in reader.lines().enumerate() {
                match get_values_from_line(line) {
                    Ok(values) => results.push(check_values(&values)),
                    Err(e) => eprintln!("Error reading line {}: {}", line_num, e),
                }
            }
            let safe_reports = results.iter().copied().filter(|&x| x).count();
            println!("There are {} safe reports", safe_reports)
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

pub fn solve_part2() {
    match open_file("data/02/input.txt") {
        Ok(reader) => {
            let mut n_safe_reports = 0;
            for (line_num, line) in reader.lines().enumerate() {
                match get_values_from_line(line) {
                    Ok(values) => {
                        let mut result = check_values(&values);
                        let mut ii = 0;
                        while !result && ii < values.len() {
                            let mut modified = values.clone();
                            modified.remove(ii);
                            result = check_values(&modified);
                            ii += 1;
                        }
                        if result {
                            n_safe_reports += 1;
                        }
                    }
                    Err(e) => eprintln!("Error reading line {}: {}", line_num, e),
                }
            }
            println!("There are {} safe reports", n_safe_reports)
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
