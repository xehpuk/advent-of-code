use aoc01::Day01;
use aoc02::Day02;
use aoc03::Day03;
use aoc04::Day04;
use std::env::args;
use std::io::Error;
use std::str::Lines;

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;

const LATEST_DAY: u8 = 4;

trait Day<'a, I: Clone + Iterator<Item = &'a str>> {
    fn part1(input: I) -> Option<i32>;
    fn part2(input: I) -> Option<i32>;

    fn solve(input: I) {
        Self::solve_part(Self::part1, input.clone());
        Self::solve_part(Self::part2, input);
    }

    fn solve_part(part: fn(I) -> Option<i32>, input: I) {
        match part(input) {
            Some(solution) => println!("{}", solution),
            None => eprintln!("No solution!"),
        }
    }
}

fn main() -> Result<(), Error> {
    let day = determine_day();
    let input = std::fs::read_to_string(format!("../{:0>2}.txt", day))?;

    match day {
        1 => solve_day::<Day01>(&input),
        2 => solve_day::<Day02>(&input),
        3 => solve_day::<Day03>(&input),
        4 => solve_day::<Day04>(&input),
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

fn solve_day<'a, D: Day<'a, Lines<'a>>>(input: &'a str) {
    D::solve(input.lines())
}
