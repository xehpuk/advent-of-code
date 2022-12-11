use super::Day;

pub struct Day09;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, usize> for Day09 {
    fn part1(_input: I) -> Option<usize> {
        todo!()
    }

    fn part2(_input: I) -> Option<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day09};
    use std::str::Split;

    const INPUT: &str = "R 4,U 4,L 3,D 1,R 4,D 1,L 5,R 2";

    fn test_input(input: &str) -> Split<char> {
        input.split(',')
    }

    fn test_part1(input: &str) -> Option<usize> {
        Day09::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<usize> {
        Day09::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(13), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
