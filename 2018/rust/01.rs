use std::io::Error;
use std::num::ParseIntError;

fn main() {
    match read_input() {
        Ok(text) => match solve_part1(&text) {
            Ok(drift) => println!("{}", drift),
            Err(err) => eprintln!("{:#?}", err),
        },
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

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("01.txt")
}
