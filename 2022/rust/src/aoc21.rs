use std::collections::HashMap;

use super::Day;

pub struct Day21;

type Number = i32;

const NAME_LEN: usize = 4;

#[derive(Debug)]
enum Job {
    Const(Number),
    Expr(Operation),
}

#[derive(Debug)]
struct Operation {
    first_operand: String,
    second_operand: String,
    operator: Operator,
}

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl<'a, I> Day<'a, I, Number> for Day21
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(input: I) -> Option<Number> {
        let monkeys = parse_monkeys(input)?;

        println!("{monkeys:#?}");

        todo!()
    }

    fn part2(_input: I) -> Option<Number> {
        todo!()
    }
}

fn parse_monkeys<'a, I>(input: I) -> Option<HashMap<String, Job>>
where
    I: Clone + Iterator<Item = &'a str>,
{
    input.map(parse_monkey).collect()
}

fn parse_monkey(s: &str) -> Option<(String, Job)> {
    let name = s[..NAME_LEN].to_string();
    let s = s[NAME_LEN..].strip_prefix(": ")?;
    if s.len() == 11 {
        let first_operand = s[..NAME_LEN].to_string();
        let s = s[NAME_LEN..].strip_prefix(' ')?;
        let operator = match &s[..1] {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            _ => return None,
        };
        let s = s[1..].strip_prefix(' ')?;
        let second_operand = s[..NAME_LEN].to_string();
        Some((
            name,
            Job::Expr(Operation {
                first_operand,
                second_operand,
                operator,
            }),
        ))
    } else {
        let number = s.parse::<Number>().ok()?;
        Some((name, Job::Const(number)))
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day21, Number};
    use std::str::Lines;

    const INPUT: &str = include_str!("../../21_test.txt");

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<Number> {
        Day21::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<Number> {
        Day21::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(152), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
