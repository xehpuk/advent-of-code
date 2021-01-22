use std::io::Error;

pub fn main() {
    match read_input() {
        Ok(polymer) => {
            let reduced_polymer = solve_part1(&polymer);
            println!("Reduced polymer: {}", reduced_polymer);
            println!("Length: {}", reduced_polymer.len());
        }
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn solve_part1(polymer: &str) -> String {
    polymer.into()
}

#[allow(dead_code, unused_variables)]
fn solve_part2(polymer: &str) -> String {
    todo!()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../05.txt")
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn test1() {
        assert_eq!("dabCBAcaDA", solve_part1("dabAcCaCBAcCcaDA"));
    }
}
