use std::collections::BTreeSet;
use std::io::Error;

pub fn main() {
    match read_input().map(|input| parse_input(&input)) {
        Ok(Some(instructions)) => match solve_part1(&instructions) {
            Some(order) => println!("Correct order: {}", order),
            None => eprintln!("Couldn't find order."),
        },
        err => eprintln!("{:#?}", err),
    }
}

fn solve_part1(instructions: &[(char, char)]) -> Option<String> {
    let mut steps_left = instructions
        .iter()
        .flat_map(|&(left, right)| vec![left, right])
        .collect::<BTreeSet<_>>();
    let mut order = String::with_capacity(steps_left.len());
    while !steps_left.is_empty() {
        let mut found = None;
        for &step in steps_left.iter() {
            if instructions
                .iter()
                .find(|(left, right)| *right == step && steps_left.contains(left))
                .is_none()
            {
                order.push(step);
                found = Some(step);
                break;
            }
        }
        match found {
            Some(step) => {
                steps_left.remove(&step);
            }
            None => return None,
        }
    }
    Some(order)
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
    std::fs::read_to_string("../07.txt")
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
        parse_input(INPUT).unwrap()
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

        assert_eq!("CABDFE", solve_part1(&instructions).unwrap());
    }
}
