use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_parser = clap::value_parser!(u32).range(1..=25), default_value_t = 3)]
    day: u32,

    #[arg(value_parser = clap::value_parser!(u32).range(1..=2), default_value_t = 1)]
    part: u32,
}

fn main() {
    let args = Args::parse();
    match (args.day, args.part) {
        (1, 1) => advent::day01::solve_part1(),
        (1, 2) => advent::day01::solve_part2(),
        (2, 1) => advent::day02::solve_part1(),
        (2, 2) => advent::day02::solve_part2(),
        (3, 1) => advent::day03::solve_part1(),
        (3, 2) => advent::day03::solve_part2(),
        (4, 1) => advent::day04::solve_part1(),
        (4, 2) => advent::day04::solve_part2(),
        (5, 1) => advent::day05::solve_part1(),
        (5, 2) => advent::day05::solve_part2(),
        (6, 1) => advent::day06::solve_part1(),
        (6, 2) => advent::day06::solve_part2(),
        (7, 1) => advent::day07::solve_part1(),
        (7, 2) => advent::day07::solve_part2(),
        (8, 1) => advent::day08::solve_part1(),
        (8, 2) => advent::day08::solve_part2(),
        (9, 1) => advent::day09::solve_part1(),
        (9, 2) => advent::day09::solve_part2(),
        (10, 1) => advent::day10::solve_part1(),
        (10, 2) => advent::day10::solve_part2(),
        (11, 1) => advent::day11::solve_part1(),
        (11, 2) => advent::day11::solve_part2(),
        _ => println!("Invalid day or part number, {}:{}", args.day, args.part),
    }
}
