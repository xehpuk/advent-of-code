use super::Day;

pub struct Day05;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, String> for Day05 {
    fn part1(_input: I) -> Option<String> {
        todo!()
    }

    fn part2(_input: I) -> Option<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day05};
    use std::str::Lines;

    const INPUT: &str = "\
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<String> {
        Day05::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<String> {
        Day05::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some("CMZ".into()), test_part1(INPUT)); // todo replace expected value
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
