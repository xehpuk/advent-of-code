use super::Day;

pub struct Day20;

type Number = i32;
type IndexedNumber = (usize, Number);

impl<'a, I> Day<'a, I, Number> for Day20
where
    I: Clone + Iterator<Item = &'a str>,
{
    fn part1(input: I) -> Option<Number> {
        let mut numbers = parse_numbers(input)?;
        decrypt_numbers(&mut numbers);
        let numbers = unenumerate(&numbers);
        find_grove_coordinates(&numbers)
    }

    fn part2(_input: I) -> Option<Number> {
        todo!()
    }
}

fn parse_numbers<'a, I>(input: I) -> Option<Vec<IndexedNumber>>
where
    I: Clone + Iterator<Item = &'a str>,
{
    input
        .map(str::parse::<Number>)
        .map(Result::ok)
        .enumerate()
        .map(|(i, n)| n.map(|n| (i, n)))
        .collect()
}

fn decrypt_numbers(numbers: &mut [IndexedNumber]) {
    for i in 0..numbers.len() {
        decrypt_number_at(numbers, i);
    }
}

fn decrypt_number_at(numbers: &mut [IndexedNumber], i: usize) {
    let len = numbers.len();
    let (i_old, shift) = numbers[i];
    if shift == 0 {
        return;
    }
    let i_new = (i_old as Number + shift).rem_euclid(len as Number - 1) as usize;
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

fn unenumerate<T, K>(numbers: &[(K, T)]) -> Vec<T>
where
    T: Clone,
    K: Clone + Ord,
{
    let mut numbers = numbers.to_vec();
    numbers.sort_unstable_by_key(|n| n.0.clone());
    numbers.into_iter().map(|n| n.1).collect()
}

fn find_grove_coordinates(numbers: &[Number]) -> Option<Number> {
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
    use super::{
        decrypt_number_at, decrypt_numbers, find_grove_coordinates, parse_numbers, unenumerate,
        Day, Day20, Number,
    };
    use std::str::Split;

    const INPUT: &str = "1,2,-3,3,-2,0,4";

    fn test_input(input: &str) -> Split<char> {
        input.split(',')
    }

    fn test_part1(input: &str) -> Option<Number> {
        Day20::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<Number> {
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
        assert_eq!(vec![-2, 1, 2, -3, 4, 0, 3], unenumerate(&numbers));

        numbers = parse_numbers(test_input("0,2,3,5,7,11,13,17,19")).unwrap();
        decrypt_numbers(&mut numbers);
        assert_eq!(vec![0, 13, 7, 19, 2, 3, 5, 11, 17], unenumerate(&numbers));
    }

    #[test]
    fn test_decrypt_number_at() {
        let mut numbers = parse_numbers(test_input(INPUT)).expect("Input unparsable!");
        decrypt_number_at(&mut numbers, 0);
        assert_eq!(vec![2, 1, -3, 3, -2, 0, 4], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 1);
        assert_eq!(vec![1, -3, 2, 3, -2, 0, 4], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 2);
        assert_eq!(vec![1, 2, 3, -2, -3, 0, 4], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 3);
        assert_eq!(vec![1, 2, -2, -3, 0, 3, 4], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 4);
        assert_eq!(vec![-2, 1, 2, -3, 0, 3, 4], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 5);
        assert_eq!(vec![-2, 1, 2, -3, 0, 3, 4], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 6);
        assert_eq!(vec![-2, 1, 2, -3, 4, 0, 3], unenumerate(&numbers));

        numbers = parse_numbers(test_input("-3,-2,-1,0,1,2,3")).unwrap();
        decrypt_number_at(&mut numbers, 0);
        assert_eq!(vec![-2, -1, 0, -3, 1, 2, 3], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 1);
        assert_eq!(vec![-1, 0, -3, 1, -2, 2, 3], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 2);
        assert_eq!(vec![0, -3, 1, -2, 2, -1, 3], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 3);
        assert_eq!(vec![0, -3, 1, -2, 2, -1, 3], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 4);
        assert_eq!(vec![0, -3, -2, 1, 2, -1, 3], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 5);
        assert_eq!(vec![2, 0, -3, -2, 1, -1, 3], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 6);
        assert_eq!(vec![2, 0, -3, 3, -2, 1, -1], unenumerate(&numbers));

        numbers = parse_numbers(test_input("0,2,3,5,7,11,13,17,19")).unwrap();
        decrypt_number_at(&mut numbers, 0);
        assert_eq!(vec![0, 2, 3, 5, 7, 11, 13, 17, 19], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 1);
        assert_eq!(vec![0, 3, 5, 2, 7, 11, 13, 17, 19], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 2);
        assert_eq!(vec![0, 5, 2, 7, 3, 11, 13, 17, 19], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 3);
        assert_eq!(vec![0, 2, 7, 3, 11, 13, 5, 17, 19], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 4);
        assert_eq!(vec![0, 7, 2, 3, 11, 13, 5, 17, 19], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 5);
        assert_eq!(vec![0, 7, 2, 3, 13, 5, 17, 11, 19], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 6);
        assert_eq!(vec![0, 13, 7, 2, 3, 5, 17, 11, 19], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 7);
        assert_eq!(vec![0, 13, 7, 2, 3, 5, 11, 17, 19], unenumerate(&numbers));
        decrypt_number_at(&mut numbers, 8);
        assert_eq!(vec![0, 13, 7, 19, 2, 3, 5, 11, 17], unenumerate(&numbers));
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
        assert_eq!(Some(1623178306), test_part2(INPUT));
    }
}
