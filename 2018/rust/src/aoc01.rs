use std::collections::HashSet;
use std::io::Error;
use std::num::ParseIntError;

pub fn main() {
    match read_input() {
        Ok(text) => {
            solve(part1, &text);
            solve(part2, &text);
        }
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn solve(part: fn(&str) -> Result<i32, ParseIntError>, text: &str) {
    match part(text) {
        Ok(drift) => println!("{}", drift),
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn part1(input: &str) -> Result<i32, ParseIntError> {
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

fn part2(input: &str) -> Result<i32, ParseIntError> {
    let mut drifts = HashSet::new();
    let mut drift = 0;
    drifts.insert(drift);
    for change in input.lines().map(|line| line.parse::<i32>()).cycle() {
        drift += change?;
        if !drifts.insert(drift) {
            return Ok(drift);
        }
    }

    unreachable!()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../01.txt")
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use std::num::ParseIntError;

    fn test_input(input: &str) -> String {
        input.split(", ").collect::<Vec<_>>().join("\n")
    }

    fn test_part1(input: &str) -> Result<i32, ParseIntError> {
        part1(&test_input(input))
    }

    fn test_part2(input: &str) -> Result<i32, ParseIntError> {
        part2(&test_input(input))
    }

    #[test]
    fn test1a() {
        assert_eq!(Ok(3), test_part1("+1, -2, +3, +1"));
    }

    #[test]
    fn test1b() {
        assert_eq!(Ok(3), test_part1("+1, -2, +3, +1"));
    }

    #[test]
    fn test1c() {
        assert_eq!(Ok(0), test_part1("+1, +1, -2"));
    }

    #[test]
    fn test1d() {
        assert_eq!(Ok(-6), test_part1("-1, -2, -3"));
    }

    #[test]
    fn test2a() {
        assert_eq!(Ok(0), test_part2("+1, -1"));
    }

    #[test]
    fn test2b() {
        assert_eq!(Ok(10), test_part2("+3, +3, +4, -2, -4"));
    }

    #[test]
    fn test2c() {
        assert_eq!(Ok(5), test_part2("-6, +3, +8, +5, -6"));
    }

    #[test]
    fn test2d() {
        assert_eq!(Ok(14), test_part2("+7, +7, -2, -7, -4"));
    }
}
