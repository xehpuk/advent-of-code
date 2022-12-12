use super::Day;

pub struct Day11;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, i32> for Day11 {
    fn part1(_input: I) -> Option<i32> {
        todo!()
    }

    fn part2(_input: I) -> Option<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day11};
    use std::str::Lines;

    const INPUT: &str = include_str!("../../11_test.txt");

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<i32> {
        Day11::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<i32> {
        Day11::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(10605), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
