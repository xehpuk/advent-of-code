use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::Error;
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Coordinates(Vec<Coordinate>);

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

impl From<Vec<Coordinate>> for Coordinates {
    fn from(vec: Vec<Coordinate>) -> Self {
        Self(vec)
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        if vec.is_empty() {
            return Ok(());
        }
        let max_x = vec.iter().map(|c| c.x).max().unwrap();
        let max_y = vec.iter().map(|c| c.y).max().unwrap();
        let mut map: HashMap<u32, HashSet<u32>> = HashMap::with_capacity(vec.len());
        for c in vec.iter() {
            map.entry(c.y).or_default().insert(c.x);
        }

        for y in 0..=max_y {
            let xs = map.get(&y);
            for x in 0..=max_x {
                if let Some(ys) = xs {
                    if ys.contains(&x) {
                        write!(f, "X")?;
                        continue;
                    }
                }
                write!(f, ".")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromIterator<Coordinate> for Coordinates {
    fn from_iter<T: IntoIterator<Item = Coordinate>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl Coordinate {
    fn manhattan_distance(&self, other: &Self) -> u32 {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
    }
}

impl Coordinates {
    fn solve_part1(&self) -> u32 {
        let vec = &self.0;
        let min_x = vec.iter().map(|c| c.x).min().unwrap();
        let min_y = vec.iter().map(|c| c.y).min().unwrap();
        let max_x = vec.iter().map(|c| c.x).max().unwrap();
        let max_y = vec.iter().map(|c| c.y).max().unwrap();
        // size, finite
        let mut areas = vec![(0u32, true); vec.len()];
        for y in min_y..=max_y {
            let edgy = y == min_y || y == max_y;
            for x in min_x..=max_x {
                // coordinate_index, distance, unique
                let mut closest = (0usize, u32::MAX, false);
                for (i, c) in vec.iter().enumerate() {
                    use Ordering::*;

                    let distance = c.manhattan_distance(&Coordinate { x, y });
                    match distance.cmp(&closest.1) {
                        Less => closest = (i, distance, true),
                        Equal => closest.2 = false,
                        Greater => {}
                    }
                }
                if closest.2 {
                    areas[closest.0].0 += 1;
                    areas[closest.0].1 &= !(edgy || x == min_x || x == max_x);
                }
            }
        }

        areas
            .into_iter()
            .filter(|(_, finite)| *finite)
            .map(|(size, _)| size)
            .max()
            .unwrap()
    }

    // This worked for the puzzle but I think it won't for sufficiently high cutoffs.
    // The region may expand the rectangle that's examined here.
    fn solve_part2(&self, cutoff: u32) -> u32 {
        let vec = &self.0;
        let min_x = vec.iter().map(|c| c.x).min().unwrap();
        let min_y = vec.iter().map(|c| c.y).min().unwrap();
        let max_x = vec.iter().map(|c| c.x).max().unwrap();
        let max_y = vec.iter().map(|c| c.y).max().unwrap();
        let mut region_size = 0u32;
        // May want to eliminate the loops to go fully functional (filter, count).
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let current = Coordinate { x, y };
                let summed_distance: u32 = vec.iter().map(|c| current.manhattan_distance(c)).sum();
                if summed_distance < cutoff {
                    region_size += 1;
                }
            }
        }
        region_size
    }
}

pub fn main() {
    match read_input().map(|input| parse_input(&input)) {
        Ok(Ok(coordinates)) => {
            println!("Max area: {}", coordinates.solve_part1());
            println!("Max area: {}", coordinates.solve_part2(10000));
        }
        err => eprintln!("{:#?}", err),
    }
}

fn parse_input(input: &str) -> Result<Coordinates, ParseError> {
    input.lines().map(str::parse).collect()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../06.txt")
}

#[cfg(test)]
mod tests {
    use super::{parse_input, Coordinates};

    const INPUT: &str = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    fn create_test_coordinates() -> Coordinates {
        parse_input(INPUT).unwrap()
    }

    #[test]
    fn test1() {
        let coordinates = create_test_coordinates();

        assert_eq!(17, coordinates.solve_part1());
    }

    #[test]
    fn test2() {
        let coordinates = create_test_coordinates();

        assert_eq!(16, coordinates.solve_part2(32));
    }
}
