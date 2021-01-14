use std::collections::HashMap;
use std::io::Error;

struct Contains {
    twice: bool,
    thrice: bool,
}

impl From<&str> for Contains {
    fn from(s: &str) -> Self {
        let mut chars: HashMap<char, u32> = HashMap::with_capacity(s.len());
        for c in s.chars() {
            *chars.entry(c).or_default() += 1;
        }
        Contains {
            twice: chars.values().any(|&count| count == 2),
            thrice: chars.values().any(|&count| count == 3),
        }
    }
}

fn main() {
    match read_input() {
        Ok(text) => {
            solve_part1(&text);
            // solve_part2(&text);
        }
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn solve_part1(input: &str) {
    let contains = input
        .lines()
        .map(|line| line.into())
        .collect::<Vec<Contains>>();
    let twice = contains.iter().filter(|c| c.twice).count();
    let thrice = contains.iter().filter(|c| c.thrice).count();

    println!("{} * {} = {}", twice, thrice, twice * thrice)
}

fn solve_part2(input: &str) {
    todo!()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../02.txt")
}
