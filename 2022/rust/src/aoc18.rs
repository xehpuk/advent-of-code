use std::collections::HashSet;
use std::str::FromStr;

use super::Day;

pub struct Day18;

#[derive(Debug, Eq, Hash, PartialEq)]
struct LavaDroplet {
    x: i8,
    y: i8,
    z: i8,
}

struct LavaDropletError;

impl LavaDroplet {
    fn _touches(&self, other: &LavaDroplet) -> bool {
        self.x == other.x && self.y == other.y && self.z.abs_diff(other.z) == 1
            || self.x == other.x && self.z == other.z && self.y.abs_diff(other.y) == 1
            || self.y == other.y && self.z == other.z && self.x.abs_diff(other.x) == 1
    }

    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::with_capacity(6);

        neighbors.push(Self {
            z: self.z + 1,
            ..*self
        });
        neighbors.push(Self {
            y: self.y + 1,
            ..*self
        });
        neighbors.push(Self {
            x: self.x + 1,
            ..*self
        });
        neighbors.push(Self {
            z: self.z - 1,
            ..*self
        });
        neighbors.push(Self {
            y: self.y - 1,
            ..*self
        });
        neighbors.push(Self {
            x: self.x - 1,
            ..*self
        });

        neighbors
    }
}

impl TryFrom<&[i8]> for LavaDroplet {
    type Error = LavaDropletError;

    fn try_from(value: &[i8]) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(LavaDropletError);
        }

        Ok(Self {
            x: value[0],
            y: value[1],
            z: value[2],
        })
    }
}

impl FromStr for LavaDroplet {
    type Err = LavaDropletError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let droplet_parts = s
            .splitn(3, ',')
            .map(|s| s.parse::<i8>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| LavaDropletError)?;

        droplet_parts.as_slice().try_into()
    }
}

impl<'a, I> Day<'a, I, usize> for Day18
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(input: I) -> Option<usize> {
        let lava_droplets = parse_lava_droplets(input)?;

        Some(
            lava_droplets
                .iter()
                .flat_map(LavaDroplet::neighbors)
                .filter(|neighbor| !lava_droplets.contains(neighbor))
                .count(),
        )
    }

    fn part2(_input: I) -> Option<usize> {
        todo!()
    }
}

fn parse_lava_droplets<'a, I>(input: I) -> Option<HashSet<LavaDroplet>>
where
    I: Clone + Iterator<Item = &'a str>,
{
    input
        .map(str::parse::<LavaDroplet>)
        .map(Result::ok)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Day, Day18};
    use std::str::Lines;

    const INPUT: &str = include_str!("../../18_test.txt");

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<usize> {
        Day18::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<usize> {
        Day18::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(64), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(Some(58), test_part2(INPUT));
    }
}
