use std::collections::HashMap;

use super::Day;

pub struct Day21;

type Number = i64;

const NAME_LEN: usize = 4;
const ROOT_NAME: &str = "root";
const MY_NAME: &str = "humn";

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

        do_job(&monkeys, ROOT_NAME)
    }

    fn part2(input: I) -> Option<Number> {
        let monkeys = parse_monkeys(input)?;

        // todo: improve performance by making "contains" return the call chain (or empty)
        // but it's ~15ms for the whole day anyway
        match monkeys.get(ROOT_NAME)? {
            Job::Const(_) => None,
            Job::Expr(operation) => {
                let (known_operand, unknown_operand) = {
                    if contains(&monkeys, &operation.first_operand, MY_NAME) {
                        (&operation.second_operand, &operation.first_operand)
                    } else {
                        (&operation.first_operand, &operation.second_operand)
                    }
                };
                let known_job = do_job(&monkeys, known_operand)?;

                find_unknown(&monkeys, unknown_operand, known_job)
            }
        }
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

fn do_job(monkeys: &HashMap<String, Job>, monkey: &str) -> Option<Number> {
    Some(match monkeys.get(monkey)? {
        Job::Const(number) => *number,
        Job::Expr(operation) => {
            let first_job = do_job(monkeys, &operation.first_operand)?;
            let second_job = do_job(monkeys, &operation.second_operand)?;
            match operation.operator {
                Operator::Add => first_job + second_job,
                Operator::Sub => first_job - second_job,
                Operator::Mul => first_job * second_job,
                Operator::Div => first_job / second_job,
            }
        }
    })
}

fn contains(monkeys: &HashMap<String, Job>, monkey: &str, needle: &str) -> bool {
    monkey == needle || {
        match monkeys.get(monkey) {
            Some(Job::Const(_)) => false,
            Some(Job::Expr(operation)) => {
                contains(monkeys, &operation.first_operand, needle)
                    || contains(monkeys, &operation.second_operand, needle)
            }
            None => false,
        }
    }
}

fn find_unknown(
    monkeys: &HashMap<String, Job>,
    monkey: &str,
    needed_job: Number,
) -> Option<Number> {
    match monkeys.get(monkey)? {
        Job::Const(_) => Some(needed_job),
        Job::Expr(operation) => {
            let (known_operand, unknown_operand) = {
                if contains(&monkeys, &operation.first_operand, MY_NAME) {
                    (&operation.second_operand, &operation.first_operand)
                } else {
                    (&operation.first_operand, &operation.second_operand)
                }
            };
            let known_job = do_job(&monkeys, known_operand)?;
            let needed_job = match operation.operator {
                Operator::Add => needed_job - known_job,
                Operator::Sub => {
                    if known_operand == &operation.first_operand {
                        known_job - needed_job
                    } else {
                        needed_job + known_job
                    }
                }
                Operator::Mul => needed_job / known_job,
                Operator::Div => {
                    if known_operand == &operation.first_operand {
                        known_job / needed_job
                    } else {
                        needed_job * known_job
                    }
                }
            };

            find_unknown(&monkeys, unknown_operand, needed_job)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{contains, parse_monkeys, Day, Day21, Number, MY_NAME, ROOT_NAME};
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
    fn test_contains() {
        let monkeys = parse_monkeys(test_input(INPUT)).expect("Input unparsable!");

        assert!(contains(&monkeys, ROOT_NAME, MY_NAME));
        assert!(!contains(&monkeys, "sjmn", MY_NAME));
        assert!(!contains(&monkeys, "drzm", MY_NAME));
        assert!(!contains(&monkeys, "hmdt", MY_NAME));
        assert!(!contains(&monkeys, "zczc", MY_NAME));
        assert!(!contains(&monkeys, "dbpl", MY_NAME));
        assert!(contains(&monkeys, "pppw", MY_NAME));
        assert!(contains(&monkeys, "cczh", MY_NAME));
        assert!(!contains(&monkeys, "sllz", MY_NAME));
        assert!(contains(&monkeys, "lgvd", MY_NAME));
        assert!(!contains(&monkeys, "ljgn", MY_NAME));
        assert!(contains(&monkeys, "ptdq", MY_NAME));
        assert!(contains(&monkeys, "humn", MY_NAME));
        assert!(!contains(&monkeys, "dvpt", MY_NAME));
        assert!(!contains(&monkeys, "lfqf", MY_NAME));
    }

    #[test]
    fn test2() {
        assert_eq!(Some(301), test_part2(INPUT));
    }
}
