use std::cmp::{max, min};
use std::collections::HashSet;
use std::iter::zip;
use std::num::ParseIntError;
use std::str::Split;

use super::Day;

pub struct Day14;

type Coordinate = (u32, u32);

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

impl<'a, I> Day<'a, I, u32> for Day14
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(input: I) -> Option<u32> {
        let coordinates = parse_scan(input);
        println!("{coordinates:?}");

        todo!()
    }

    fn part2(_input: I) -> Option<u32> {
        todo!()
    }
}

fn parse_scan<'a, I>(input: I) -> HashSet<Coordinate>
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

    coordinates
}

#[cfg(test)]
mod tests {
    use std::str::Lines;

    use super::{Day, Day14};

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
    fn test1() {
        assert_eq!(Some(24), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
