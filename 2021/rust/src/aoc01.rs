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
        Ok(drift) => println!("{}", drift),
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn part1(input: &str) -> Result<i32, ParseIntError> {
    Ok(input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()?
        .windows(2)
        .fold(0, |increased, depth| {
            if depth[0] < depth[1] {
                increased + 1
            } else {
                increased
            }
        }))
}

fn part2(input: &str) -> Result<i32, ParseIntError> {
    Ok(input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()?
        .windows(3)
        .map(|depths| depths.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .fold(0, |increased, depth| {
            if depth[0] < depth[1] {
                increased + 1
            } else {
                increased
            }
        }))
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../01.txt")
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "\
199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test1() {
        assert_eq!(Ok(7), part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(Ok(5), part2(INPUT));
    }
}
