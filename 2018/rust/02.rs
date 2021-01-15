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
            match solve_part2(&text) {
                Some((i, common)) => println!("{}: {}", i, common),
                None => eprintln!("{}", "Correct box IDs not found!"),
            }
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

fn solve_part2(input: &str) -> Option<(usize, String)> {
    let box_ids: Vec<_> = input.lines().collect();
    for (i, box_id) in box_ids.iter().enumerate() {
        let box_id_chars: Vec<char> = box_id.chars().collect();
        'id2: for box_id2 in &box_ids[i + 1..] {
            let mut diff_index = None;
            for (j, (&box_id_char, box_id2_char)) in
                box_id_chars.iter().zip(box_id2.chars()).enumerate()
            {
                if box_id_char != box_id2_char {
                    if diff_index.is_some() {
                        continue 'id2;
                    }
                    diff_index = Some(j);
                }
            }
            if let Some(diff_index) = diff_index {
                return Some((
                    diff_index,
                    box_id_chars
                        .iter()
                        .enumerate()
                        .filter(|&(i, _)| i != diff_index)
                        .map(|(_, c)| c)
                        .collect(),
                ));
            }
        }
    }

    None
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../02.txt")
}
