use super::Day;

use std::cmp::max;

pub struct Day08;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, usize> for Day08 {
    fn part1(input: I) -> Option<usize> {
        let forest = parse_forest(input)?;

        Some(
            forest
                .iter()
                .enumerate()
                .map(|(row, columns)| {
                    columns
                        .iter()
                        .enumerate()
                        .filter(|&(column, &tree)| {
                            !(columns
                                .iter()
                                .take(column)
                                .any(|&neighbor| neighbor >= tree)
                                && columns
                                    .iter()
                                    .skip(column + 1)
                                    .any(|&neighbor| neighbor >= tree)
                                && (0..row)
                                    .map(|row| &forest[row])
                                    .any(|row| row[column] >= tree)
                                && (row + 1..forest.len())
                                    .map(|row| &forest[row])
                                    .any(|row| row[column] >= tree))
                        })
                        .count()
                })
                .sum(),
        )
    }

    fn part2(input: I) -> Option<usize> {
        let forest = parse_forest(input)?;

        let mut highest_scenic_score = 0;
        for (row, columns) in forest.iter().enumerate() {
            for (column, &tree) in columns.iter().enumerate() {
                let mut up = 0;
                for neighbor in (0..row)
                    .rev()
                    .map(|row| &forest[row])
                    .map(|row| row[column])
                {
                    up += 1;
                    if neighbor >= tree {
                        break;
                    }
                }
                let mut left = 0;
                for neighbor in (0..column).rev().map(|column| columns[column]) {
                    left += 1;
                    if neighbor >= tree {
                        break;
                    }
                }
                let mut down = 0;
                for neighbor in (row + 1..forest.len())
                    .map(|row| &forest[row])
                    .map(|row| row[column])
                {
                    down += 1;
                    if neighbor >= tree {
                        break;
                    }
                }
                let mut right = 0;
                for &neighbor in columns.iter().skip(column + 1) {
                    right += 1;
                    if neighbor >= tree {
                        break;
                    }
                }
                highest_scenic_score = max(highest_scenic_score, up * left * down * right);
            }
        }
        Some(highest_scenic_score)
    }
}

fn parse_forest<'a, I: Iterator<Item = &'a str>>(input: I) -> Option<Vec<Vec<u8>>> {
    input
        .map(|line| {
            line.chars()
                .map(|tree| tree.to_digit(10).map(|tree| tree as u8))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Day, Day08};
    use std::str::Lines;

    const INPUT: &str = "\
30373
25512
65332
33549
35390";

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<usize> {
        Day08::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<usize> {
        Day08::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(21), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(Some(8), test_part2(INPUT));
    }
}
