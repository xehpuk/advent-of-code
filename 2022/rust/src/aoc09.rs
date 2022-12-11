use super::Day;
use std::collections::HashSet;
use std::str::FromStr;

use Direction::{Down, Left, Right, Up};

pub struct Day09;

struct Motion {
    direction: Direction,
    steps: i32,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct MotionParseError;

impl FromStr for Direction {
    type Err = MotionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Left),
            "R" => Ok(Right),
            "U" => Ok(Up),
            "D" => Ok(Down),
            &_ => Err(MotionParseError),
        }
    }
}

impl FromStr for Motion {
    type Err = MotionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let direction = iter.next().ok_or(MotionParseError)?.parse()?;
        let steps = iter
            .next()
            .ok_or(MotionParseError)?
            .parse()
            .or(Err(MotionParseError))?;

        Ok(Motion { direction, steps })
    }
}

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, usize> for Day09 {
    fn part1(input: I) -> Option<usize> {
        let mut head = (0, 0);
        let mut tail = (0, 0);
        let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
        for motion in input.map(str::parse::<Motion>) {
            match motion.ok()?.direction {
                Left => {}
                Right => {}
                Up => {}
                Down => {}
            }
        }
        todo!()
    }

    fn part2(_input: I) -> Option<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day09};
    use std::str::Split;

    const INPUT: &str = "R 4,U 4,L 3,D 1,R 4,D 1,L 5,R 2";

    fn test_input(input: &str) -> Split<char> {
        input.split(',')
    }

    fn test_part1(input: &str) -> Option<usize> {
        Day09::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<usize> {
        Day09::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(13), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
