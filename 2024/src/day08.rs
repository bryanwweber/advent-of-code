use itertools::Itertools;
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

fn find_unique_frequencies(grid: &Vec<Vec<char>>) -> Vec<char> {
    let mut frequencies = Vec::new();
    for line in grid.iter() {
        for &cell in line.iter() {
            if cell != '.' && !frequencies.contains(&cell) {
                frequencies.push(cell);
            }
        }
    }

    frequencies
}

fn find_nodes(grid: &[Vec<char>], frequencies: &[char]) -> HashMap<char, Vec<(i32, i32)>> {
    let mut nodes = HashMap::new();
    for freq in frequencies {
        let mut freq_nodes = Vec::new();
        for (row, line) in grid.iter().enumerate() {
            for (col, &cell) in line.iter().enumerate() {
                if cell == *freq {
                    freq_nodes.push((row as i32, col as i32));
                }
            }
        }
        nodes.insert(*freq, freq_nodes);
    }

    nodes
}

fn find_antinodes(
    all_nodes: &HashMap<char, Vec<(i32, i32)>>,
    height: i32,
    width: i32,
    find_all: bool,
) -> HashSet<(i32, i32)> {
    all_nodes
        .values()
        .flat_map(|nodes| {
            nodes.iter().combinations(2).flat_map(|pair| {
                let (row_1, col_1) = pair[0];
                let (row_2, col_2) = pair[1];
                let row_diff = row_2 - row_1;
                let col_diff = col_2 - col_1;
                let mut ix = 0;
                let mut these_antinodes: HashSet<(i32, i32)> = HashSet::new();
                these_antinodes.extend(pair);
                loop {
                    ix += 1;
                    let antinode_1 = (row_1 - ix * row_diff, col_1 - ix * col_diff);
                    let antinode_2 = (row_2 + ix * row_diff, col_2 + ix * col_diff);

                    let valid_antinodes: Vec<(i32, i32)> = vec![antinode_1, antinode_2]
                        .into_iter()
                        .filter(|&pos| is_within_bounds(pos, height, width))
                        .collect();
                    if valid_antinodes.is_empty() {
                        break these_antinodes;
                    }
                    these_antinodes.extend(valid_antinodes);

                    if !find_all {
                        break these_antinodes;
                    }
                }
            })
        })
        .collect()
}

fn is_within_bounds(pos: (i32, i32), height: i32, width: i32) -> bool {
    pos.0 >= 0 && pos.0 < height && pos.1 >= 0 && pos.1 < width
}

pub fn solve_part1() {
    match open_file("data/08/input.txt") {
        Ok(reader) => {
            let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
            let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
            let frequencies = find_unique_frequencies(&grid);
            let all_nodes = find_nodes(&grid, &frequencies);
            let antinodes =
                find_antinodes(&all_nodes, grid.len() as i32, grid[0].len() as i32, false);
            println!("Found {} antinodes", antinodes.len());
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

pub fn solve_part2() {
    match open_file("data/08/input.txt") {
        Ok(reader) => {
            let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
            let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
            let frequencies = find_unique_frequencies(&grid);
            let all_nodes = find_nodes(&grid, &frequencies);
            let antinodes =
                find_antinodes(&all_nodes, grid.len() as i32, grid[0].len() as i32, true);
            println!("Found {} antinodes", antinodes.len());
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
