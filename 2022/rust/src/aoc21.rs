use std::collections::HashMap;

use super::Day;

pub struct Day21;

type Number = i32;

impl<'a, I> Day<'a, I, Number> for Day21
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(_input: I) -> Option<Number> {
        todo!()
    }

    fn part2(_input: I) -> Option<Number> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day21, Number};
    use std::str::Lines;

    const INPUT: &str = include_str!("../../21_test.txt");

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<Number> {
        Day21::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<Number> {
        Day21::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(152), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
