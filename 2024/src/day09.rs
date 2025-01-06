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

#[derive(Clone, Debug)]
enum Block {
    File(usize),
    Free,
}

fn read_data_to_disk(data: &str) -> Vec<Block> {
    let mut disk: Vec<Block> = Vec::new();
    let mut file_id: usize = 0;
    for (data_size, blank_size) in data.chars().tuples() {
        disk.extend(repeat(Block::File(file_id)).take(data_size.to_digit(10).unwrap() as usize));
        disk.extend(repeat(Block::Free).take(blank_size.to_digit(10).unwrap() as usize));
        file_id += 1;
    }
    disk.extend(
        repeat(Block::File(file_id))
            .take(data.chars().last().unwrap().to_digit(10).unwrap() as usize),
    );
    disk
}

pub fn solve_part1() {
    match open_file("data/09/input.txt") {
        Ok(reader) => {
            let data: String = reader.lines().next().unwrap().unwrap();
            let mut disk: Vec<Block> = read_data_to_disk(&data);

            // Find all free blocks and files in one pass
            let free_blocks: Vec<usize> = disk
                .iter()
                .enumerate()
                .filter(|(_, b)| matches!(b, Block::Free))
                .map(|(i, _)| i)
                .collect();

            let file_blocks: Vec<usize> = disk
                .iter()
                .enumerate()
                .filter(|(_, b)| matches!(b, Block::File(_)))
                .map(|(i, _)| i)
                .rev()
                .collect();

            // Perform swaps
            for (free_idx, file_idx) in free_blocks
                .iter()
                .zip(file_blocks.iter())
                .filter(|(f, file)| f < file)
            {
                disk.swap(*free_idx, *file_idx);
            }
            let checksum = disk
                .iter()
                .enumerate()
                .map(|(i, block)| match block {
                    Block::File(id) => i * id,
                    Block::Free => 0,
                })
                .sum::<usize>();
            println!("Checksum: {}", checksum);
        }
        Err(e) => println!("Error: {:?}", e),
    }
}

pub fn solve_part2() {
    match open_file("data/09/input.txt") {
        Ok(reader) => {
            let data: String = reader.lines().next().unwrap().unwrap();
            let mut disk: Vec<Block> = read_data_to_disk(&data);

            let mut runs = Vec::new();
            let mut ix: usize = 0;
            while ix < disk.len() {
                if let Block::File(current_id) = disk[ix] {
                    let mut run_length = 1;
                    while ix + run_length < disk.len() {
                        if let Block::File(next_id) = disk[ix + run_length] {
                            if next_id == current_id {
                                run_length += 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    runs.push((ix, run_length));
                    ix += run_length;
                } else {
                    ix += 1;
                }
            }

            for &(start, length) in runs.iter().rev() {
                let mut free_count: usize = 0;
                for iy in 0..start {
                    if let Block::Free = disk[iy] {
                        free_count += 1;
                        if free_count >= length {
                            let free_start: usize = iy - length + 1;
                            for (run_pos, free_pos) in (start..start + length)
                                .rev()
                                .zip((free_start..free_start + length).rev())
                            {
                                disk.swap(run_pos, free_pos);
                            }
                            break;
                        }
                    } else {
                        free_count = 0;
                    }
                }
            }
            let mut checksum = 0;
            for (i, block) in disk.iter().enumerate() {
                if let Block::File(id) = block {
                    checksum += i * *id;
                }
            }
            println!("Checksum: {}", checksum);
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
