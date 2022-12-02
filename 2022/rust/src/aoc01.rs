use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::Error;

struct Calories<'a, I: Iterator<Item = &'a str>> {
    str_iter: I,
    current: Option<i32>,
}

impl<'a, I: Iterator<Item = &'a str>> Calories<'a, I> {
    pub fn new(str_iter: I) -> Calories<'a, I> {
        Calories {
            str_iter,
            current: None,
        }
    }
}

impl<'a, I: Iterator<Item = &'a str>> Iterator for Calories<'a, I> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.str_iter.next() {
            if line.is_empty() {
                if let Some(v) = self.current {
                    self.current = None;
                    return Some(v);
                }
            } else {
                let calorie: i32 = line.parse().ok()?;
                self.current = Some(self.current.map_or(calorie, |v| v + calorie));
            }
            self.next()
        } else {
            let next = self.current;
            self.current = None;
            next
        }
    }
}

pub fn main() {
    match read_input() {
        Ok(text) => {
            solve(part1, &text);
            solve(part2, &text);
        }
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn solve(part: fn(&str) -> Option<i32>, text: &str) {
    match part(text) {
        Some(most_calories) => println!("{}", most_calories),
        None => eprintln!("No result!"),
    }
}

fn part1(input: &str) -> Option<i32> {
    Calories::new(input.lines()).max()
}

fn part2(input: &str) -> Option<i32> {
    let mut most_calories = BinaryHeap::new();

    for current_calories in Calories::new(input.lines()) {
        if most_calories.len() < 3 {
            most_calories.push(Reverse(current_calories));
        } else {
            let mut least_calories = most_calories.peek_mut().unwrap();
            if current_calories > least_calories.0 {
                *least_calories = Reverse(current_calories);
            }
        }
    }

    Some(most_calories.iter().map(|calories| calories.0).sum())
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../01.txt")
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "1000,2000,3000,,4000,,5000,6000,,7000,8000,9000,,10000";

    fn test_input(input: &str) -> String {
        input.split(',').collect::<Vec<_>>().join("\n")
    }

    fn test_part1(input: &str) -> Option<i32> {
        part1(&test_input(input))
    }

    fn test_part2(input: &str) -> Option<i32> {
        part2(&test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(24000), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(Some(45000), test_part2(INPUT));
    }
}