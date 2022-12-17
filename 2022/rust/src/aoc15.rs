use std::cmp::{max, min};
use std::collections::HashSet;
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

impl Day15 {
    // part 1
    const Y: i32 = if cfg!(test) { 10 } else { 2_000_000 };
    // part 2
    const MAX: i32 = if cfg!(test) { 20 } else { 4_000_000 };
}

impl<'a, I> Day<'a, I, usize> for Day15
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(input: I) -> Option<usize> {
        let readings = input
            .map(str::parse::<Reading>)
            .map(Result::ok)
            .map(|r| r.map(|r| (manhattan_distance(&r.sensor, &r.beacon), r)))
            .collect::<Option<Vec<_>>>()?;

        let beacons = readings
            .iter()
            .map(|(_, r)| r.beacon)
            .collect::<HashSet<_>>();

        let x_min = readings.iter().map(|(d, r)| r.sensor.0 - *d as i32).min()?;
        let x_max = readings.iter().map(|(d, r)| r.sensor.0 + *d as i32).max()?;

        Some(
            (x_min..=x_max)
                .map(|x| (x, Self::Y) as Position)
                .filter(|p| {
                    !beacons.contains(p)
                        && readings
                            .iter()
                            .any(|(d, r)| manhattan_distance(p, &r.sensor) <= *d)
                })
                .count(),
        )
    }

    fn part2(input: I) -> Option<usize> {
        let readings = input
            .map(str::parse::<Reading>)
            .map(Result::ok)
            .map(|r| r.map(|r| (manhattan_distance(&r.sensor, &r.beacon), r.sensor)))
            .collect::<Option<Vec<_>>>()?;

        let check = |x, y| {
            let p = (x, y);
            if readings.iter().all(|(d, r)| manhattan_distance(r, &p) > *d) {
                return Some(p.0 as usize * 4_000_000 + p.1 as usize);
            }
            None
        };

        for (distance, sensor) in &readings {
            let distance = *distance as i32;
            for x in max(0, sensor.0)..=min(Self::MAX, sensor.0 + 1 + distance) {
                let x_i = x - sensor.0;

                let y_min = sensor.1 + x_i - 1 - distance;
                if y_min < 0 {
                    continue;
                }
                if let Some(result) = check(x, y_min) {
                    return Some(result);
                }

                let y_max = sensor.1 - x_i + 1 + distance;
                if y_max > Self::MAX {
                    break;
                }
                if let Some(result) = check(x, y_max) {
                    return Some(result);
                }
            }
            for x in max(0, sensor.0 - distance - 1)..=min(Self::MAX, sensor.0 - 1) {
                let x_i = x - (sensor.0 - distance - 1);

                let y_min = sensor.1 - x_i;
                if y_min < 0 {
                    continue;
                }
                if let Some(result) = check(x, y_min) {
                    return Some(result);
                }

                let y_max = sensor.1 + x_i;
                if y_max > Self::MAX {
                    break;
                }
                if let Some(result) = check(x, y_max) {
                    return Some(result);
                }
            }
        }

        None
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

    fn test_part1(input: &str) -> Option<usize> {
        Day15::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<usize> {
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
        assert_eq!(Some(56000011), test_part2(INPUT));
    }
}
