use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn open_file(filepath: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let path: &Path = Path::new(filepath);
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);
    Ok(reader)
}

fn parse_pattern(line: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = line.split("|").collect();
    if parts.len() != 2 {
        return None;
    }
    let left: i32 = parts[0].trim().parse().unwrap();
    let right: i32 = parts[1].trim().parse().unwrap();
    Some((left, right))
}

fn read_input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let patterns: Vec<(i32, i32)> = match open_file("data/05/rules.txt") {
        Ok(reader) => reader
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| parse_pattern(&line))
            .collect::<Vec<(i32, i32)>>(),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return (vec![], vec![]);
        }
    };
    let updates: Vec<Vec<i32>> = match open_file("data/05/updates.txt") {
        Ok(reader) => reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| {
                line.split(',')
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect()
            })
            .collect(),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return (vec![], vec![]);
        }
    };
    (patterns, updates)
}

fn sort_sequence(patterns: &[(i32, i32)], update: &[i32], want_valid: bool) -> Option<Vec<i32>> {
    let mut nums = update.to_vec();

    nums.sort_by(|a, b| {
        for pattern in patterns {
            if pattern.0 == *a && pattern.1 == *b {
                return std::cmp::Ordering::Less;
            } else if pattern.0 == *b && pattern.1 == *a {
                return std::cmp::Ordering::Greater;
            }
        }
        std::cmp::Ordering::Equal
    });
    let valid = update
        .windows(2)
        .all(|pair| patterns.iter().any(|p| p.0 == pair[0] && p.1 == pair[1]));

    if valid && want_valid {
        Some(nums)
    } else if valid && !want_valid {
        None
    } else if !valid && want_valid {
        None
    } else {
        Some(nums)
    }
}

pub fn solve_part1() {
    let (patterns, updates) = read_input();
    let pb = ProgressBar::new(updates.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {eta}")
            .unwrap(),
    );
    let total: i32 = updates
        .par_iter()
        .progress_with(pb)
        .map(|update| match sort_sequence(&patterns, &update, true) {
            Some(sorted) => sorted[sorted.len() / 2],
            None => 0,
        })
        .sum();
    println!("Total: {}", total);
}

pub fn solve_part2() {
    let (patterns, updates) = read_input();
    let pb = ProgressBar::new(updates.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {eta}")
            .unwrap(),
    );
    let total: i32 = updates
        .par_iter()
        .progress_with(pb)
        .map(|update| match sort_sequence(&patterns, &update, false) {
            Some(sorted) => sorted[sorted.len() / 2],
            None => 0,
        })
        .sum();
    println!("Total: {}", total);
}
