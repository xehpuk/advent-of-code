use super::Day;

use std::collections::HashMap;
use std::ops::ControlFlow;
use ControlFlow::{Break, Continue};
use Element::{Directory, File};

pub struct Day07;

#[derive(Debug)]
enum Element {
    File { size: u64 },
    Directory { children: HashMap<String, Element> },
}

impl Element {
    fn navigate(&mut self, path: &[String]) -> Option<&mut Self> {
        match path.iter().try_fold(self, |destination, path| {
            if let Directory { children } = destination {
                if let Some(next) = children.get_mut(path) {
                    return Continue(next);
                }
            }
            Break(())
        }) {
            Continue(destination) => Some(destination),
            Break(_) => None,
        }
    }

    fn size(&self) -> u64 {
        match self {
            Directory { children } => children.values().map(Element::size).sum(),
            File { size } => *size,
        }
    }

    fn sizes(&self) -> Vec<u64> {
        fn helper(cwd: &Element, sizes: &mut Vec<u64>) -> u64 {
            match cwd {
                Directory { children } => {
                    let size = children
                        .values()
                        .map(|element| helper(element, sizes))
                        .sum();
                    sizes.push(size);

                    size
                }
                File { size } => *size,
            }
        }
        let mut elements = vec![];
        helper(self, &mut elements);
        elements
    }
}

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, u64> for Day07 {
    fn part1(input: I) -> Option<u64> {
        let element = parse_terminal_output(input)?;
        println!("{:#?}", element);
        println!("{}", element.size());
        let sizes = element.sizes();
        println!("{:#?}", sizes);
        Some(sizes.iter().filter(|&&size| size <= 100_000).sum())
    }

    fn part2(input: I) -> Option<u64> {
        let sizes = parse_terminal_output(input)?.sizes();
        let max = sizes.iter().max()?;
        let available = 70_000_000 - max;
        let needed = 30_000_000u64.checked_sub(available)?;

        sizes.into_iter().filter(|&size| size >= needed).min()
    }
}

fn parse_terminal_output<'a, I: Iterator<Item = &'a str>>(input: I) -> Option<Element> {
    let mut root = Directory {
        children: HashMap::new(),
    };
    let mut path = vec![];
    for line in input {
        let mut parts = line.split(' ');
        let first = parts.next()?;
        let second = parts.next()?;
        match first {
            "$" => match second {
                "cd" => {
                    let destination = parts.next()?;
                    match destination {
                        "/" => {
                            path.clear();
                        }
                        ".." => {
                            path.pop();
                        }
                        &_ => {
                            path.push(destination.to_string());
                        }
                    }
                }
                "ls" => {}
                &_ => return None,
            },
            _ => {
                let mut cwd = root.navigate(&path)?;
                if let Directory { children } = &mut cwd {
                    children.insert(
                        second.into(),
                        match first {
                            "dir" => Directory {
                                children: HashMap::new(),
                            },
                            file_size => File {
                                size: file_size.parse().ok()?,
                            },
                        },
                    );
                }
            }
        }
    }

    Some(root)
}

#[cfg(test)]
mod tests {
    use crate::aoc07::parse_terminal_output;
    use std::str::Lines;

    use super::{Day, Day07};

    const INPUT: &str = include_str!("../../07_test.txt");

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<u64> {
        Day07::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<u64> {
        Day07::part2(test_input(input))
    }

    #[test]
    fn test_size() {
        assert_eq!(
            Some(48381165),
            parse_terminal_output(test_input(INPUT)).map(|element| element.size())
        );
    }

    #[test]
    fn test1() {
        assert_eq!(Some(95437), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(Some(24933642), test_part2(INPUT));
    }
}
