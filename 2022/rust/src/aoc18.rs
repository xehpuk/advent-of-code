use std::collections::HashSet;
use std::str::FromStr;

use super::Day;

pub struct Day18;

#[derive(Debug, Eq, Hash, PartialEq)]
struct LavaDroplet {
    x: u8,
    y: u8,
    z: u8,
}

struct LavaDropletError;

impl LavaDroplet {
    fn touches(&self, other: &LavaDroplet) -> bool {
        self.x == other.x && self.y == other.y && self.z.abs_diff(other.z) == 1
            || self.x == other.x && self.z == other.z && self.y.abs_diff(other.y) == 1
            || self.y == other.y && self.z == other.z && self.x.abs_diff(other.x) == 1
    }
}

impl TryFrom<&[u8]> for LavaDroplet {
    type Error = LavaDropletError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
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
            .map(|s| s.parse::<u8>())
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
        let lava_droplets = input
            .map(str::parse::<LavaDroplet>)
            .map(Result::ok)
            .collect::<Option<HashSet<_>>>()?;

        Some(
            lava_droplets
                .iter()
                .map(|lava_droplet| {
                    6 - lava_droplets
                        .iter()
                        .filter(|other| lava_droplet.touches(other))
                        .count()
                })
                .sum(),
        )
    }

    fn part2(_input: I) -> Option<usize> {
        todo!()
    }
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
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
