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

fn diagnoals(grid: &[Vec<char>], min_length: usize) -> Vec<Vec<char>> {
    if grid.is_empty() || grid[0].is_empty() {
        return Vec::new();
    }

    let rows = grid.len();
    let cols = grid[0].len();
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

pub fn solve_part1() {
    match open_file("data/04/input.txt") {
        Ok(reader) => match reader.lines().collect::<Result<Vec<_>, _>>() {
            Ok(lines) => {
                let grid = strings_to_grid(lines);
                let content = grid_to_strings(&grid).join("\n");
                let mut total = find_xmas(&content);
                total += find_xmas(&grid_to_strings(&transpose(&grid)).join("\n"));
                total += find_xmas(&grid_to_strings(&diagnoals(&grid, 4)).join("\n"));
                println!("Found {} instances of XMAS", total);
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Position {
    row: isize,
    col: isize,
    character: char,
}

#[derive(Debug, Eq, PartialEq)]
struct Pattern {
    positions: Vec<Position>,
}

impl Ord for Pattern {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.positions[0].cmp(&other.positions[0])
    }
}

impl PartialOrd for Pattern {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_diagonal_positions(
    grid: &[Vec<char>],
    start_row: usize,
    start_col: usize,
    is_left: bool,
) -> Vec<Position> {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut diagonal_positions = Vec::new();
    let mut row = start_row as isize;
    let mut col = start_col as isize;

    while row < rows
        && (if is_left {
            col >= 0
        } else {
            col < cols as isize
        })
    {
        let character = grid[row as usize][col as usize];
        diagonal_positions.push(Position {
            row,
            col,
            character,
        });
        row += 1;
        if is_left {
            col -= 1;
        } else {
            col += 1;
        }
    }

    diagonal_positions
}

fn find_in_positions(positions: &[Position]) -> Vec<Pattern> {
    let mut patterns = Vec::new();

    for i in 0..positions.len().saturating_sub(2) {
        let pos1 = positions[i];
        let pos2 = positions[i + 1];
        let pos3 = positions[i + 2];

        if (pos1.character == 'M' && pos2.character == 'A' && pos3.character == 'S')
            || (pos1.character == 'S' && pos2.character == 'A' && pos3.character == 'M')
        {
            patterns.push(Pattern {
                positions: vec![pos1, pos2, pos3],
            });
        }
    }

    patterns
}

fn find_pattern_in_diagonals(grid: &[Vec<char>]) -> Vec<Pattern> {
    if grid.is_empty() || grid[0].is_empty() {
        return Vec::new();
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut patterns = Vec::new();

    // Down-right diagonals
    let starts = (0..cols)
        .map(|c| (0, c)) // First row
        .chain((1..rows).map(|r| (r, 0))); // First column

    for (start_row, start_col) in starts {
        patterns.extend(find_in_positions(&get_diagonal_positions(
            grid, start_row, start_col, false,
        )));
    }

    // Down-left diagonals
    let starts = (0..cols)
        .map(|c| (0, c)) // First row
        .chain((1..rows).map(|r| (r, cols - 1))); // Last column

    for (start_row, start_col) in starts {
        patterns.extend(find_in_positions(&get_diagonal_positions(
            grid, start_row, start_col, true,
        )));
    }
    patterns.sort();
    patterns
}

fn find_x_patterns(patterns: &[Pattern]) -> usize {
    let mut count = 0;
    // Nested loop over the patterns to check for all the combinations where the
    // position of the A is the same
    for i in 0..patterns.len() {
        for j in i + 1..patterns.len() {
            if patterns[i].positions[1].row == patterns[j].positions[1].row
                && patterns[i].positions[1].col == patterns[j].positions[1].col
            {
                count += 1;
            }
        }
    }
    count
}

pub fn solve_part2() {
    match open_file("data/04/input.txt") {
        Ok(reader) => match reader.lines().collect::<Result<Vec<_>, _>>() {
            Ok(lines) => {
                let grid = strings_to_grid(lines);
                let patterns = find_pattern_in_diagonals(&grid);
                let total = find_x_patterns(&patterns);
                println!("Found {} instances of MAS", total);
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
