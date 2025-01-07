use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn open_file(filepath: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let path: &Path = Path::new(filepath);
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);
    Ok(reader)
}

#[derive(Clone, Debug)]
struct Stone {
    value: i64,
}

impl Stone {
    fn blink(&self) -> Vec<Self> {
        if self.value == 0 {
            return vec![Self { value: 1 }];
        }
        let digits = self.value.to_string();
        let n_digits = digits.len();
        if n_digits % 2 == 0 {
            let (left_half, right_half) = digits.split_at(n_digits / 2);
            return vec![
                Self {
                    value: left_half.parse::<i64>().unwrap(),
                },
                Self {
                    value: right_half.parse::<i64>().unwrap(),
                },
            ];
        }
        return vec![Self {
            value: self.value * 2024,
        }];
    }
}

pub fn solve_part1() {
    let result = open_file("data/11/input.txt").and_then(|reader| {
        let mut data = reader
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| Stone {
                value: s.parse::<i64>().unwrap(),
            })
            .collect::<Vec<_>>();
        let mut blink_counter = 0;
        while blink_counter < 25 {
            let mut new_data = Vec::new();
            for stone in &data {
                let new_stones = stone.blink();
                new_data.extend(new_stones);
            }
            blink_counter += 1;
            data = new_data;
        }
        Ok(data.len())
    });
    match result {
        Ok(n_stones) => println!("Number of stones: {}", n_stones),
        Err(err) => eprintln!("Error: {}", err),
    }
}
pub fn solve_part2() {}
