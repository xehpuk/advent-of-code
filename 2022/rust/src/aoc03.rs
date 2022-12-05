use super::Day;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::str::Chars;

pub struct Day03;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, i32> for Day03 {
    fn part1(input: I) -> Option<i32> {
        input
            .map(str::chars)
            .map(Chars::collect::<Vec<_>>)
            .map(|chars| {
                let (first, second) = chars.split_at(chars.len() / 2);
                // todo: remove RandomState
                let first: HashSet<_, RandomState> = HashSet::from_iter(first.iter());
                let second: HashSet<_, RandomState> = HashSet::from_iter(second.iter());
                let mut intersection = first.intersection(&second);
                let x = *intersection.next()?;
                if intersection.next().is_some() {
                    // should only have one item
                    return None;
                }
                Some(x.to_owned())
            })
            .map(|c| c.and_then(get_priority))
            .sum()
    }

    fn part2(input: I) -> Option<i32> {
        let mut count = 0;
        let mut intersection = HashSet::new();
        let mut priorities = 0;

        for line in input {
            count += 1;
            let unique_chars = HashSet::from_iter(line.chars());
            if count == 1 {
                intersection = unique_chars;
            } else {
                intersection.retain(|c| unique_chars.contains(c));
                if count >= 3 {
                    let mut iter = intersection.iter();
                    let badge = *iter.next()?;
                    if iter.next().is_some() {
                        // should only have one item
                        return None;
                    }
                    priorities += get_priority(badge)?;
                    count = 0;
                }
            }
        }

        if count != 0 {
            // elves not divisible into groups of three
            return None;
        }

        Some(priorities)
    }
}

fn get_priority(c: char) -> Option<i32> {
    match c {
        'a'..='z' => Some(1 + c as u32 - 'a' as u32),
        'A'..='Z' => Some(27 + c as u32 - 'A' as u32),
        _ => None,
    }?
    .try_into()
    .ok()
}

#[cfg(test)]
mod tests {
    use super::{Day, Day03};
    use std::str::Lines;

    const INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<i32> {
        Day03::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<i32> {
        Day03::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(157), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(Some(70), test_part2(INPUT));
    }
}
