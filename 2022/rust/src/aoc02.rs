use std::io::Error;

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
        Some(result) => println!("{}", result),
        None => eprintln!("No result!"),
    }
}

fn part1(input: &str) -> Option<i32> {
    todo!()
}

fn part2(input: &str) -> Option<i32> {
    todo!()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../02.txt")
}

#[cfg(test)]
mod tests {
    use super::part1;

    const INPUT: &str = "A Y,B X,C Z";

    fn test_input(input: &str) -> String {
        input.split(',').collect::<Vec<_>>().join("\n")
    }

    fn test_part1(input: &str) -> Option<i32> {
        part1(&test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(15), test_part1(INPUT));
    }
}
