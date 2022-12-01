use std::cmp::max;
use std::io::Error;
use std::num::ParseIntError;

pub fn main() {
    match read_input() {
        Ok(text) => {
            solve(part1, &text);
            solve(part2, &text);
        }
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn solve(part: fn(&str) -> Result<i32, ParseIntError>, text: &str) {
    match part(text) {
        Ok(most_calories) => println!("{}", most_calories),
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn part1(input: &str) -> Result<i32, ParseIntError> {
    let mut most_calories = 0;
    let mut current_calories = 0;
    for line in input.lines() {
        if line.is_empty() {
            most_calories = max(most_calories, current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<i32>()?;
        }
    }
    Ok(most_calories)
}

fn part2(input: &str) -> Result<i32, ParseIntError> {
    todo!()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../01.txt")
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use std::num::ParseIntError;

    fn test_input(input: &str) -> String {
        input.split(",").collect::<Vec<_>>().join("\n")
    }

    fn test_part1(input: &str) -> Result<i32, ParseIntError> {
        part1(&test_input(input))
    }

    fn test_part2(input: &str) -> Result<i32, ParseIntError> {
        part2(&test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(
            Ok(24000),
            test_part1("1000,2000,3000,,4000,,5000,6000,,7000,8000,9000,,10000")
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Ok(45000),
            test_part2("1000,2000,3000,,4000,,5000,6000,,7000,8000,9000,,10000")
        );
    }
}
