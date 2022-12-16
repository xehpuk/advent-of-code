use super::Day;

pub struct Day__;

impl<'a, I> Day<'a, I, T> for Day__
where
    I: Clone + Iterator<Item = &'a str>,
{
    // todo specify T
    fn part1(_input: I) -> Option<T> {
        todo!()
    }

    fn part2(_input: I) -> Option<T> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day__};
    use std::str::Lines;

    const INPUT: &str = ""; // todo add test input

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<T> {
        Day__::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<T> {
        Day__::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(None, test_part1(INPUT)); // todo replace expected value
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
