use std::io::Error;

#[derive(Clone, Copy)]
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

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn parse(input: &str) -> Option<Self> {
        match input {
            "X" => Some(Self::Lose),
            "Y" => Some(Self::Draw),
            "Z" => Some(Self::Win),
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
        .map(parse_line1)
        .collect::<Option<Vec<(Item, Item)>>>()?;

    Some(items.into_iter().map(calc_score1).sum())
}

fn part2(input: &str) -> Option<i32> {
    // todo: do not collect
    let items = input
        .lines()
        .map(parse_line2)
        .collect::<Option<Vec<(Item, Outcome)>>>()?;

    Some(items.into_iter().map(calc_score2).sum())
}

fn calc_score1(round: (Item, Item)) -> i32 {
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

fn calc_score2(round: (Item, Outcome)) -> i32 {
    return calc_score1((
        round.0,
        match round {
            (Item::Rock, Outcome::Win) => Item::Paper,
            (Item::Rock, Outcome::Draw) => Item::Rock,
            (Item::Rock, Outcome::Lose) => Item::Scissors,
            (Item::Paper, Outcome::Win) => Item::Scissors,
            (Item::Paper, Outcome::Draw) => Item::Paper,
            (Item::Paper, Outcome::Lose) => Item::Rock,
            (Item::Scissors, Outcome::Win) => Item::Rock,
            (Item::Scissors, Outcome::Draw) => Item::Scissors,
            (Item::Scissors, Outcome::Lose) => Item::Paper,
        },
    ));
}

fn parse_line1(line: &str) -> Option<(Item, Item)> {
    let mut chosen_items = line.split(' ');

    fn next_item<'a, T: Iterator<Item = &'a str>>(chosen_items: &mut T) -> Option<Item> {
        chosen_items.next().map(Item::parse)?
    }

    let opponent_item = next_item(&mut chosen_items)?;
    let my_item = next_item(&mut chosen_items)?;

    Some((opponent_item, my_item))
}

fn parse_line2(line: &str) -> Option<(Item, Outcome)> {
    let mut chosen_items = line.split(' ');

    let opponent_item = chosen_items.next().map(Item::parse)??;
    let outcome = chosen_items.next().map(Outcome::parse)??;

    Some((opponent_item, outcome))
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../02.txt")
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "A Y,B X,C Z";

    fn test_input(input: &str) -> String {
        input.split(',').collect::<Vec<_>>().join("\n")
    }

    fn test_part1(input: &str) -> Option<i32> {
        part1(&test_input(input))
    }

    fn test_part2(input: &str) -> Option<i32> {
        part2(&test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(15), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(Some(12), test_part2(INPUT));
    }
}
