#[derive(Clone, Copy)]
enum Item {
    Rock,
    Paper,
    Scissors,
}

impl Item {
    fn parse_opponent(input: &str) -> Option<Self> {
        match input {
            "A" => Some(Self::Rock),
            "B" => Some(Self::Paper),
            "C" => Some(Self::Scissors),
            &_ => None,
        }
    }

    fn parse_mine(input: &str) -> Option<Self> {
        match input {
            "X" => Some(Self::Rock),
            "Y" => Some(Self::Paper),
            "Z" => Some(Self::Scissors),
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

pub fn part1(input: &str) -> Option<i32> {
    // todo: do not collect
    let items = input
        .lines()
        .map(parse_line1)
        .collect::<Option<Vec<(Item, Item)>>>()?;

    Some(items.into_iter().map(calc_score1).sum())
}

pub fn part2(input: &str) -> Option<i32> {
    // todo: do not collect
    let items = input
        .lines()
        .map(parse_line2)
        .collect::<Option<Vec<(Item, Outcome)>>>()?;

    Some(items.into_iter().map(calc_score2).sum())
}

fn calc_score1(round: (Item, Item)) -> i32 {
    (match round.1 {
        Item::Rock => 1,
        Item::Paper => 2,
        Item::Scissors => 3,
    }) + match round {
        (Item::Rock, Item::Rock) => 3,
        (Item::Rock, Item::Paper) => 6,
        (Item::Rock, Item::Scissors) => 0,
        (Item::Paper, Item::Rock) => 0,
        (Item::Paper, Item::Paper) => 3,
        (Item::Paper, Item::Scissors) => 6,
        (Item::Scissors, Item::Rock) => 6,
        (Item::Scissors, Item::Paper) => 0,
        (Item::Scissors, Item::Scissors) => 3,
    }
}

fn calc_score2(round: (Item, Outcome)) -> i32 {
    calc_score1((
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
    ))
}

fn parse_line<T, U>(
    line: &str,
    first: fn(&str) -> Option<T>,
    second: fn(&str) -> Option<U>,
) -> Option<(T, U)> {
    let mut chosen_items = line.split(' ');

    let opponent_item = chosen_items.next().map(first)??;
    let my_item = chosen_items.next().map(second)??;

    Some((opponent_item, my_item))
}

fn parse_line1(line: &str) -> Option<(Item, Item)> {
    parse_line(line, Item::parse_opponent, Item::parse_mine)
}

fn parse_line2(line: &str) -> Option<(Item, Outcome)> {
    parse_line(line, Item::parse_opponent, Outcome::parse)
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
