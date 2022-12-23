use super::Day;

pub struct Day20;

impl<'a, I> Day<'a, I, i32> for Day20
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(_input: I) -> Option<i32> {
        todo!()
    }

    fn part2(_input: I) -> Option<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day20};
    use std::str::Split;

    const INPUT: &str = "1,2,-3,3,-2,0,4";

    fn test_input(input: &str) -> Split<char> {
        input.split(',')
    }

    fn test_part1(input: &str) -> Option<i32> {
        Day20::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<i32> {
        Day20::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(3), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
