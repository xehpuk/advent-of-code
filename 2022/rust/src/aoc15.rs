use std::str::FromStr;

use super::Day;

pub struct Day15;

type Position = (i32, i32);

#[derive(Debug, PartialEq)]
struct Reading {
    sensor: Position,
    beacon: Position,
}

#[derive(Debug, PartialEq)]
struct ReadError;

impl FromStr for Reading {
    type Err = ReadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = |_| ReadError;

        let s = s.strip_prefix("Sensor at x=").ok_or(ReadError)?;
        let i = s.find(',').ok_or(ReadError)?;
        let x_sensor = s[..i].parse().map_err(err)?;
        let s = &s[i..].strip_prefix(", y=").ok_or(ReadError)?;
        let i = s.find(':').ok_or(ReadError)?;
        let y_sensor = s[..i].parse().map_err(err)?;
        let s = &s[i..]
            .strip_prefix(": closest beacon is at x=")
            .ok_or(ReadError)?;
        let i = s.find(',').ok_or(ReadError)?;
        let x_beacon = s[..i].parse().map_err(err)?;
        let s = &s[i..].strip_prefix(", y=").ok_or(ReadError)?;
        let y_beacon = s.parse().map_err(err)?;

        Ok(Self {
            sensor: (x_sensor, y_sensor),
            beacon: (x_beacon, y_beacon),
        })
    }
}

impl<'a, I> Day<'a, I, u32> for Day15
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(_input: I) -> Option<u32> {
        todo!()
    }

    fn part2(_input: I) -> Option<u32> {
        todo!()
    }
}

fn manhattan_distance(start: &Position, end: &Position) -> u32 {
    start.0.abs_diff(end.0) + start.1.abs_diff(end.1)
}

#[cfg(test)]
mod tests {
    use std::str::Lines;

    use super::{manhattan_distance, Day, Day15, ReadError, Reading};

    const INPUT: &str = include_str!("../../15_test.txt");

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<u32> {
        Day15::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<u32> {
        Day15::part2(test_input(input))
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Err(ReadError), "".parse::<Reading>());
        assert_eq!(Err(ReadError), "Sensor at x=".parse::<Reading>());
        assert_eq!(Err(ReadError), "Sensor at x=2".parse::<Reading>());
        assert_eq!(Err(ReadError), "Sensor at x=2,".parse::<Reading>());
        assert_eq!(Err(ReadError), "Sensor at x=2, y=".parse::<Reading>());
        assert_eq!(Err(ReadError), "Sensor at x=2, y=18".parse::<Reading>());
        assert_eq!(Err(ReadError), "Sensor at x=2, y=18:".parse::<Reading>());
        assert_eq!(
            Err(ReadError),
            "Sensor at x=2, y=18: closest beacon is at x=".parse::<Reading>()
        );
        assert_eq!(
            Err(ReadError),
            "Sensor at x=2, y=18: closest beacon is at x=-2".parse::<Reading>()
        );
        assert_eq!(
            Err(ReadError),
            "Sensor at x=2, y=18: closest beacon is at x=-2,".parse::<Reading>()
        );
        assert_eq!(
            Err(ReadError),
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=".parse::<Reading>()
        );

        assert_eq!(
            Ok(Reading {
                sensor: (2, 18),
                beacon: (-2, 15)
            }),
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".parse::<Reading>()
        );
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(0, manhattan_distance(&(0, 0), &(0, 0)));
        assert_eq!(0, manhattan_distance(&(1, 1), &(1, 1)));
        assert_eq!(0, manhattan_distance(&(-1, -1), &(-1, -1)));

        assert_eq!(1, manhattan_distance(&(0, 0), &(0, 1)));
        assert_eq!(1, manhattan_distance(&(0, 0), &(1, 0)));
        assert_eq!(1, manhattan_distance(&(0, 1), &(0, 0)));
        assert_eq!(1, manhattan_distance(&(1, 0), &(0, 0)));

        assert_eq!(1, manhattan_distance(&(0, 0), &(0, -1)));
        assert_eq!(1, manhattan_distance(&(0, 0), &(-1, 0)));
        assert_eq!(1, manhattan_distance(&(0, -1), &(0, 0)));
        assert_eq!(1, manhattan_distance(&(-1, 0), &(0, 0)));

        assert_eq!(4, manhattan_distance(&(1, 2), &(3, 4)));
        assert_eq!(6, manhattan_distance(&(1, 2), &(-1, -2)));
        assert_eq!(6, manhattan_distance(&(1, 2), &(-2, -1)));
        assert_eq!(10, manhattan_distance(&(1, 2), &(-3, -4)));
    }

    #[test]
    fn test1() {
        assert_eq!(Some(26), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
