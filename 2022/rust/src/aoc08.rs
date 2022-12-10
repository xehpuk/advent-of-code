use super::Day;

pub struct Day08;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, usize> for Day08 {
    fn part1(input: I) -> Option<usize> {
        let forest = input
            .map(|line| {
                line.chars()
                    .map(|tree| tree.to_digit(10).map(|tree| tree as u8))
                    .collect()
            })
            .collect::<Option<Vec<Vec<_>>>>()?;

        let mut trees = 0;
        for (row, columns) in forest.iter().enumerate() {
            trees += columns
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
                .count();
        }

        Some(trees)
    }

    fn part2(_input: I) -> Option<usize> {
        todo!()
    }
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
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
