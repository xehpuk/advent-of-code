use anyhow::Result;

const INPUT: &str = include_str!("../../01.txt");

pub fn solve() {
    println!("day 01, part 1: {}", part1(INPUT).unwrap());
    println!("day 01, part 2: {}", part2(INPUT).unwrap());
}

fn part1(input: &str) -> Result<i32> {
    todo!()
}

fn part2(input: &str) -> Result<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT1: &str = include_str!("../../01_test1.txt");
    const INPUT2: &str = include_str!("../../01_test2.txt");

    #[test]
    fn test1() {
        assert_eq!(142, part1(INPUT1).unwrap());
    }

    #[test]
    fn test2() {
        assert_eq!(281, part2(INPUT2).unwrap());
    }
}
