use super::Day;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::str::Chars;

pub struct Day03;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I> for Day03 {
    fn part1(input: I) -> Option<i32> {
        input
            .map(str::chars)
            .map(Chars::collect::<Vec<_>>)
            .map(|chars| {
                let (first, second) = chars.split_at(chars.len() / 2);
                let first: HashSet<_, RandomState> = HashSet::from_iter(first.iter());
                let second: HashSet<_, RandomState> = HashSet::from_iter(second.iter());
                let mut intersection = first.intersection(&second);
                let x = *intersection.next()?;
                if intersection.next().is_some() {
                    return None;
                }
                Some(x.to_owned())
            })
            .map(|c| {
                c.and_then(|c| match c {
                    'a'..='z' => Some(1 + c as u32 - 'a' as u32),
                    'A'..='Z' => Some(27 + c as u32 - 'A' as u32),
                    _ => None,
                })
            })
            .sum::<Option<u32>>()?
            .try_into()
            .ok()
    }

    fn part2(input: I) -> Option<i32> {
        todo!()
    }
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

    #[test]
    fn test1() {
        assert_eq!(Some(157), test_part1(INPUT));
    }
}
