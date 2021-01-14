use std::collections::HashSet;
use std::io::Error;
use std::num::ParseIntError;

fn main() {
    match read_input() {
        Ok(text) => {
            match solve_part1(&text) {
                Ok(drift) => println!("{}", drift),
                Err(err) => eprintln!("{:#?}", err),
            }
            match solve_part2(&text) {
                Ok(drift) => println!("{}", drift),
                Err(err) => eprintln!("{:#?}", err),
            }
        }
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn solve_part1(input: &str) -> Result<i32, ParseIntError> {
    input
        .lines()
        .map(|line| line.parse())
        .fold(Ok(0), |drift, change| match drift {
            Ok(drift) => match change {
                Ok(change) => Ok(drift + change),
                _ => change,
            },
            _ => drift,
        })
}

fn solve_part2(input: &str) -> Result<i32, ParseIntError> {
    let changes: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut drifts = HashSet::new();
    let mut drift = 0;
    drifts.insert(drift);
    for change in changes.into_iter().cycle() {
        drift += change;
        if !drifts.insert(drift) {
            return Ok(drift);
        }
    }

    unreachable!()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("01.txt")
}
