use std::collections::hash_map::RandomState;
use std::collections::HashSet;

use super::Day;

pub struct Day06;

const LEN1: usize = 4;
const LEN2: usize = 14;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, i32> for Day06 {
    fn part1(mut input: I) -> Option<i32> {
        for (i, window) in input.next()?.as_bytes().windows(LEN1).enumerate() {
            let array: [u8; LEN1] = window.try_into().ok()?;
            let set = HashSet::from(array);
            if set.len() == LEN1 {
                return (i + LEN1).try_into().ok();
            }
        }
        None
    }

    fn part2(mut input: I) -> Option<i32> {
        for (i, window) in input.next()?.as_bytes().windows(LEN2).enumerate() {
            let set = HashSet::<_, RandomState>::from_iter(window.iter());
            if set.len() == LEN2 {
                return (i + LEN2).try_into().ok();
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::str::Lines;

    use super::{Day, Day06};

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
        assert_eq!(Some(19), test_part2(INPUT[0]));
    }

    #[test]
    fn test2b() {
        assert_eq!(Some(23), test_part2(INPUT[1]));
    }

    #[test]
    fn test2c() {
        assert_eq!(Some(23), test_part2(INPUT[2]));
    }

    #[test]
    fn test2d() {
        assert_eq!(Some(29), test_part2(INPUT[3]));
    }

    #[test]
    fn test2e() {
        assert_eq!(Some(26), test_part2(INPUT[4]));
    }
}
