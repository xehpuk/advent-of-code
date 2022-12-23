use super::Day;

pub struct Day20;

type IndexedNumber = (usize, i32);

impl<'a, I> Day<'a, I, i32> for Day20
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(input: I) -> Option<i32> {
        let mut numbers = parse_numbers(input)?;
        decrypt_numbers(&mut numbers);
        let numbers = unenumerate(&numbers);
        find_grove_coordinates(&numbers)
    }

    fn part2(_input: I) -> Option<i32> {
        todo!()
    }
}

fn parse_numbers<'a, I>(input: I) -> Option<Vec<IndexedNumber>>
where
    I: Clone + Iterator<Item = &'a str>,
{
    input
        .map(str::parse::<i32>)
        .map(Result::ok)
        .enumerate()
        .map(|(i, n)| n.map(|n| (i, n)))
        .collect()
}

fn decrypt_numbers(numbers: &mut [IndexedNumber]) {
    let len = numbers.len();
    for i in 0..len {
        let (i_old, shift) = numbers[i];
        if shift == 0 {
            continue;
        }
        let mut i_new = i_old as i32 + shift;
        // element wraps around
        if i_new <= 0 {
            i_new -= 1;
        } else if i_new >= len as i32 {
            i_new += 1;
        }
        let i_new = i_new.rem_euclid(len as i32) as usize;
        if i_new < i_old {
            // element moved to front; move others backward
            for n in numbers.iter_mut() {
                if (i_new..i_old).contains(&n.0) {
                    n.0 += 1;
                }
            }
        } else {
            // element moved to back; move others forward
            for n in numbers.iter_mut() {
                if (i_old + 1..=i_new).contains(&n.0) {
                    n.0 -= 1;
                }
            }
        }
        // last but not least: set new position of element itself
        numbers[i].0 = i_new;
    }
}

fn unenumerate<T, K>(numbers: &[(K, T)]) -> Vec<T>
where
    T: Clone,
    K: Clone + Ord,
{
    let mut numbers = numbers.to_vec();
    numbers.sort_unstable_by_key(|n| n.0.clone());
    numbers.into_iter().map(|n| n.1).collect()
}

fn find_grove_coordinates(numbers: &[i32]) -> Option<i32> {
    let zero_index = numbers.iter().position(|&n| n == 0)?;

    Some(
        [1000, 2000, 3000]
            .map(|i| (zero_index + i) % numbers.len())
            .map(|i| numbers[i])
            .into_iter()
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::{decrypt_numbers, find_grove_coordinates, parse_numbers, unenumerate, Day, Day20};
    use std::str::Split;

    const INPUT: &str = "1,2,-3,3,-2,0,4";

    fn test_input(input: &str) -> Split<char> {
        input.split(',')
    }

    fn test_part1(input: &str) -> Option<i32> {
        Day20::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<i32> {
        Day20::part2(test_input(input))
    }

    #[test]
    fn test_unenumerate() {
        let numbers = parse_numbers(test_input(INPUT)).expect("Input unparsable!");
        assert_eq!(vec![1, 2, -3, 3, -2, 0, 4], unenumerate(&numbers));
    }

    #[test]
    fn test_decrypt_numbers() {
        let mut numbers = parse_numbers(test_input(INPUT)).expect("Input unparsable!");
        decrypt_numbers(&mut numbers);
        assert_eq!(vec![1, 2, -3, 4, 0, 3, -2], unenumerate(&numbers));
    }

    #[test]
    fn test_find_grove_coordinates() {
        assert_eq!(
            Some(3),
            find_grove_coordinates(&vec![1, 2, -3, 4, 0, 3, -2])
        );
    }

    #[test]
    fn test1() {
        assert_eq!(Some(3), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(None, test_part2(INPUT)); // todo replace expected value
    }
}
