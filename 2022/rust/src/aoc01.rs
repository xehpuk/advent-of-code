use super::Day;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Day01;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I> for Day01 {
    fn part1(input: I) -> Option<i32> {
        Calories::new(input).max()
    }

    fn part2(input: I) -> Option<i32> {
        let mut most_calories = BinaryHeap::new();

        for current_calories in Calories::new(input) {
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
}

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

#[cfg(test)]
mod tests {
    use super::{Day, Day01};
    use std::str::Split;

    const INPUT: &str = "1000,2000,3000,,4000,,5000,6000,,7000,8000,9000,,10000";

    fn test_input(input: &str) -> Split<char> {
        input.split(',')
    }

    fn test_part1(input: &str) -> Option<i32> {
        Day01::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<i32> {
        Day01::part2(test_input(input))
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
