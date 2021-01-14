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
        Ok(text) => {
            match solve_part1(&text) {
                Ok((m, n)) => println!("{} * {} = {}", m, n, m * n),
                Err(err) => eprintln!("{:#?}", err),
            }
            match solve_part2(&text) {
                Ok((m, n, o)) => println!("{} * {} * {} = {}", m, n, o, m * n * o),
                Err(err) => eprintln!("{:#?}", err),
            }
        }
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn solve_part1(input: &str) -> Result<(u32, u32), SolveErr> {
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

fn solve_part2(input: &str) -> Result<(u32, u32, u32), SolveErr> {
    let mut vec = input
        .lines()
        .map(|line| line.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?;
    vec.sort();

    // https://en.wikipedia.org/wiki/3SUM#Quadratic_algorithm
    let n = vec.len();
    for i in 0..n - 2 {
        let a = vec[i];
        let mut start = i + 1;
        let mut end = n - 1;
        while start < end {
            let b = vec[start];
            let c = vec[end];
            let sum = a + b + c;
            if sum == SUM {
                return Ok((a, b, c));
                // If we wanted to find all combinations:
                // start = start + 1;
                // end = end - 1;
            }
            if sum > SUM {
                end -= 1;
            } else {
                start += 1;
            }
        }
    }

    Err(SolveErr::Unsolvable)
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("01.txt")
}
