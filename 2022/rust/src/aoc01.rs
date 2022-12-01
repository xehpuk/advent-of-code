use std::io::Error;
use std::num::ParseIntError;

pub fn main() {
    match read_input() {
        Ok(text) => {
            solve(part1, &text);
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
    todo!()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../01.txt")
}

#[cfg(test)]
mod tests {
    use super::part1;
    use std::num::ParseIntError;

    fn test_input(input: &str) -> String {
        input.split(",").collect::<Vec<_>>().join("\n")
    }

    fn test_part1(input: &str) -> Result<i32, ParseIntError> {
        part1(&test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(
            Ok(24000),
            test_part1("1000,2000,3000,,4000,,5000,6000,,7000,8000,9000,,10000")
        );
    }
}
