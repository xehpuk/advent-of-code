use super::Day;

pub struct Day06;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, i32> for Day06 {
    fn part1(_input: I) -> Option<i32> {
        // regex: (.)((?!\1).)((?!\1|\2).)((?!\1|\2|\3).)
        todo!()
    }

    fn part2(_input: I) -> Option<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day06};
    use std::str::Lines;

    const INPUT: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<i32> {
        Day06::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<i32> {
        Day06::part2(test_input(input))
    }

    #[test]
    fn test1a() {
        assert_eq!(Some(7), test_part1(INPUT[0]));
    }

    #[test]
    fn test1b() {
        assert_eq!(Some(5), test_part1(INPUT[1]));
    }

    #[test]
    fn test1c() {
        assert_eq!(Some(6), test_part1(INPUT[2]));
    }

    #[test]
    fn test1d() {
        assert_eq!(Some(10), test_part1(INPUT[3]));
    }

    #[test]
    fn test1e() {
        assert_eq!(Some(11), test_part1(INPUT[4]));
    }

    #[test]
    fn test2a() {
        assert_eq!(None, test_part2(INPUT[0])); // todo replace expected value
    }

    #[test]
    fn test2b() {
        assert_eq!(None, test_part2(INPUT[1])); // todo replace expected value
    }

    #[test]
    fn test2c() {
        assert_eq!(None, test_part2(INPUT[2])); // todo replace expected value
    }

    #[test]
    fn test2d() {
        assert_eq!(None, test_part2(INPUT[3])); // todo replace expected value
    }

    #[test]
    fn test2e() {
        assert_eq!(None, test_part2(INPUT[4])); // todo replace expected value
    }
}
