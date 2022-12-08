use super::Day;

pub struct Day07;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, u64> for Day07 {
    fn part1(_input: I) -> Option<u64> {
        todo!()
    }

    fn part2(_input: I) -> Option<u64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::str::Lines;

    use super::{Day, Day07};

    const INPUT: &str = include_str!("../../07_test.txt");

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<u64> {
        Day07::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<u64> {
        Day07::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(95437), test_part1(INPUT)); // todo replace expected value
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
