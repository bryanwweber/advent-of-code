use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::repeat;
use std::path::Path;

fn open_file(filepath: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let path: &Path = Path::new(filepath);
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);
    Ok(reader)
}

pub fn solve_part1() {
    match open_file("data/09/input.txt") {
        Ok(reader) => {
            let data: String = reader.lines().next().unwrap().unwrap();
            let mut data_index: i32 = 0;
            let mut uncompressed: Vec<(i32, i32)> = Vec::new();
            let mut blank_space: Vec<i32> = Vec::new();
            for (data_size, blank_size) in data.chars().tuples() {
                uncompressed.push((data_index, data_size.to_digit(10).unwrap() as i32));
                blank_space.push(blank_size.to_digit(10).unwrap() as i32);
                data_index += 1;
            }
            uncompressed.push((
                data_index,
                data.chars().last().unwrap().to_digit(10).unwrap() as i32,
            ));
            let mut compressed: Vec<i32> = Vec::new();
            let mut ix = 0;
            compressed.extend(repeat(uncompressed[ix].0).take(uncompressed[ix].1 as usize));
            uncompressed[ix].1 = 0;
            ix += 1;
            let mut blank_index: usize = 0;
            let mut this_blanks = blank_space[blank_index];
            let mut length = uncompressed.len() - 1;
            while length > 0 {
                if uncompressed[length].1 == 0 {
                    length -= 1;
                    continue;
                }
                let this_size = uncompressed[length].1;
                if this_size <= this_blanks {
                    compressed.extend(repeat(uncompressed[length].0).take(this_size as usize));
                    this_blanks -= this_size;
                    length -= 1;
                } else if this_blanks == 0 {
                    compressed.extend(repeat(uncompressed[ix].0).take(uncompressed[ix].1 as usize));
                    uncompressed[ix].1 = 0;
                    ix += 1;
                    blank_index += 1;
                    this_blanks = blank_space[blank_index];
                } else {
                    compressed.extend(repeat(uncompressed[length].0).take(this_blanks as usize));
                    uncompressed[length].1 -= this_blanks;
                    compressed.extend(repeat(uncompressed[ix].0).take(uncompressed[ix].1 as usize));
                    uncompressed[ix].1 = 0;
                    ix += 1;
                    blank_index += 1;
                    this_blanks = blank_space[blank_index];
                }
            }
            let total: i64 = compressed
                .iter()
                .enumerate()
                .map(|(ix, x)| (ix as i32 * x) as i64)
                .sum();
            println!("Total: {}", total);
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
pub fn solve_part2() {}
