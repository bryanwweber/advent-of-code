use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn open_file(filepath: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let path: &Path = Path::new(filepath);
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);
    Ok(reader)
}

fn find_starting_position(grid: &[Vec<char>]) -> (usize, usize) {
    for (row, line) in grid.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == '^' {
                return (row, col);
            }
        }
    }

    (0, 0)
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct StateWithSteps {
    row: i32,
    col: i32,
    direction: char,
    steps: usize,
}

impl StateWithSteps {
    fn new(row: i32, col: i32, direction: char, steps: usize) -> Self {
        Self {
            row,
            col,
            direction,
            steps,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    row: i32,
    col: i32,
    direction: char,
}

impl State {
    fn new(row: i32, col: i32, direction: char) -> Self {
        Self {
            row,
            col,
            direction,
        }
    }

    fn move_forward(&self, grid: &[Vec<char>]) -> Option<Self> {
        let next_space: (i32, i32) = match self.direction {
            '^' => (self.row - 1, self.col),
            'v' => (self.row + 1, self.col),
            '<' => (self.row, self.col - 1),
            '>' => (self.row, self.col + 1),
            _ => panic!("Invalid direction: {}", self.direction),
        };
        if next_space.0 < 0 || next_space.1 < 0 {
            // println!("Went off the top or left edge of the grid");
            return None;
        } else if next_space.0 >= grid.len() as i32 || next_space.1 >= grid[0].len() as i32 {
            // println!("Went off the bottom or right edge of the grid");
            return None;
        } else if grid[next_space.0 as usize][next_space.1 as usize] == '#' {
            return Some(self.turn_right());
        } else {
            return Some(Self::new(next_space.0, next_space.1, self.direction));
        }
    }

    fn turn_right(&self) -> Self {
        match self.direction {
            '^' => Self::new(self.row, self.col, '>'),
            'v' => Self::new(self.row, self.col, '<'),
            '<' => Self::new(self.row, self.col, '^'),
            '>' => Self::new(self.row, self.col, 'v'),
            _ => panic!("Invalid direction: {}", self.direction),
        }
    }
}

fn detect_loop(grid: &mut Vec<Vec<char>>, start_row: i32, start_col: i32) -> Option<usize> {
    let mut current: StateWithSteps = StateWithSteps::new(start_row, start_col, '^', 0);
    let mut visited: HashMap<(i32, i32, char), usize> = HashMap::new();

    loop {
        let key = (current.row, current.col, current.direction);

        if let Some(prev_steps) = visited.get(&key) {
            return Some(current.steps - prev_steps);
        }

        visited.insert(key, current.steps);

        let state = State::new(current.row, current.col, current.direction);
        match state.move_forward(&grid) {
            Some(next_state) => {
                current = StateWithSteps::new(
                    next_state.row,
                    next_state.col,
                    next_state.direction,
                    current.steps + 1,
                );
            }
            None => return None,
        }
    }
}

pub fn solve_part1() {
    match open_file("data/06/input.txt") {
        Ok(reader) => {
            let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
            let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
            let starting_position = find_starting_position(&grid);
            let mut current_state: State =
                State::new(starting_position.0 as i32, starting_position.1 as i32, '^');
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            loop {
                visited.insert((current_state.row, current_state.col));
                match current_state.move_forward(&grid) {
                    Some(next_state) => current_state = next_state,
                    None => break,
                }
            }
            println!("Visited {} unique cells", visited.len());
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

pub fn solve_part2() {
    match open_file("data/06/input.txt") {
        Ok(reader) => {
            let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
            let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
            let starting_position = find_starting_position(&grid);
            let loop_count = (0..grid.len())
                .flat_map(|row| (0..grid[0].len()).map(move |col| (row, col)))
                .par_bridge()
                .filter(|(row, col)| {
                    if grid[*row][*col] == '.' {
                        let mut test_grid = grid.clone();
                        test_grid[*row][*col] = '#';
                        detect_loop(
                            &mut test_grid,
                            starting_position.0 as i32,
                            starting_position.1 as i32,
                        )
                        .is_some()
                    } else {
                        false
                    }
                })
                .count();
            println!("Found {} positions that create loops", loop_count);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
