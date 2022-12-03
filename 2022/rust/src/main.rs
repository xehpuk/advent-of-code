use std::env::args;
use std::io::Error;

mod aoc01;
mod aoc02;

type Part = fn(&str) -> Option<i32>;

const LATEST_DAY: u8 = 2;

fn main() -> Result<(), Error> {
    let day = determine_day();
    let input = std::fs::read_to_string(format!("../{:0>2}.txt", day))?;
    match day {
        1 => solve_day(aoc01::part1, aoc01::part2, &input),
        2 => solve_day(aoc02::part1, aoc02::part2, &input),
        _ => unreachable!(),
    }

    Ok(())
}

fn determine_day() -> u8 {
    match args().nth(1) {
        Some(day) => {
            let day = day.parse().expect("Day must be a number.");
            if !(1..=LATEST_DAY).contains(&day) {
                panic!(
                    "Number must be in 1..={}, {} clearly isn't.",
                    LATEST_DAY, day
                );
            }
            day
        }
        None => {
            println!(
                "Running latest exercise (day {}). You may provide a number, e.g.: cargo run -- 1",
                LATEST_DAY
            );
            LATEST_DAY
        }
    }
}

fn solve_day(part1: Part, part2: Part, input: &str) {
    solve_part(part1, input);
    solve_part(part2, input);
}

fn solve_part(part: fn(&str) -> Option<i32>, input: &str) {
    match part(input) {
        Some(result) => println!("{}", result),
        None => eprintln!("No result!"),
    }
}
