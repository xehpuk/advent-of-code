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

fn react(a: u8, b: u8) -> bool {
    a != b && a.eq_ignore_ascii_case(&b)
}

fn solve_part1(polymer: &str) -> String {
    let mut reduced_polymer = Vec::with_capacity(polymer.len());
    for b in polymer.bytes() {
        if reduced_polymer.last().map_or(true, |&a| !react(a, b)) {
            reduced_polymer.push(b);
        } else {
            reduced_polymer.pop();
        }
    }
    String::from_utf8(reduced_polymer).unwrap()
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
