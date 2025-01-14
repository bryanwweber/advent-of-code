use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn open_file(filepath: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let path: &Path = Path::new(filepath);
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);
    Ok(reader)
}

fn count_stones_recursive(
    value: u64,
    remaining_blinks: usize,
    cache: &mut HashMap<(u64, usize), u64>,
) -> u64 {
    if remaining_blinks == 0 {
        return 1;
    }
    if let Some(&count) = cache.get(&(value, remaining_blinks)) {
        return count;
    }
    let count = if value == 0 {
        count_stones_recursive(1, remaining_blinks - 1, cache)
    } else {
        // Neat trick! Found it at https://blog.jverkamp.com/2024/12/11/aoc-2024-day-11-exponential-growthinator/
        let n_digits = value.ilog10() + 1;
        if n_digits % 2 == 0 {
            let divisor = 10u64.pow(n_digits / 2);
            count_stones_recursive(value / divisor, remaining_blinks - 1, cache)
                + count_stones_recursive(value % divisor, remaining_blinks - 1, cache)
        } else {
            count_stones_recursive(value * 2024, remaining_blinks - 1, cache)
        }
    };
    cache.insert((value, remaining_blinks), count);
    count
}

fn solve(input_file: &str, n_blinks: usize) {
    let result = open_file(input_file).and_then(|reader| {
        let line = reader.lines().next().unwrap().unwrap();
        let mut cache: HashMap<(u64, usize), u64> = HashMap::new();
        let count: u64 = line
            .split(" ")
            .map(|x| count_stones_recursive(x.parse().unwrap(), n_blinks, &mut cache))
            .sum();
        Ok(count)
    });
    match result {
        Ok(count) => println!("Count: {}", count),
        Err(err) => eprintln!("Error: {}", err),
    }
}

pub fn solve_part1() {
    solve("data/11/input.txt", 25)
}
pub fn solve_part2() {
    solve("data/11/input.txt", 75)
}
