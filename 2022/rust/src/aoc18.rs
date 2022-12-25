use std::cmp::Ordering;
use std::collections::HashSet;
use std::str::FromStr;

use super::Day;

pub struct Day18;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Cube {
    x: i8,
    y: i8,
    z: i8,
}

#[derive(Debug)]
struct LavaDroplet {
    cubes: HashSet<Cube>,
    cube_min: Cube,
    cube_max: Cube,
}

/// https://github.com/rust-lang/rust/issues/50133
struct InputWrapper<'a, I>(I)
where
    I: Iterator<Item = &'a str>;

struct LavaDropletError;

impl Cube {
    fn neighbors(&self) -> Vec<Self> {
        vec![
            Self {
                z: self.z + 1,
                ..*self
            },
            Self {
                y: self.y + 1,
                ..*self
            },
            Self {
                x: self.x + 1,
                ..*self
            },
            Self {
                z: self.z - 1,
                ..*self
            },
            Self {
                y: self.y - 1,
                ..*self
            },
            Self {
                x: self.x - 1,
                ..*self
            },
        ]
    }
}

impl TryFrom<&[i8]> for Cube {
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

impl FromStr for Cube {
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

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x == other.x && self.y == other.y && self.z == other.z {
            return Some(Ordering::Equal);
        }
        if self.x <= other.x && self.y <= other.y && self.z <= other.z {
            return Some(Ordering::Less);
        }
        if self.x >= other.x && self.y >= other.y && self.z >= other.z {
            return Some(Ordering::Greater);
        }
        None
    }
}

impl<'a, I> TryFrom<InputWrapper<'a, I>> for LavaDroplet
where
    I: Iterator<Item = &'a str>,
{
    type Error = LavaDropletError;

    fn try_from(value: InputWrapper<'a, I>) -> Result<Self, Self::Error> {
        let mut cubes = HashSet::new();
        let mut cube_min = Cube {
            x: i8::MAX,
            y: i8::MAX,
            z: i8::MAX,
        };
        let mut cube_max = Cube {
            x: i8::MIN,
            y: i8::MIN,
            z: i8::MIN,
        };
        for cube in value.0.map(str::parse::<Cube>) {
            let cube = cube?;
            cube_min.x = cube_min.x.min(cube.x - 1);
            cube_min.y = cube_min.y.min(cube.y - 1);
            cube_min.z = cube_min.z.min(cube.z - 1);
            cube_max.x = cube_max.x.max(cube.x + 1);
            cube_max.y = cube_max.y.max(cube.y + 1);
            cube_max.z = cube_max.z.max(cube.z + 1);
            cubes.insert(cube);
        }

        Ok(LavaDroplet {
            cubes,
            cube_min,
            cube_max,
        })
    }
}

impl<'a, I> Day<'a, I, usize> for Day18
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(input: I) -> Option<usize> {
        let lava_droplet: LavaDroplet = InputWrapper(input).try_into().ok()?;

        Some(
            lava_droplet
                .cubes
                .iter()
                .flat_map(Cube::neighbors)
                .filter(|neighbor| !lava_droplet.cubes.contains(neighbor))
                .count(),
        )
    }

    fn part2(input: I) -> Option<usize> {
        let lava_droplet: LavaDroplet = InputWrapper(input).try_into().ok()?;

        let mut exterior_air_cubes = HashSet::<Cube>::new();
        let mut queue = vec![lava_droplet.cube_min.clone()];

        while !queue.is_empty() {
            let current = queue.pop()?;
            let neighbors = current.neighbors();
            exterior_air_cubes.insert(current);

            for neighbor in neighbors {
                #[allow(clippy::neg_cmp_op_on_partial_ord)]
                if exterior_air_cubes.contains(&neighbor)
                    || lava_droplet.cubes.contains(&neighbor)
                    || !(neighbor >= lava_droplet.cube_min)
                    || !(neighbor <= lava_droplet.cube_max)
                {
                    continue;
                }
                exterior_air_cubes.insert(neighbor.clone());
                queue.push(neighbor);
            }
        }

        Some(
            lava_droplet
                .cubes
                .iter()
                .flat_map(Cube::neighbors)
                .filter(|neighbor| exterior_air_cubes.contains(neighbor))
                .count(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Cube, Day, Day18};
    use std::cmp::Ordering;
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
    fn test_partial_cmp() {
        assert_eq!(
            Some(Ordering::Equal),
            Cube { x: 0, y: 0, z: 0 }.partial_cmp(&Cube { x: 0, y: 0, z: 0 })
        );
        assert_eq!(
            Some(Ordering::Equal),
            Cube { x: 1, y: 0, z: 0 }.partial_cmp(&Cube { x: 1, y: 0, z: 0 })
        );
        assert_eq!(
            Some(Ordering::Equal),
            Cube { x: 0, y: 1, z: 0 }.partial_cmp(&Cube { x: 0, y: 1, z: 0 })
        );
        assert_eq!(
            Some(Ordering::Equal),
            Cube { x: 0, y: 0, z: 1 }.partial_cmp(&Cube { x: 0, y: 0, z: 1 })
        );
        assert_eq!(
            Some(Ordering::Less),
            Cube { x: 0, y: 0, z: 0 }.partial_cmp(&Cube { x: 1, y: 0, z: 0 })
        );
        assert_eq!(
            Some(Ordering::Less),
            Cube { x: 0, y: 0, z: 0 }.partial_cmp(&Cube { x: 0, y: 1, z: 0 })
        );
        assert_eq!(
            Some(Ordering::Less),
            Cube { x: 0, y: 0, z: 0 }.partial_cmp(&Cube { x: 0, y: 0, z: 1 })
        );
        assert_eq!(
            Some(Ordering::Greater),
            Cube { x: 1, y: 0, z: 0 }.partial_cmp(&Cube { x: 0, y: 0, z: 0 })
        );
        assert_eq!(
            Some(Ordering::Greater),
            Cube { x: 0, y: 1, z: 0 }.partial_cmp(&Cube { x: 0, y: 0, z: 0 })
        );
        assert_eq!(
            Some(Ordering::Greater),
            Cube { x: 0, y: 0, z: 1 }.partial_cmp(&Cube { x: 0, y: 0, z: 0 })
        );
        assert_eq!(
            None,
            Cube { x: 1, y: 0, z: 0 }.partial_cmp(&Cube { x: 0, y: 1, z: 0 })
        );
        assert_eq!(
            None,
            Cube { x: 1, y: 0, z: 0 }.partial_cmp(&Cube { x: 0, y: 0, z: 1 })
        );
        assert_eq!(
            None,
            Cube { x: 0, y: 1, z: 0 }.partial_cmp(&Cube { x: 1, y: 0, z: 0 })
        );
        assert_eq!(
            None,
            Cube { x: 0, y: 1, z: 0 }.partial_cmp(&Cube { x: 0, y: 0, z: 1 })
        );
        assert_eq!(
            None,
            Cube { x: 0, y: 0, z: 1 }.partial_cmp(&Cube { x: 1, y: 0, z: 0 })
        );
        assert_eq!(
            None,
            Cube { x: 0, y: 0, z: 1 }.partial_cmp(&Cube { x: 0, y: 1, z: 0 })
        );
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
