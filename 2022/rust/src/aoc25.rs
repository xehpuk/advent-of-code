use super::Day;

pub struct Day25;

type Snafu = String;

impl<'a, I> Day<'a, I, Snafu> for Day25
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(input: I) -> Option<Snafu> {
        input
            .map(snafu_to_decimal)
            .sum::<Option<i64>>()
            .map(decimal_to_snafu)
    }

    fn part2(_input: I) -> Option<Snafu> {
        todo!()
    }
}

fn snafu_to_decimal(snafu: &str) -> Option<i64> {
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
        n *= 5;
        n += digit;
    }
    Some(n)
}

fn decimal_to_snafu(mut n: i64) -> Snafu {
    if n == 0 {
        return "0".to_string();
    }
    let mut snafu = "".to_string();
    while n != 0 {
        let (carry, current) = match n % 5 {
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
        n /= 5;
        n += carry;
    }
    snafu
}

#[cfg(test)]
mod tests {
    use super::{decimal_to_snafu, snafu_to_decimal, Day, Day25, Snafu};
    use std::str::Lines;

    const INPUT: &str = include_str!("../../25_test.txt");

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<Snafu> {
        Day25::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<Snafu> {
        Day25::part2(test_input(input))
    }

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(Some(976), snafu_to_decimal("2=-01"));

        assert_eq!(Some(-20), snafu_to_decimal("-10"));
        assert_eq!(Some(-19), snafu_to_decimal("-11"));
        assert_eq!(Some(-18), snafu_to_decimal("-12"));
        assert_eq!(Some(-17), snafu_to_decimal("-2="));
        assert_eq!(Some(-16), snafu_to_decimal("-2-"));
        assert_eq!(Some(-15), snafu_to_decimal("-20"));
        assert_eq!(Some(-14), snafu_to_decimal("-21"));
        assert_eq!(Some(-13), snafu_to_decimal("-22"));
        assert_eq!(Some(-12), snafu_to_decimal("=="));
        assert_eq!(Some(-11), snafu_to_decimal("=-"));
        assert_eq!(Some(-10), snafu_to_decimal("=0"));
        assert_eq!(Some(-9), snafu_to_decimal("=1"));
        assert_eq!(Some(-8), snafu_to_decimal("=2"));
        assert_eq!(Some(-7), snafu_to_decimal("-="));
        assert_eq!(Some(-6), snafu_to_decimal("--"));
        assert_eq!(Some(-5), snafu_to_decimal("-0"));
        assert_eq!(Some(-4), snafu_to_decimal("-1"));
        assert_eq!(Some(-3), snafu_to_decimal("-2"));
        assert_eq!(Some(-2), snafu_to_decimal("="));
        assert_eq!(Some(-1), snafu_to_decimal("-"));
        assert_eq!(Some(0), snafu_to_decimal("0"));
        assert_eq!(Some(1), snafu_to_decimal("1"));
        assert_eq!(Some(2), snafu_to_decimal("2"));
        assert_eq!(Some(3), snafu_to_decimal("1="));
        assert_eq!(Some(4), snafu_to_decimal("1-"));
        assert_eq!(Some(5), snafu_to_decimal("10"));
        assert_eq!(Some(6), snafu_to_decimal("11"));
        assert_eq!(Some(7), snafu_to_decimal("12"));
        assert_eq!(Some(8), snafu_to_decimal("2="));
        assert_eq!(Some(9), snafu_to_decimal("2-"));
        assert_eq!(Some(10), snafu_to_decimal("20"));
        assert_eq!(Some(11), snafu_to_decimal("21"));
        assert_eq!(Some(12), snafu_to_decimal("22"));
        assert_eq!(Some(13), snafu_to_decimal("1=="));
        assert_eq!(Some(14), snafu_to_decimal("1=-"));
        assert_eq!(Some(15), snafu_to_decimal("1=0"));
        assert_eq!(Some(16), snafu_to_decimal("1=1"));
        assert_eq!(Some(17), snafu_to_decimal("1=2"));
        assert_eq!(Some(18), snafu_to_decimal("1-="));
        assert_eq!(Some(19), snafu_to_decimal("1--"));
        assert_eq!(Some(20), snafu_to_decimal("1-0"));
        assert_eq!(Some(2022), snafu_to_decimal("1=11-2"));
        assert_eq!(Some(12345), snafu_to_decimal("1-0---0"));
        assert_eq!(Some(314159265), snafu_to_decimal("1121-1110-1=0"));

        assert_eq!(Some(1747), snafu_to_decimal("1=-0-2"));
        assert_eq!(Some(906), snafu_to_decimal("12111"));
        assert_eq!(Some(198), snafu_to_decimal("2=0="));
        assert_eq!(Some(11), snafu_to_decimal("21"));
        assert_eq!(Some(201), snafu_to_decimal("2=01"));
        assert_eq!(Some(31), snafu_to_decimal("111"));
        assert_eq!(Some(1257), snafu_to_decimal("20012"));
        assert_eq!(Some(32), snafu_to_decimal("112"));
        assert_eq!(Some(353), snafu_to_decimal("1=-1="));
        assert_eq!(Some(107), snafu_to_decimal("1-12"));
        assert_eq!(Some(37), snafu_to_decimal("122"));
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(decimal_to_snafu(976), "2=-01".to_string());

        assert_eq!(decimal_to_snafu(-20), "-10".to_string());
        assert_eq!(decimal_to_snafu(-19), "-11".to_string());
        assert_eq!(decimal_to_snafu(-18), "-12".to_string());
        assert_eq!(decimal_to_snafu(-17), "-2=".to_string());
        assert_eq!(decimal_to_snafu(-16), "-2-".to_string());
        assert_eq!(decimal_to_snafu(-15), "-20".to_string());
        assert_eq!(decimal_to_snafu(-14), "-21".to_string());
        assert_eq!(decimal_to_snafu(-13), "-22".to_string());
        assert_eq!(decimal_to_snafu(-12), "==".to_string());
        assert_eq!(decimal_to_snafu(-11), "=-".to_string());
        assert_eq!(decimal_to_snafu(-10), "=0".to_string());
        assert_eq!(decimal_to_snafu(-9), "=1".to_string());
        assert_eq!(decimal_to_snafu(-8), "=2".to_string());
        assert_eq!(decimal_to_snafu(-7), "-=".to_string());
        assert_eq!(decimal_to_snafu(-6), "--".to_string());
        assert_eq!(decimal_to_snafu(-5), "-0".to_string());
        assert_eq!(decimal_to_snafu(-4), "-1".to_string());
        assert_eq!(decimal_to_snafu(-3), "-2".to_string());
        assert_eq!(decimal_to_snafu(-2), "=".to_string());
        assert_eq!(decimal_to_snafu(-1), "-".to_string());
        assert_eq!(decimal_to_snafu(0), "0".to_string());
        assert_eq!(decimal_to_snafu(1), "1".to_string());
        assert_eq!(decimal_to_snafu(2), "2".to_string());
        assert_eq!(decimal_to_snafu(3), "1=".to_string());
        assert_eq!(decimal_to_snafu(4), "1-".to_string());
        assert_eq!(decimal_to_snafu(5), "10".to_string());
        assert_eq!(decimal_to_snafu(6), "11".to_string());
        assert_eq!(decimal_to_snafu(7), "12".to_string());
        assert_eq!(decimal_to_snafu(8), "2=".to_string());
        assert_eq!(decimal_to_snafu(9), "2-".to_string());
        assert_eq!(decimal_to_snafu(10), "20".to_string());
        assert_eq!(decimal_to_snafu(11), "21".to_string());
        assert_eq!(decimal_to_snafu(12), "22".to_string());
        assert_eq!(decimal_to_snafu(13), "1==".to_string());
        assert_eq!(decimal_to_snafu(14), "1=-".to_string());
        assert_eq!(decimal_to_snafu(15), "1=0".to_string());
        assert_eq!(decimal_to_snafu(16), "1=1".to_string());
        assert_eq!(decimal_to_snafu(17), "1=2".to_string());
        assert_eq!(decimal_to_snafu(18), "1-=".to_string());
        assert_eq!(decimal_to_snafu(19), "1--".to_string());
        assert_eq!(decimal_to_snafu(20), "1-0".to_string());
        assert_eq!(decimal_to_snafu(2022), "1=11-2".to_string());
        assert_eq!(decimal_to_snafu(12345), "1-0---0".to_string());
        assert_eq!(decimal_to_snafu(314159265), "1121-1110-1=0".to_string());

        assert_eq!(decimal_to_snafu(1747), "1=-0-2".to_string());
        assert_eq!(decimal_to_snafu(906), "12111".to_string());
        assert_eq!(decimal_to_snafu(198), "2=0=".to_string());
        assert_eq!(decimal_to_snafu(11), "21".to_string());
        assert_eq!(decimal_to_snafu(201), "2=01".to_string());
        assert_eq!(decimal_to_snafu(31), "111".to_string());
        assert_eq!(decimal_to_snafu(1257), "20012".to_string());
        assert_eq!(decimal_to_snafu(32), "112".to_string());
        assert_eq!(decimal_to_snafu(353), "1=-1=".to_string());
        assert_eq!(decimal_to_snafu(107), "1-12".to_string());
        assert_eq!(decimal_to_snafu(37), "122".to_string());
    }

    #[test]
    fn test1() {
        assert_eq!(Some("2=-1=0".to_string()), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
