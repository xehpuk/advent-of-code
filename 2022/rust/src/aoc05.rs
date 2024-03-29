use super::Day;
use std::borrow::BorrowMut;
use std::collections::VecDeque;

pub struct Day05;

enum Crane {
    CargoMover9000,
    CargoMover9001,
}

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, String> for Day05 {
    fn part1(input: I) -> Option<String> {
        solve(input, Crane::CargoMover9000)
    }

    fn part2(input: I) -> Option<String> {
        solve(input, Crane::CargoMover9001)
    }
}

fn solve<'a, I: Clone + Iterator<Item = &'a str>>(mut input: I, crane: Crane) -> Option<String> {
    let mut stacks = Vec::new();
    for row in input
        .borrow_mut()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().skip(1).step_by(4).enumerate())
    {
        for (stack, _crate) in row {
            if _crate.is_alphabetic() {
                while stacks.len() <= stack {
                    stacks.push(VecDeque::new());
                }
                stacks.get_mut(stack)?.push_back(_crate);
            }
        }
    }
    for mut instruction in input.map(str::chars) {
        let mut parse = |skip: &str| {
            instruction
                .borrow_mut()
                .skip(skip.chars().count())
                .take_while(char::is_ascii_digit)
                .collect::<String>()
                .parse::<usize>()
                .ok()
        };

        let count = parse("move ")?;
        let from = parse("from ")?.checked_sub(1)?;
        let to = parse("to ")?.checked_sub(1)?;

        match crane {
            Crane::CargoMover9000 => {
                for _ in 0..count {
                    let _crate = stacks.get_mut(from)?.pop_front()?;
                    stacks.get_mut(to)?.push_front(_crate);
                }
            }
            Crane::CargoMover9001 => {
                let source = stacks.get_mut(from)?;
                let mut split = source.drain(0..count).collect::<VecDeque<_>>();
                let destination = stacks.get_mut(to)?;
                split.append(destination);
                *destination = split;
            }
        }
    }
    Some(
        stacks
            .iter()
            .map(|stack| stack.front().unwrap_or(&' '))
            .collect::<String>(),
    )
}

#[cfg(test)]
mod tests {
    use super::{Day, Day05};
    use std::str::Lines;

    const INPUT: &str = include_str!("../../05_test.txt");

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
        assert_eq!(Some("CMZ".into()), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(Some("MCD".into()), test_part2(INPUT));
    }
}
