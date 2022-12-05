use std::env::args;
use std::fmt::Display;
use std::io::Error;

use aoc01::Day01;
use aoc02::Day02;
use aoc03::Day03;
use aoc04::Day04;
use aoc05::Day05;

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;

const LATEST_DAY: u8 = 5;

trait Day<'a, I: Clone + Iterator<Item = &'a str>, T: Display> {
    fn part1(input: I) -> Option<T>;
    fn part2(input: I) -> Option<T>;

    fn solve(input: I) {
        Self::solve_part(Self::part1, input.clone());
        Self::solve_part(Self::part2, input);
    }

    fn solve_part(part: fn(I) -> Option<T>, input: I) {
        match part(input) {
            Some(solution) => println!("{}", solution),
            None => eprintln!("No solution!"),
        }
    }
}

fn main() -> Result<(), Error> {
    let day = determine_day();
    let input = std::fs::read_to_string(format!("../{:0>2}.txt", day))?;
    let lines = input.lines();

    match day {
        1 => Day01::solve(lines),
        2 => Day02::solve(lines),
        3 => Day03::solve(lines),
        4 => Day04::solve(lines),
        5 => Day05::solve(lines),
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
