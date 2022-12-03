use super::Day;

pub struct Day03;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I> for Day03 {
    fn part1(input: I) -> Option<i32> {
        todo!()
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
