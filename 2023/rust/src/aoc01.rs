use anyhow::Result;
use thiserror::Error;

const INPUT: &str = include_str!("../../01.txt");

pub fn solve() {
    println!("day 01, part 1: {}", part1(INPUT).unwrap());
    println!("day 01, part 2: {}", part2(INPUT).unwrap());
}

#[derive(Debug, Error)]
enum AocError {
    #[error("no digit found")]
    NoDigitFound,
}

fn part1(input: &str) -> Result<i32> {
    fn parse_line(line: &str) -> Result<i32> {
        let first_digit = line
            .chars()
            .find_map(|c| c.to_digit(10).map(|d| d as i32))
            .ok_or(AocError::NoDigitFound)?;
        let last_digit = line
            .chars()
            .rev()
            .find_map(|c| c.to_digit(10).map(|d| d as i32))
            .unwrap();

        Ok(10 * first_digit + last_digit)
    }

    input.lines().map(parse_line).sum::<Result<_>>()
}

fn part2(input: &str) -> Result<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT1: &str = include_str!("../../01_test1.txt");
    const INPUT2: &str = include_str!("../../01_test2.txt");

    #[test]
    fn test1() {
        assert_eq!(142, part1(INPUT1).unwrap());
    }

    #[test]
    fn test2() {
        assert_eq!(281, part2(INPUT2).unwrap());
    }
}
