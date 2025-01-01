use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn open_file(filepath: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let path: &Path = Path::new(filepath);
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);
    Ok(reader)
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|s| s.trim_end_matches(':'))
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

fn generate_operator_permutations(count: usize, distinct_operators: usize) -> Vec<Vec<i32>> {
    let total_combinations = distinct_operators.pow(count as u32);
    let mut result = Vec::with_capacity(total_combinations);

    for i in 0..total_combinations {
        let mut current = Vec::with_capacity(count);
        let mut num = i;
        for _ in 0..count {
            current.push((num % distinct_operators) as i32);
            num /= distinct_operators;
        }
        result.push(current);
    }

    result
}
pub fn solve_part1() {
    match open_file("data/07/input.txt") {
        Ok(reader) => {
            let lines = reader.lines().filter_map(|line| line.ok());
            let equations = lines.map(|line| parse_line(&line));
            let correct_equations: i64 = equations
                .map(|eq| {
                    let result = eq[0];
                    let n_total_operators = eq[1..].len() - 1;
                    let operator_permuatations =
                        generate_operator_permutations(n_total_operators, 2);
                    for perm in operator_permuatations {
                        let mut current_result = eq[1];
                        for (i, &op) in perm.iter().enumerate() {
                            match op {
                                0 => current_result += eq[i + 2],
                                1 => current_result *= eq[i + 2],
                                _ => panic!("Invalid operator"),
                            }
                        }
                        if current_result == result {
                            return result;
                        }
                    }
                    0
                })
                .filter(|&b| b > 0)
                .sum();
            println!("Found {} correct equations", correct_equations);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
pub fn solve_part2() {
    match open_file("data/07/input.txt") {
        Ok(reader) => {
            let lines = reader.lines().filter_map(|line| line.ok());
            let equations = lines.map(|line| parse_line(&line));
            let correct_equations: i64 = equations
                .map(|eq| {
                    let result = eq[0];
                    let n_total_operators = eq[1..].len() - 1;
                    let operator_permuatations =
                        generate_operator_permutations(n_total_operators, 3);
                    for perm in operator_permuatations {
                        let mut current_result: i64 = 0;
                        let mut left_item: i64 = eq[1];
                        for (i, &op) in perm.iter().enumerate() {
                            match op {
                                0 => {
                                    current_result = left_item;
                                    current_result += eq[i + 2];
                                    left_item = current_result;
                                }
                                1 => {
                                    current_result = left_item;
                                    current_result *= eq[i + 2];
                                    left_item = current_result;
                                }
                                2 => {
                                    current_result = (left_item.to_string()
                                        + &eq[i + 2].to_string())
                                        .parse::<i64>()
                                        .unwrap();
                                    left_item = current_result;
                                }
                                _ => panic!("Invalid operator"),
                            }
                        }
                        if current_result == result {
                            return result;
                        }
                    }
                    0
                })
                .filter(|&b| b > 0)
                .sum();
            println!("Found {} correct equations", correct_equations);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
