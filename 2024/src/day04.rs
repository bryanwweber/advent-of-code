use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn open_file(filepath: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let path = Path::new(filepath);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

fn transpose(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    if grid.is_empty() || grid[0].is_empty() {
        return Vec::new();
    }

    (0..grid[0].len())
        .map(|col| grid.iter().map(|row| row[col]).collect())
        .collect()
}

fn strings_to_grid(lines: Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn grid_to_strings(grid: &[Vec<char>]) -> Vec<String> {
    grid.iter().map(|row| row.iter().collect()).collect()
}

fn diagnoals(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    if grid.is_empty() || grid[0].is_empty() {
        return Vec::new();
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let min_length = 4;
    let mut diagonals = Vec::new();

    // Down-right diagonals
    let starts = (0..cols)
        .map(|c| (0, c)) // First row
        .chain((1..rows).map(|r| (r, 0))); // First column (excluding 0,0)

    for (start_row, start_col) in starts {
        let diagonal: Vec<char> = (0..)
            .take_while(|&i| start_row + i < rows && start_col + i < cols)
            .map(|i| grid[start_row + i][start_col + i])
            .collect();

        if diagonal.len() >= min_length {
            diagonals.push(diagonal);
        }
    }

    // Down-left diagonals
    let starts = (0..cols)
        .map(|c| (0, c)) // First row
        .chain((1..rows).map(|r| (r, cols - 1))); // Last column (excluding top-right)

    for (start_row, start_col) in starts {
        let diagonal: Vec<char> = (0..)
            .take_while(|&i| start_row + i < rows && start_col >= i)
            .map(|i| grid[start_row + i][start_col - i])
            .collect();

        if diagonal.len() >= min_length {
            diagonals.push(diagonal);
        }
    }

    diagonals
}

fn find_xmas(content: &str) -> usize {
    let matches = content.matches("XMAS").count();
    let matches2 = content.matches("SAMX").count();

    return matches + matches2;
}

fn find_mas(content: &str) -> Vec<usize> {
    let matches: Vec<usize> = content.match_indices("MAS").map(|(i, _)| i).collect();
    let matches2: Vec<usize> = content.match_indices("SMA").map(|(i, _)| i).collect();
    return matches.into_iter().chain(matches2).collect();
}

pub fn solve_part1() {
    match open_file("data/04/input.txt") {
        Ok(reader) => match reader.lines().collect::<Result<Vec<_>, _>>() {
            Ok(lines) => {
                let grid = strings_to_grid(lines);
                let content = grid_to_strings(&grid).join("\n");
                let mut total = find_xmas(&content);
                total += find_xmas(&grid_to_strings(&transpose(&grid)).join("\n"));
                total += find_xmas(&grid_to_strings(&diagnoals(&grid)).join("\n"));
                println!("Found {} instances of XMAS", total);
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

pub fn solve_part2() {
    match open_file("data/04/input.txt") {
        Ok(reader) => match reader.lines().collect::<Result<Vec<_>, _>>() {
            Ok(lines) => {
                let grid = strings_to_grid(lines);
                let total = find_mas(&grid_to_strings(&diagnoals(&grid)).join("\n"));
                println!("Found {} instances of MAS", total.len());
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
