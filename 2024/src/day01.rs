use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_numbers(filepath: &str) -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
    let path = Path::new(filepath);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let values: Vec<&str> = line.split_whitespace().collect();

        if values.len() != 2 {
            return Err(format!(
                "Invalid format at line {}: expected 2 numbers, but found {}",
                line_num + 1,
                values.len()
            )
            .into());
        }
        let num1: i32 = values[0].parse()?;
        let num2: i32 = values[1].parse()?;
        numbers.push((num1, num2));
    }
    Ok(numbers)
}

fn sort_columns(numbers: &[(i32, i32)]) -> (Vec<i32>, Vec<i32>) {
    let mut col1: Vec<i32> = numbers.iter().map(|&(x, _)| x).collect();
    let mut col2: Vec<i32> = numbers.iter().map(|&(_, y)| y).collect();

    col1.sort();
    col2.sort();
    (col1, col2)
}

fn calculate_difference_sum(col1: &[i32], col2: &[i32]) -> i32 {
    col1.iter()
        .zip(col2.iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}

fn count_column_values(numbers: &[i32]) -> HashMap<i32, i32> {
    let mut count_map = HashMap::with_capacity(numbers.len());

    numbers.iter().for_each(|&num| {
        *count_map.entry(num).or_insert(0) += 1;
    });

    count_map
}

pub fn solve_part1() {
    match read_numbers("data/01/input.txt") {
        Ok(numbers) => {
            let (sorted_col1, sorted_col2) = sort_columns(&numbers);
            let sum = calculate_difference_sum(&sorted_col1, &sorted_col2);
            println!("\nSum of differences: {}", sum);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

pub fn solve_part2() {
    match read_numbers("data/01/input.txt") {
        Ok(numbers) => {
            let mut similarity: i32 = 0;
            let col1: Vec<i32> = numbers.iter().map(|&(x, _)| x).collect();
            let col2: Vec<i32> = numbers.iter().map(|&(_, y)| y).collect();
            let count_map: HashMap<i32, i32> = count_column_values(&col2);
            for &num1 in &col1 {
                let count = count_map.get(&num1).unwrap_or(&0);
                let product = num1 * count;
                similarity += product;
            }
            println!("Similarity score: {}", similarity)
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
