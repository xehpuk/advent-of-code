use anyhow::Result;
use thiserror::Error;

const INPUT: &str = include_str!("../../_input/01.txt");

struct Digit(i32, &'static str);

static DIGITS: [Digit; 9] = [
    Digit(1, "one"),
    Digit(2, "two"),
    Digit(3, "three"),
    Digit(4, "four"),
    Digit(5, "five"),
    Digit(6, "six"),
    Digit(7, "seven"),
    Digit(8, "eight"),
    Digit(9, "nine"),
];

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

    input.lines().map(parse_line).sum()
}

fn part2(input: &str) -> Result<i32> {
    fn parse_line(line: &str) -> Result<i32> {
        let first_digit: i32 = line
            .char_indices()
            .find_map(|(i, c)| {
                c.to_digit(10).map(|d| d as i32).or_else(|| {
                    DIGITS.iter().find_map(|&Digit(figure, numeral)| {
                        if line[i..].starts_with(numeral) {
                            Some(figure)
                        } else {
                            None
                        }
                    })
                })
            })
            .ok_or(AocError::NoDigitFound)?;
        let last_digit = line
            .char_indices()
            .rev()
            .find_map(|(i, c)| {
                c.to_digit(10).map(|d| d as i32).or_else(|| {
                    DIGITS.iter().find_map(|&Digit(figure, numeral)| {
                        if line[..(i + c.len_utf8())].ends_with(numeral) {
                            Some(figure)
                        } else {
                            None
                        }
                    })
                })
            })
            .unwrap();

        Ok(10 * first_digit + last_digit)
    }

    input.lines().map(parse_line).sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT1: &str = include_str!("../../_input/01_test1.txt");
    const INPUT2: &str = include_str!("../../_input/01_test2.txt");

    #[test]
    fn test1() {
        assert_eq!(142, part1(INPUT1).unwrap());
    }

    #[test]
    fn test2() {
        assert_eq!(281, part2(INPUT2).unwrap());
    }
}
