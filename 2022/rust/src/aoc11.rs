use super::Day;

use std::str::FromStr;
use Operand::{Int, Old};
use Operator::{Add, Mul};

pub struct Day11;

#[derive(Debug, PartialEq)]
struct MonkeyError;

#[derive(Debug, PartialEq)]
enum Operand {
    Int(i64),
    Old,
}

impl FromStr for Operand {
    type Err = MonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Old),
            _ => Ok(Int(s.parse().or(Err(MonkeyError))?)),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Mul,
}

impl FromStr for Operator {
    type Err = MonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Add),
            "*" => Ok(Mul),
            _ => Err(MonkeyError),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

impl FromStr for Operation {
    type Err = MonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s["  Operation: new = old ".len()..].split(' ');
        let operator = iter.next().ok_or(MonkeyError)?.parse()?;
        let operand = iter.next().ok_or(MonkeyError)?.parse()?;

        Ok(Self { operator, operand })
    }
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    if_true: usize,
    if_false: usize,
}

impl FromStr for Monkey {
    type Err = MonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1); // skip "Monkey {N}:"
        let items = lines.next().ok_or(MonkeyError)?["  Starting items: ".len()..]
            .split(", ")
            .map(str::parse)
            .collect::<Result<Vec<i64>, _>>()
            .or(Err(MonkeyError))?;
        let operation = lines.next().ok_or(MonkeyError)?.parse::<Operation>()?;
        let test = lines.next().ok_or(MonkeyError)?["  Test: divisible by ".len()..]
            .parse()
            .or(Err(MonkeyError))?;
        let if_true = lines.next().ok_or(MonkeyError)?["    If true: throw to monkey ".len()..]
            .parse()
            .or(Err(MonkeyError))?;
        let if_false = lines.next().ok_or(MonkeyError)?["    If false: throw to monkey ".len()..]
            .parse()
            .or(Err(MonkeyError))?;

        Ok(Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
        })
    }
}

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, usize> for Day11 {
    fn part1(mut input: I) -> Option<usize> {
        let mut monkeys = parse_monkeys(&mut input).ok()?;
        let mut inspections = vec![0; monkeys.len()];

        for _ in 0..20 {
            for i in 0..monkeys.len() {
                inspections[i] += monkeys[i].items.len();
                for mut item in monkeys[i].items.drain(..).collect::<Vec<_>>() {
                    item = match &monkeys[i].operation {
                        Operation {
                            operator: Add {},
                            operand: Int(summand),
                        } => item + summand,
                        Operation {
                            operator: Add {},
                            operand: Old {},
                        } => item + item,
                        Operation {
                            operator: Mul {},
                            operand: Int(factor),
                        } => item * factor,
                        Operation {
                            operator: Mul {},
                            operand: Old {},
                        } => item * item,
                    };
                    item /= 3;
                    let test = if item % monkeys[i].test == 0 {
                        monkeys[i].if_true
                    } else {
                        monkeys[i].if_false
                    };
                    monkeys[test].items.push(item);
                }
            }
        }

        inspections.sort_unstable();

        let mut iter = inspections.iter().rev();

        Some(iter.next()? * iter.next()?)
    }

    fn part2(mut input: I) -> Option<usize> {
        let mut monkeys = parse_monkeys(&mut input).ok()?;
        let modulo: i64 = monkeys.iter().map(|monkey| monkey.test).product();
        let mut inspections = vec![0; monkeys.len()];

        for _ in 0..10_000 {
            for i in 0..monkeys.len() {
                inspections[i] += monkeys[i].items.len();
                for mut item in monkeys[i].items.drain(..).collect::<Vec<_>>() {
                    item = match &monkeys[i].operation {
                        Operation {
                            operator: Add {},
                            operand: Int(summand),
                        } => item + *summand,
                        Operation {
                            operator: Add {},
                            operand: Old {},
                        } => item + item,
                        Operation {
                            operator: Mul {},
                            operand: Int(factor),
                        } => item * *factor,
                        Operation {
                            operator: Mul {},
                            operand: Old {},
                        } => item * item,
                    } % modulo;
                    let test = if item % monkeys[i].test == 0 {
                        monkeys[i].if_true
                    } else {
                        monkeys[i].if_false
                    };
                    monkeys[test].items.push(item);
                }
            }
        }

        inspections.sort_unstable();

        let mut iter = inspections.iter().rev();

        Some(iter.next()? * iter.next()?)
    }
}

fn parse_monkeys<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<Vec<Monkey>, MonkeyError> {
    let mut monkeys = vec![];
    loop {
        let monkey_string = iter
            .take_while(|line| !line.is_empty())
            .map(|line| line.to_string() + "\n")
            .collect::<String>();
        if monkey_string.is_empty() {
            break;
        }
        monkeys.push(monkey_string.parse()?);
    }
    Ok(monkeys)
}

#[cfg(test)]
mod tests {
    use super::{Day, Day11, Monkey, MonkeyError, Operand, Operation, Operator};

    use std::str::Lines;

    const INPUT: &str = include_str!("../../11_test.txt");

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<usize> {
        Day11::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<usize> {
        Day11::part2(test_input(input))
    }

    #[test]
    fn test_parse_monkey() {
        assert_eq!(Err(MonkeyError), "".parse::<Monkey>());
        assert_eq!(Err(MonkeyError), "Monkey 0:".parse::<Monkey>());
        assert_eq!(
            Err(MonkeyError),
            "Monkey 0:
  Starting items: 79, 98"
                .parse::<Monkey>()
        );
        assert_eq!(
            Err(MonkeyError),
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19"
                .parse::<Monkey>()
        );
        assert_eq!(
            Err(MonkeyError),
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23"
                .parse::<Monkey>()
        );
        assert_eq!(
            Err(MonkeyError),
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2"
                .parse::<Monkey>()
        );
        assert_eq!(
            Ok(Monkey {
                items: vec![79, 98],
                operation: Operation {
                    operator: Operator::Mul,
                    operand: Operand::Int(19)
                },
                test: 23,
                if_true: 2,
                if_false: 3
            }),
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"
                .parse::<Monkey>()
        );
        assert_eq!(
            Ok(Monkey {
                items: vec![54, 65, 75, 74],
                operation: Operation {
                    operator: Operator::Add,
                    operand: Operand::Int(6)
                },
                test: 19,
                if_true: 2,
                if_false: 0
            }),
            "Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0"
                .parse::<Monkey>()
        );
        assert_eq!(
            Ok(Monkey {
                items: vec![79, 60, 97],
                operation: Operation {
                    operator: Operator::Mul,
                    operand: Operand::Old
                },
                test: 13,
                if_true: 1,
                if_false: 3
            }),
            "Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3"
                .parse::<Monkey>()
        );
        assert_eq!(
            Ok(Monkey {
                items: vec![74],
                operation: Operation {
                    operator: Operator::Add,
                    operand: Operand::Int(3)
                },
                test: 17,
                if_true: 0,
                if_false: 1
            }),
            "Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
                .parse::<Monkey>()
        );
    }

    #[test]
    fn test1() {
        assert_eq!(Some(10605), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(Some(2713310158), test_part2(INPUT));
    }
}
