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

#[derive(Copy, Clone)]
enum TrailCollectionStrategy {
    Unique,
    All,
}

trait TrailCollection {
    fn add(&mut self, point: (i32, i32));
    fn into_vec(self: Box<Self>) -> Vec<(i32, i32)>;
}

impl TrailCollection for Vec<(i32, i32)> {
    fn add(&mut self, point: (i32, i32)) {
        self.push(point);
    }

    fn into_vec(self: Box<Self>) -> Vec<(i32, i32)> {
        *self
    }
}

impl TrailCollection for HashSet<(i32, i32)> {
    fn add(&mut self, point: (i32, i32)) {
        self.insert(point);
    }

    fn into_vec(self: Box<Self>) -> Vec<(i32, i32)> {
        self.into_iter().collect()
    }
}

struct Grid {
    data: Vec<Vec<Location>>,
}

impl Grid {
    fn from_lines<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let data: Vec<Vec<Location>> = lines
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
        Self { data }
    }

    fn find_zeros(&self) -> impl Iterator<Item = Location> + '_ {
        self.data.iter().enumerate().flat_map(|(i_row, row)| {
            row.iter()
                .enumerate()
                .filter(move |(_, loc)| loc.value == 0)
                .map(move |(i_col, _)| Location {
                    row: i_row,
                    col: i_col,
                    value: 0,
                })
        })
    }

    fn get_neighbors<'a>(&'a self, pos: &'a Location) -> impl Iterator<Item = Location> + 'a {
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .filter_map(move |(dx, dy)| {
                let new_row = pos.row as i32 + dx;
                let new_col = pos.col as i32 + dy;

                if self.is_valid_position(new_row, new_col) {
                    Some(self.data[new_row as usize][new_col as usize])
                } else {
                    None
                }
            })
    }

    fn is_valid_position(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.data.len() as i32 && col >= 0 && col < self.data[0].len() as i32
    }

    fn find_trails(&self, start: Location, strategy: TrailCollectionStrategy) -> Vec<(i32, i32)> {
        let mut collection: Box<dyn TrailCollection> = match strategy {
            TrailCollectionStrategy::Unique => Box::new(HashSet::new()),
            TrailCollectionStrategy::All => Box::new(Vec::new()),
        };
        let mut current_positions: VecDeque<Location> = VecDeque::from([start]);
        loop {
            if current_positions.is_empty() {
                break;
            }
            let current_position = current_positions.pop_front().unwrap();
            if current_position.value == 9 {
                collection.add((current_position.row as i32, current_position.col as i32));
                continue;
            }

            current_positions.extend(
                self.get_neighbors(&current_position)
                    .filter(|loc| loc.value == current_position.value + 1),
            );
        }

        collection.into_vec()
    }
}

fn solve(strategy: TrailCollectionStrategy) {
    let result = open_file("data/10/input.txt").and_then(|reader| {
        let grid = Grid::from_lines(reader.lines().filter_map(|line| line.ok()));
        let trail_count = grid
            .find_zeros()
            .map(|loc| grid.find_trails(loc, strategy).len())
            .sum::<usize>();
        Ok(trail_count)
    });
    match result {
        Ok(trail_count) => println!("{:?}", trail_count),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn solve_part1() {
    solve(TrailCollectionStrategy::Unique);
}

pub fn solve_part2() {
    solve(TrailCollectionStrategy::All);
}
