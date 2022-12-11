use super::Day;

use std::str::FromStr;
use Instruction::{Addx, Noop};

pub struct Day10;

#[derive(Debug, PartialEq)]
enum Instruction {
    Addx(i32),
    Noop,
}

#[derive(Debug, PartialEq)]
struct InstructionParseError;

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let op = iter.next().ok_or(InstructionParseError)?;

        match op {
            "addx" => {
                let v = iter
                    .next()
                    .ok_or(InstructionParseError)?
                    .parse()
                    .or(Err(InstructionParseError))?;

                if iter.next().is_some() {
                    Err(InstructionParseError)
                } else {
                    Ok(Addx(v))
                }
            }
            "noop" => {
                if iter.next().is_some() {
                    Err(InstructionParseError)
                } else {
                    Ok(Noop)
                }
            }
            &_ => Err(InstructionParseError),
        }
    }
}

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, i32> for Day10 {
    fn part1(_input: I) -> Option<i32> {
        todo!()
    }

    fn part2(_input: I) -> Option<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction::{Addx, Noop};
    use super::{Day, Day10};
    use super::{Instruction, InstructionParseError};

    const INPUT: &str = include_str!("../../10_test.txt");

    fn test_input(input: &str) -> impl Iterator<Item = &str> + Clone {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<i32> {
        Day10::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<i32> {
        Day10::part2(test_input(input))
    }

    #[test]
    fn test_parse() {
        assert_eq!(Ok(Addx(0)), "addx 0".parse());
        assert_eq!(Ok(Addx(1337)), "addx 1337".parse());
        assert_eq!(Ok(Addx(-42)), "addx -42".parse());
        assert_eq!(Ok(Noop), "noop".parse());

        assert_eq!(Err(InstructionParseError), "addx".parse::<Instruction>());
        assert_eq!(
            Err(InstructionParseError),
            "addx 1337 42".parse::<Instruction>()
        );
        assert_eq!(Err(InstructionParseError), "noop 0".parse::<Instruction>());
        assert_eq!(
            Err(InstructionParseError),
            "noop 1337".parse::<Instruction>()
        );
        assert_eq!(
            Err(InstructionParseError),
            "noop -42".parse::<Instruction>()
        );
    }

    #[test]
    fn test1() {
        assert_eq!(Some(13140), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
