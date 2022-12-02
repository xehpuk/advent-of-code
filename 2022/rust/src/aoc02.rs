use std::io::Error;

enum Item {
    Rock,
    Paper,
    Scissors,
}

impl Item {
    fn parse(input: &str) -> Option<Self> {
        match input {
            "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissors),
            &_ => None,
        }
    }
}

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
    // todo: do not collect
    let items = input
        .lines()
        .map(parse_line)
        .collect::<Option<Vec<(Item, Item)>>>()?;

    Some(items.into_iter().map(calc_score).sum())
}

fn part2(input: &str) -> Option<i32> {
    todo!()
}

fn calc_score(round: (Item, Item)) -> i32 {
    return match round.1 {
        Item::Rock => 1,
        Item::Paper => 2,
        Item::Scissors => 3,
    } + match round {
        (Item::Rock, Item::Rock) => 3,
        (Item::Rock, Item::Paper) => 6,
        (Item::Rock, Item::Scissors) => 0,
        (Item::Paper, Item::Rock) => 0,
        (Item::Paper, Item::Paper) => 3,
        (Item::Paper, Item::Scissors) => 6,
        (Item::Scissors, Item::Rock) => 6,
        (Item::Scissors, Item::Paper) => 0,
        (Item::Scissors, Item::Scissors) => 3,
    };
}

fn parse_line(line: &str) -> Option<(Item, Item)> {
    let mut chosen_items = line.split(' ');

    fn next_item<'a, T: Iterator<Item = &'a str>>(chosen_items: &mut T) -> Option<Item> {
        chosen_items.next().map(Item::parse)?
    }

    let opponent_item = next_item(&mut chosen_items)?;
    let my_item = next_item(&mut chosen_items)?;

    Some((opponent_item, my_item))
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
