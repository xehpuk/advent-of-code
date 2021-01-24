use std::io::Error;

pub fn main() {
    match read_input().map(|input| parse_input(&input)) {
        Ok(Some(instructions)) => {
            println!("Correct order: {}", solve_part1(&instructions));
        }
        err => eprintln!("{:#?}", err),
    }
}

fn solve_part1(instructions: &[(char, char)]) -> String {
    todo!()
}

fn parse_input(input: &str) -> Option<Vec<(char, char)>> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            Some((chars.nth(5)?, chars.nth(30)?))
        })
        .collect()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../06.txt")
}

#[cfg(test)]
mod tests {
    use super::{parse_input, solve_part1};

    const INPUT: &str = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    fn create_test_instructions() -> Vec<(char, char)> {
        parse_input(&INPUT).unwrap()
    }

    #[test]
    fn test_parsing() {
        assert_eq!(
            vec![
                ('C', 'A'),
                ('C', 'F'),
                ('A', 'B'),
                ('A', 'D'),
                ('B', 'E'),
                ('D', 'E'),
                ('F', 'E'),
            ],
            create_test_instructions()
        );
    }

    #[test]
    fn test1() {
        let instructions = create_test_instructions();

        assert_eq!("CABDFE", solve_part1(&instructions));
    }
}
