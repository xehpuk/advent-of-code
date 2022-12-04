use super::Day;

pub struct Day04;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I> for Day04 {
    fn part1(input: I) -> Option<i32> {
        todo!()
    }

    fn part2(input: I) -> Option<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day04};
    use std::str::Lines;

    const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<i32> {
        Day04::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<i32> {
        Day04::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(2), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT));
    }
}
