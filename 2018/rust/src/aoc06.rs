use std::io::Error;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum ParseError {
    TooFew,
    TooMany,
    Int(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        Self::Int(err)
    }
}

impl FromStr for Coordinate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(", ").map(str::parse);
        let x = coords.next().ok_or(Self::Err::TooFew)??;
        let y = coords.next().ok_or(Self::Err::TooFew)??;
        if coords.next().is_some() {
            Err(Self::Err::TooMany)
        } else {
            Ok(Self { x, y })
        }
    }
}

pub fn main() {
    match read_input().map(|input| parse_input(&input)) {
        Ok(Ok(coordinates)) => {
            println!("{:#?}", coordinates);
        }
        err => eprintln!("{:#?}", err),
    }
}

#[allow(dead_code, unused_variables)]
fn solve_part1(coordinates: &[Coordinate]) -> u32 {
    todo!()
}

#[allow(dead_code, unused_variables)]
fn solve_part2(coordinates: &[Coordinate]) -> u32 {
    todo!()
}

fn parse_input(input: &str) -> Result<Vec<Coordinate>, ParseError> {
    input.lines().map(str::parse).collect()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../06.txt")
}

#[cfg(test)]
mod tests {
    use super::{parse_input, solve_part1, solve_part2, Coordinate};

    const INPUT: &str = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    fn create_test_coordinates() -> Vec<Coordinate> {
        parse_input(&INPUT).unwrap()
    }

    #[test]
    fn test1a() {
        let coordinates = create_test_coordinates();

        assert_eq!(17, solve_part1(&coordinates));
    }
}
