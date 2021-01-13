use std::collections::HashSet;
use std::io::Error;
use std::num::ParseIntError;

const SUM: u32 = 2020;

#[derive(Debug)]
enum SolveErr {
    Parse(ParseIntError),
    Unsolvable,
}

impl From<ParseIntError> for SolveErr {
    fn from(err: ParseIntError) -> Self {
        Self::Parse(err)
    }
}

fn main() {
    match read_input() {
        Ok(text) => match solve(&text) {
            Ok((m, n)) => println!("{} * {} = {}", m, n, m * n),
            Err(err) => eprintln!("{:#?}", err),
        },
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn solve(input: &str) -> Result<(u32, u32), SolveErr> {
    let set = input
        .lines()
        .map(|line| line.parse::<u32>())
        .collect::<Result<HashSet<u32>, _>>()?;

    for &expense in &set {
        let candidate = SUM - expense;
        if set.contains(&candidate) {
            return Ok((expense, candidate));
        }
    }

    Err(SolveErr::Unsolvable)
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("01.txt")
}
