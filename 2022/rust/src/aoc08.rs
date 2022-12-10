use super::Day;

pub struct Day08;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, i32> for Day08 {
    fn part1(_input: I) -> Option<i32> {
        todo!()
    }

    fn part2(_input: I) -> Option<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day08};
    use std::str::Lines;

    const INPUT: &str = "\
30373
25512
65332
33549
35390";

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<i32> {
        Day08::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<i32> {
        Day08::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(21), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
