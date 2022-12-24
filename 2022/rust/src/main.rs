use std::env::args;
use std::fmt::Display;
use std::io::Error;

use aoc01::Day01;
use aoc02::Day02;
use aoc03::Day03;
use aoc04::Day04;
use aoc05::Day05;
use aoc06::Day06;
use aoc07::Day07;
use aoc08::Day08;
use aoc09::Day09;
use aoc10::Day10;
use aoc11::Day11;
use aoc12::Day12;
use aoc14::Day14;
use aoc15::Day15;
use aoc18::Day18;
use aoc20::Day20;
use aoc21::Day21;
use aoc23::Day23;

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;
mod aoc06;
mod aoc07;
mod aoc08;
mod aoc09;
mod aoc10;
mod aoc11;
mod aoc12;
mod aoc14;
mod aoc15;
mod aoc18;
mod aoc20;
mod aoc21;
mod aoc23;

const LATEST_DAY: u8 = 23;

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
        6 => Day06::solve(lines),
        7 => Day07::solve(lines),
        8 => Day08::solve(lines),
        9 => Day09::solve(lines),
        10 => Day10::solve(lines),
        11 => Day11::solve(lines),
        12 => Day12::solve(lines),
        14 => Day14::solve(lines),
        15 => Day15::solve(lines),
        18 => Day18::solve(lines),
        20 => Day20::solve(lines),
        21 => Day21::solve(lines),
        23 => Day23::solve(lines),
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
