use super::Day;

pub struct Day25;

type Snafu = String;

impl<'a, I> Day<'a, I, Snafu> for Day25
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(_input: I) -> Option<Snafu> {
        todo!()
    }

    fn part2(_input: I) -> Option<Snafu> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day25, Snafu};
    use std::str::Lines;

    const INPUT: &str = include_str!("../../25_test.txt");

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<Snafu> {
        Day25::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<Snafu> {
        Day25::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some("2=-1=0".to_string()), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
