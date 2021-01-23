use std::io::Error;
use std::str::from_utf8;

pub fn main() {
    match read_input() {
        Ok(polymer) => {
            let reduced_polymer = solve_part1(&polymer);
            println!("Reduced polymer: {}", reduced_polymer);
            println!("Length: {}", reduced_polymer.len());
            let reduced_polymer = solve_part2(&polymer);
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

fn solve_part2(polymer: &str) -> String {
    (b'a'..=b'z')
        .fold(None, |reduced_polymer: Option<String>, c| {
            let current_reduced_polymer = solve_part1(
                from_utf8(
                    &polymer
                        .bytes()
                        .filter(|a| !c.eq_ignore_ascii_case(a))
                        .collect::<Vec<u8>>(),
                )
                .unwrap(),
            );

            Some(match reduced_polymer {
                Some(reduced_polymer) => {
                    if reduced_polymer.len() < current_reduced_polymer.len() {
                        reduced_polymer
                    } else {
                        current_reduced_polymer
                    }
                }
                None => current_reduced_polymer,
            })
        })
        .unwrap()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../05.txt")
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn test1a() {
        assert_eq!("dabCBAcaDA", solve_part1("dabAcCaCBAcCcaDA"));
    }

    #[test]
    fn test1b() {
        assert_eq!("dbCBcD", solve_part1("dbcCCBcCcD"));
    }

    #[test]
    fn test1c() {
        assert_eq!("daCAcaDA", solve_part1("daAcCaCAcCcaDA"));
    }

    #[test]
    fn test1d() {
        assert_eq!("daDA", solve_part1("dabAaBAaDA"));
    }

    #[test]
    fn test1e() {
        assert_eq!("abCBAc", solve_part1("abAcCaCBAcCcaA"));
    }

    #[test]
    fn test2() {
        assert_eq!(4, solve_part2("dabAcCaCBAcCcaDA").len());
    }
}
