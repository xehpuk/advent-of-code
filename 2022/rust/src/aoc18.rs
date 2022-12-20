use super::Day;

pub struct Day18;

impl<'a, I> Day<'a, I, usize> for Day18
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(_input: I) -> Option<usize> {
        todo!()
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
