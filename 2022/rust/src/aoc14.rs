use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::iter::{successors, zip};
use std::num::ParseIntError;
use std::str::Split;

use super::Day;

pub struct Day14;

type Coordinate = (u32, u32);

const SAND_START: Coordinate = (500, 0);

#[derive(Clone)]
struct Path<'a, I>(I)
where
    I: Iterator<Item = &'a str>;

impl<'a> Path<'a, Split<'a, &str>> {
    fn new(s: &'a str) -> Self {
        Path(s.split(" -> "))
    }
}

impl<'a, I> Iterator for Path<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = Result<Coordinate, ParseIntError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inner = self.0.next()?.split(',');
        let mut parse = || inner.next().map(str::parse);

        let x: u32 = match parse()? {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        };
        let y: u32 = match parse()? {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        };

        Some(Ok((x, y)))
    }
}

struct Coordinates {
    inner: HashSet<Coordinate>,
    x_min: u32,
    x_max: u32,
    y_max: u32,
}

impl Coordinates {
    fn new(inner: HashSet<Coordinate>) -> Self {
        let x_min = inner.iter().map(|c| c.0).min().unwrap();
        let x_max = inner.iter().map(|c| c.0).max().unwrap();
        let y_max = inner.iter().map(|c| c.1).max().unwrap();

        Coordinates {
            inner,
            x_min,
            x_max,
            y_max,
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self {
            inner,
            x_min,
            x_max,
            y_max,
        } = self;

        let x_length = number_of_digits(*x_max);
        let y_length = number_of_digits(*y_max);

        let padding = " ".repeat(y_length + 1);

        for x_i in (0..x_length).map(|i| i as u32).rev() {
            f.write_str(&padding)?;
            for x in *x_min..=*x_max {
                f.write_char((b'0' + digit_i(x, x_i)) as char)?;
            }
            f.write_char('\n')?;
        }

        for y in 0..=*y_max {
            f.write_str(&format!("{:>y_length$} ", y))?;
            for x in *x_min..=*x_max {
                let coordinate = (x, y);
                f.write_char(if coordinate == SAND_START {
                    '+'
                } else if inner.contains(&coordinate) {
                    '#'
                } else {
                    '.'
                })?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl<'a, I> Day<'a, I, u32> for Day14
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(input: I) -> Option<u32> {
        let mut coordinates = parse_scan(input);

        for i in 0.. {
            let mut sand = SAND_START;
            loop {
                if sand.1 >= coordinates.y_max {
                    return Some(i);
                }
                let current = sand;
                sand.1 += 1;
                if !coordinates.inner.contains(&sand) {
                    continue;
                }
                sand.0 -= 1;
                if !coordinates.inner.contains(&sand) {
                    continue;
                }
                sand.0 += 2;
                if !coordinates.inner.contains(&sand) {
                    continue;
                }
                coordinates.inner.insert(current);
                break;
            }
        }

        unreachable!()
    }

    fn part2(input: I) -> Option<u32> {
        let mut coordinates = parse_scan(input);

        for i in 0.. {
            let mut sand = SAND_START;
            loop {
                if coordinates.inner.contains(&sand) {
                    return Some(i);
                }
                if sand.1 >= coordinates.y_max + 1 {
                    coordinates.inner.insert(sand);
                    break;
                }
                let current = sand;
                sand.1 += 1;
                if !coordinates.inner.contains(&sand) {
                    continue;
                }
                sand.0 -= 1;
                if !coordinates.inner.contains(&sand) {
                    continue;
                }
                sand.0 += 2;
                if !coordinates.inner.contains(&sand) {
                    continue;
                }
                coordinates.inner.insert(current);
                break;
            }
        }

        unreachable!()
    }
}

fn parse_scan<'a, I>(input: I) -> Coordinates
where
    I: Iterator<Item = &'a str>,
{
    let mut coordinates = HashSet::new();

    for path in input.map(Path::new) {
        let coordinates_iter = path.flat_map(Result::ok);
        for ((from_x, from_y), (to_x, to_y)) in
            zip(coordinates_iter.clone(), coordinates_iter.skip(1))
        {
            if from_x == to_x {
                for y in min(from_y, to_y)..=max(from_y, to_y) {
                    coordinates.insert((from_x, y));
                }
            } else {
                for x in min(from_x, to_x)..=max(from_x, to_x) {
                    coordinates.insert((x, from_y));
                }
            }
        }
    }

    Coordinates::new(coordinates)
}

fn number_of_digits(n: u32) -> usize {
    successors(Some(n), |&n| (n >= 10).then_some(n / 10)).count()
}

fn digit_i(n: u32, i: u32) -> u8 {
    (n % 10u32.pow(i + 1) / 10u32.pow(i)) as u8
}

#[cfg(test)]
mod tests {
    use std::str::Lines;

    use super::{digit_i, number_of_digits, Day, Day14};

    const INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<u32> {
        Day14::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<u32> {
        Day14::part2(test_input(input))
    }

    #[test]
    fn test_number_of_digits() {
        assert_eq!(1, number_of_digits(0));
        assert_eq!(1, number_of_digits(1));
        assert_eq!(1, number_of_digits(9));
        assert_eq!(2, number_of_digits(10));
        assert_eq!(2, number_of_digits(11));
        assert_eq!(2, number_of_digits(99));
        assert_eq!(3, number_of_digits(100));
        assert_eq!(3, number_of_digits(101));
        assert_eq!(3, number_of_digits(999));
        assert_eq!(4, number_of_digits(1000));
        assert_eq!(4, number_of_digits(1001));
        assert_eq!(10, number_of_digits(u32::MAX));
    }

    #[test]
    fn test_digit_i() {
        assert_eq!(0, digit_i(0, 0));
        assert_eq!(1, digit_i(1, 0));
        assert_eq!(9, digit_i(9, 0));
        assert_eq!(0, digit_i(0, 1));
        assert_eq!(0, digit_i(9, 1));

        assert_eq!(4, digit_i(1234, 0));
        assert_eq!(3, digit_i(1234, 1));
        assert_eq!(2, digit_i(1234, 2));
        assert_eq!(1, digit_i(1234, 3));
        assert_eq!(0, digit_i(1234, 4));

        assert_eq!(1, digit_i(4321, 0));
        assert_eq!(2, digit_i(4321, 1));
        assert_eq!(3, digit_i(4321, 2));
        assert_eq!(4, digit_i(4321, 3));
        assert_eq!(0, digit_i(4321, 4));
    }

    #[test]
    fn test1() {
        assert_eq!(Some(24), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(Some(93), test_part2(INPUT));
    }
}
