use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fmt::Write;

use super::Day;

pub struct Day23;

type Number = usize;
type Coord = i32;
type Elf = (Coord, Coord);

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

struct Elves<'a>(&'a HashSet<Elf>);

impl<'a> Display for Elves<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let (Some(x_min), Some(x_max), Some(y_min), Some(y_max)) =
            (x_min(self.0), x_max(self.0), y_min(self.0), y_max(self.0))
        {
            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    f.write_char(if self.0.contains(&(x, y)) { '#' } else { '.' })?;
                }
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

impl<'a, I> Day<'a, I, Number> for Day23
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(input: I) -> Option<Number> {
        let mut elves = parse_elves(input)?;
        for round in 0..10 {
            spread_out(&mut elves, round);
        }
        Some(count_empty_tiles(&elves))
    }

    fn part2(input: I) -> Option<Number> {
        let mut elves = parse_elves(input)?;
        for round in 0.. {
            if !spread_out(&mut elves, round) {
                return Some(round + 1);
            }
        }
        unreachable!()
    }
}

fn parse_elves<'a, I>(input: I) -> Option<HashSet<Elf>>
where
    I: Iterator<Item = &'a str>,
{
    let mut elves = HashSet::new();
    for tile in input
        .enumerate()
        .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, c)| (x, y, c)))
    {
        match tile.2 {
            '#' => {
                elves.insert((tile.0 as Coord, tile.1 as Coord));
            }
            '.' => {}
            _ => {
                return None;
            }
        }
    }
    Some(elves)
}

fn spread_out(elves: &mut HashSet<Elf>, round: usize) -> bool {
    let mut propositions = HashMap::<Elf, Vec<_>>::new();
    for elf in elves.iter().filter(|elf| has_neighbor(elves, elf)) {
        for direction in (0..DIRECTIONS.len())
            .map(|d| (d + round) % DIRECTIONS.len())
            .map(|d| &DIRECTIONS[d])
        {
            let new_position = new_position(elves, elf, direction);
            if &new_position != elf {
                propositions.entry(new_position).or_default().push(*elf);
                break;
            }
        }
    }
    let mut positions_changed = false;
    for (proposition, candidates) in propositions {
        if candidates.len() == 1 {
            elves.remove(&candidates[0]);
            elves.insert(proposition);
            positions_changed = true;
        }
    }
    positions_changed
}

fn has_neighbor(elves: &HashSet<Elf>, elf: &Elf) -> bool {
    (elf.0 - 1..=elf.0 + 1)
        .flat_map(|x| (elf.1 - 1..=elf.1 + 1).map(move |y| (x, y)))
        .filter(|neighbor| neighbor != elf)
        .any(|neighbor| elves.contains(&neighbor))
}

fn new_position(elves: &HashSet<Elf>, elf: &Elf, direction: &Direction) -> Elf {
    if !match direction {
        // "have" to collect, or else:
        // error[E0308]: `match` arms have incompatible types
        //    = note: no two closures, even if identical, have the same type
        //    = help: consider boxing your closure and/or using it as a trait object
        Direction::North => (elf.0 - 1..=elf.0 + 1)
            .map(|x| (x, elf.1 - 1))
            .collect::<Vec<_>>(),
        Direction::South => (elf.0 - 1..=elf.0 + 1)
            .map(|x| (x, elf.1 + 1))
            .collect::<Vec<_>>(),
        Direction::West => (elf.1 - 1..=elf.1 + 1)
            .map(|y| (elf.0 - 1, y))
            .collect::<Vec<_>>(),
        Direction::East => (elf.1 - 1..=elf.1 + 1)
            .map(|y| (elf.0 + 1, y))
            .collect::<Vec<_>>(),
    }
    .iter()
    .any(|neighbor| elves.contains(neighbor))
    {
        match direction {
            Direction::North => (elf.0, elf.1 - 1),
            Direction::South => (elf.0, elf.1 + 1),
            Direction::West => (elf.0 - 1, elf.1),
            Direction::East => (elf.0 + 1, elf.1),
        }
    } else {
        *elf
    }
}

fn count_empty_tiles(elves: &HashSet<Elf>) -> usize {
    if let (Some(x_min), Some(x_max), Some(y_min), Some(y_max)) =
        (x_min(elves), x_max(elves), y_min(elves), y_max(elves))
    {
        ((x_max - x_min + 1) * (y_max - y_min + 1)) as usize - elves.len()
    } else {
        0
    }
}

fn x_min(elves: &HashSet<Elf>) -> Option<Coord> {
    elves.iter().map(|elf| elf.0).min()
}

fn x_max(elves: &HashSet<Elf>) -> Option<Coord> {
    elves.iter().map(|elf| elf.0).max()
}

fn y_min(elves: &HashSet<Elf>) -> Option<Coord> {
    elves.iter().map(|elf| elf.1).min()
}

fn y_max(elves: &HashSet<Elf>) -> Option<Coord> {
    elves.iter().map(|elf| elf.1).max()
}

#[cfg(test)]
mod tests {
    use super::{count_empty_tiles, has_neighbor, parse_elves, spread_out, Day, Day23, Number};
    use std::collections::HashSet;
    use std::str::Lines;

    const INPUT: &str = include_str!("../../23_test.txt");
    const INPUTS: [&str; 7] = [
        include_str!("../../23_test.00.txt"),
        include_str!("../../23_test.01.txt"),
        include_str!("../../23_test.02.txt"),
        include_str!("../../23_test.03.txt"),
        include_str!("../../23_test.04.txt"),
        include_str!("../../23_test.05.txt"),
        include_str!("../../23_test.10.txt"),
    ];

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<Number> {
        Day23::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<Number> {
        Day23::part2(test_input(input))
    }

    #[test]
    fn test_parse_elves() {
        assert_eq!(
            Some(HashSet::from([
                (4, 0),
                (2, 1),
                (3, 1),
                (4, 1),
                (6, 1),
                (0, 2),
                (4, 2),
                (6, 2),
                (1, 3),
                (5, 3),
                (6, 3),
                (0, 4),
                (2, 4),
                (3, 4),
                (4, 4),
                (0, 5),
                (1, 5),
                (3, 5),
                (5, 5),
                (6, 5),
                (1, 6),
                (4, 6),
            ])),
            parse_elves(test_input(INPUT))
        );
    }

    #[test]
    fn test_spread_out() {
        let mut elves = parse_elves(test_input(INPUTS[0])).unwrap();
        spread_out(&mut elves, 0);
        assert_eq!(parse_elves(test_input(INPUTS[1])).unwrap(), elves);
        spread_out(&mut elves, 1);
        assert_eq!(parse_elves(test_input(INPUTS[2])).unwrap(), elves);
        spread_out(&mut elves, 2);
        assert_eq!(parse_elves(test_input(INPUTS[3])).unwrap(), elves);
        spread_out(&mut elves, 3);
        assert_eq!(parse_elves(test_input(INPUTS[4])).unwrap(), elves);
        spread_out(&mut elves, 4);
        assert_eq!(parse_elves(test_input(INPUTS[5])).unwrap(), elves);
        spread_out(&mut elves, 5);
        spread_out(&mut elves, 6);
        spread_out(&mut elves, 7);
        spread_out(&mut elves, 8);
        spread_out(&mut elves, 9);
        assert_eq!(parse_elves(test_input(INPUTS[6])).unwrap(), elves);
    }

    #[test]
    fn test_has_neighbor() {
        assert_eq!(false, has_neighbor(&HashSet::from([(0, 0)]), &(0, 0)));
        assert_eq!(
            false,
            has_neighbor(&HashSet::from([(0, 0), (-2, -1)]), &(0, 0))
        );
        assert_eq!(
            false,
            has_neighbor(&HashSet::from([(0, 0), (-2, 0)]), &(0, 0))
        );
        assert_eq!(
            false,
            has_neighbor(&HashSet::from([(0, 0), (-2, 1)]), &(0, 0))
        );
        assert_eq!(
            false,
            has_neighbor(&HashSet::from([(0, 0), (2, -1)]), &(0, 0))
        );
        assert_eq!(
            false,
            has_neighbor(&HashSet::from([(0, 0), (2, 0)]), &(0, 0))
        );
        assert_eq!(
            false,
            has_neighbor(&HashSet::from([(0, 0), (2, 1)]), &(0, 0))
        );
        assert_eq!(
            false,
            has_neighbor(&HashSet::from([(0, 0), (0, -2)]), &(0, 0))
        );
        assert_eq!(
            false,
            has_neighbor(&HashSet::from([(0, 0), (0, 2)]), &(0, 0))
        );

        assert_eq!(
            true,
            has_neighbor(&HashSet::from([(0, 0), (0, 1)]), &(0, 0))
        );
        assert_eq!(
            true,
            has_neighbor(&HashSet::from([(0, 0), (-1, 1)]), &(0, 0))
        );
        assert_eq!(
            true,
            has_neighbor(&HashSet::from([(0, 0), (-1, 0)]), &(0, 0))
        );
        assert_eq!(
            true,
            has_neighbor(&HashSet::from([(0, 0), (-1, -1)]), &(0, 0))
        );
        assert_eq!(
            true,
            has_neighbor(&HashSet::from([(0, 0), (0, -1)]), &(0, 0))
        );
        assert_eq!(
            true,
            has_neighbor(&HashSet::from([(0, 0), (1, -1)]), &(0, 0))
        );
        assert_eq!(
            true,
            has_neighbor(&HashSet::from([(0, 0), (1, 0)]), &(0, 0))
        );
        assert_eq!(
            true,
            has_neighbor(&HashSet::from([(0, 0), (1, 1)]), &(0, 0))
        );
    }

    #[test]
    fn test_count_empty_tiles() {
        assert_eq!(
            27,
            count_empty_tiles(&parse_elves(test_input(INPUT)).expect("Unparsable input!"))
        );
    }

    #[test]
    fn test1() {
        assert_eq!(Some(110), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(Some(20), test_part2(INPUT));
    }
}
