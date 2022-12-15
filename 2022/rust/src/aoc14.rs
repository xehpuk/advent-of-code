use super::Day;

pub struct Day14;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, u32> for Day14 {
    fn part1(_input: I) -> Option<u32> {
        todo!()
    }

    fn part2(_input: I) -> Option<u32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day14};
    use std::str::Lines;

    const INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<u32> {
        Day14::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<u32> {
        Day14::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(24), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
