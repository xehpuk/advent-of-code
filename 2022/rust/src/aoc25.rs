use super::Day;

pub struct Day25;

type Decimal = i64;
type Snafu = String;

const SNAFU_BASE: Decimal = 5;

impl<'a, I> Day<'a, I, Snafu> for Day25
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(input: I) -> Option<Snafu> {
        input
            .map(snafu_to_decimal)
            .sum::<Option<Decimal>>()
            .map(decimal_to_snafu)
    }

    fn part2(_: I) -> Option<Snafu> {
        None
    }
}

fn snafu_to_decimal(snafu: &str) -> Option<Decimal> {
    let mut n = 0;
    for digit in snafu.chars().map(|digit| {
        Some(match digit {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => {
                return None;
            }
        })
    }) {
        let digit = digit?;
        n *= SNAFU_BASE;
        n += digit;
    }
    Some(n)
}

fn decimal_to_snafu(mut n: Decimal) -> Snafu {
    if n == 0 {
        return "0".to_string();
    }
    let mut snafu = "".to_string();
    while n != 0 {
        let (carry, current) = match n % SNAFU_BASE {
            -4 => (-1, "1"),
            -3 => (-1, "2"),
            -2 => (0, "="),
            -1 => (0, "-"),
            0 => (0, "0"),
            1 => (0, "1"),
            2 => (0, "2"),
            3 => (1, "="),
            4 => (1, "-"),
            _ => unreachable!(),
        };
        snafu = current.to_owned() + &snafu;
        n /= SNAFU_BASE;
        n += carry;
    }
    snafu
}

#[cfg(test)]
mod tests {
    use super::{decimal_to_snafu, snafu_to_decimal, Day, Day25, Decimal, Snafu};
    use std::str::Lines;

    const INPUT: &str = include_str!("../../25_test.txt");

    const CONVERSIONS: [(Decimal, &str); 56] = [
        (976, "2=-01"),
        (-20, "-10"),
        (-19, "-11"),
        (-18, "-12"),
        (-17, "-2="),
        (-16, "-2-"),
        (-15, "-20"),
        (-14, "-21"),
        (-13, "-22"),
        (-12, "=="),
        (-11, "=-"),
        (-10, "=0"),
        (-9, "=1"),
        (-8, "=2"),
        (-7, "-="),
        (-6, "--"),
        (-5, "-0"),
        (-4, "-1"),
        (-3, "-2"),
        (-2, "="),
        (-1, "-"),
        (0, "0"),
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (11, "21"),
        (12, "22"),
        (13, "1=="),
        (14, "1=-"),
        (15, "1=0"),
        (16, "1=1"),
        (17, "1=2"),
        (18, "1-="),
        (19, "1--"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
        (1747, "1=-0-2"),
        (906, "12111"),
        (198, "2=0="),
        (11, "21"),
        (201, "2=01"),
        (31, "111"),
        (1257, "20012"),
        (32, "112"),
        (353, "1=-1="),
        (107, "1-12"),
        (37, "122"),
    ];

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<Snafu> {
        Day25::part1(test_input(input))
    }

    #[test]
    fn test_snafu_to_decimal() {
        for (decimal, snafu) in CONVERSIONS {
            assert_eq!(Some(decimal), snafu_to_decimal(snafu));
        }
    }

    #[test]
    fn test_decimal_to_snafu() {
        for (decimal, snafu) in CONVERSIONS {
            assert_eq!(snafu.to_string(), decimal_to_snafu(decimal));
        }
    }

    #[test]
    fn test1() {
        assert_eq!(Some("2=-1=0".to_string()), test_part1(INPUT));
    }
}
