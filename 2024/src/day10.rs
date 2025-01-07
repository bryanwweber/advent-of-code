use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn open_file(filepath: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let path: &Path = Path::new(filepath);
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);
    Ok(reader)
}

#[derive(Clone, Debug, Copy)]
struct Location {
    row: usize,
    col: usize,
    value: i32,
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn find_unique_trail_ends(
    grid: &Vec<Vec<Location>>,
    starting_position: Location,
) -> HashSet<(i32, i32)> {
    let mut trails: HashSet<(i32, i32)> = HashSet::new();
    let mut current_positions: VecDeque<Location> = VecDeque::new();
    current_positions.push_back(starting_position);
    let max_row = grid.len() - 1;
    let max_col = grid[0].len() - 1;
    loop {
        if current_positions.is_empty() {
            break;
        }
        let current_position = current_positions.pop_front().unwrap();
        if current_position.value == 9 {
            trails.insert((current_position.row as i32, current_position.col as i32));
            continue;
        }

        let mut neighbors = Vec::new();

        if current_position.col < max_col {
            neighbors.push((
                grid[current_position.row][current_position.col + 1],
                Direction::East,
            ));
        }
        if current_position.col > 0 {
            neighbors.push((
                grid[current_position.row][current_position.col - 1],
                Direction::West,
            ));
        }
        if current_position.row < max_row {
            neighbors.push((
                grid[current_position.row + 1][current_position.col],
                Direction::South,
            ));
        }
        if current_position.row > 0 {
            neighbors.push((
                grid[current_position.row - 1][current_position.col],
                Direction::North,
            ));
        }
        current_positions.extend(
            neighbors
                .iter()
                .filter(|(loc, _)| loc.value == current_position.value + 1)
                .map(|(loc, _)| *loc)
                .collect::<Vec<Location>>(),
        );
    }
    trails
}

pub fn solve_part1() {
    match open_file("data/10/input.txt") {
        Ok(reader) => {
            let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
            let grid: Vec<Vec<Location>> = lines
                .iter()
                .enumerate()
                .map(|(i_row, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(i_col, c)| Location {
                            row: i_row,
                            col: i_col,
                            value: c.to_digit(10).unwrap() as i32,
                        })
                        .collect()
                })
                .collect();
            let zeros = grid.iter().enumerate().flat_map(|(i_row, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, loc)| loc.value == 0)
                    .map(move |(i_col, _)| Location {
                        row: i_row,
                        col: i_col,
                        value: 0,
                    })
            });
            let trails = zeros
                .map(|loc| find_unique_trail_ends(&grid, loc).into_iter().count())
                .sum::<usize>();
            println!("{:?}", trails);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn find_all_trails(grid: &Vec<Vec<Location>>, starting_position: Location) -> Vec<(i32, i32)> {
    let mut trails: Vec<(i32, i32)> = Vec::new();
    let mut current_positions: VecDeque<Location> = VecDeque::new();
    current_positions.push_back(starting_position);
    let max_row = grid.len() - 1;
    let max_col = grid[0].len() - 1;
    loop {
        if current_positions.is_empty() {
            break;
        }
        let current_position = current_positions.pop_front().unwrap();
        if current_position.value == 9 {
            trails.push((current_position.row as i32, current_position.col as i32));
            continue;
        }

        let mut neighbors = Vec::new();

        if current_position.col < max_col {
            neighbors.push((
                grid[current_position.row][current_position.col + 1],
                Direction::East,
            ));
        }
        if current_position.col > 0 {
            neighbors.push((
                grid[current_position.row][current_position.col - 1],
                Direction::West,
            ));
        }
        if current_position.row < max_row {
            neighbors.push((
                grid[current_position.row + 1][current_position.col],
                Direction::South,
            ));
        }
        if current_position.row > 0 {
            neighbors.push((
                grid[current_position.row - 1][current_position.col],
                Direction::North,
            ));
        }
        current_positions.extend(
            neighbors
                .iter()
                .filter(|(loc, _)| loc.value == current_position.value + 1)
                .map(|(loc, _)| *loc)
                .collect::<Vec<Location>>(),
        );
    }
    trails
}

pub fn solve_part2() {
    match open_file("data/10/input.txt") {
        Ok(reader) => {
            let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
            let grid: Vec<Vec<Location>> = lines
                .iter()
                .enumerate()
                .map(|(i_row, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(i_col, c)| Location {
                            row: i_row,
                            col: i_col,
                            value: c.to_digit(10).unwrap() as i32,
                        })
                        .collect()
                })
                .collect();
            let zeros = grid.iter().enumerate().flat_map(|(i_row, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, loc)| loc.value == 0)
                    .map(move |(i_col, _)| Location {
                        row: i_row,
                        col: i_col,
                        value: 0,
                    })
            });
            let trails = zeros
                .map(|loc| find_all_trails(&grid, loc).into_iter().count())
                .sum::<usize>();
            println!("{:?}", trails);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
