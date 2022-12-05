use super::Day;
use std::ops::RangeInclusive;

pub struct Day04;

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I> for Day04 {
    fn part1(input: I) -> Option<i32> {
        let mut containing_assignment_pairs = 0;
        for line in input {
            let vec = line.split(',').collect::<Vec<_>>();
            let vec = vec
                .into_iter()
                .map(|range| range.split('-').collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let vec = vec
                .into_iter()
                .map(|range| {
                    let start = range.get(0).unwrap().parse::<i32>().unwrap();
                    let end = range.get(1).unwrap().parse::<i32>().unwrap();
                    start..=end
                })
                .collect::<Vec<_>>();
            let first_range = vec.get(0).unwrap();
            let second_range = vec.get(1).unwrap();
            if (first_range.start() <= second_range.start()
                && first_range.end() >= second_range.end())
                || (second_range.start() <= first_range.start()
                    && second_range.end() >= first_range.end())
            {
                containing_assignment_pairs += 1;
            }
        }
        Some(containing_assignment_pairs)
        // let ranges = input.map(|line| line.split(',').collect::<Vec<_>>());
        // let ids: Vec<_> = input
        //     .flat_map(|line| line.split(','))
        //     .flat_map(|range| range.split('-'))
        //     .flat_map(|id| id.parse::<i32>().ok())
        //     .collect();
        //
        // ids.chunks_exact(4);

        // for line in input {
        //     let ranges = line.splitn(2, ',');
        //     let ranges = line
        //         .splitn(2, ',')
        //         .map(|range| range.splitn(2, '-').map(|id| id.parse::<i32>().ok()));
        //     for mut range in ranges {
        //         // let mut ids = range.splitn(2, '-').map(|id| id.parse::<i32>().ok());
        //         // let range: RangeInclusive<i32> = ids.next()??..=ids.next()??;
        //         // for id in ids {
        //         //     let id_number = id.parse::<i32>().ok()?;
        //         // }
        //     }
        // }
        // input.map(|line| {
        //     line.split(',')
        //         .map(|range| range.split('-').map(|id| id.parse::<i32>()))
        // });
        // todo!()
    }

    fn part2(input: I) -> Option<i32> {
        let mut overlapping_assignment_pairs = 0;
        for line in input {
            let vec = line.split(',').collect::<Vec<_>>();
            let vec = vec
                .into_iter()
                .map(|range| range.split('-').collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let vec = vec
                .into_iter()
                .map(|range| {
                    let start = range.get(0).unwrap().parse::<i32>().unwrap();
                    let end = range.get(1).unwrap().parse::<i32>().unwrap();
                    start..=end
                })
                .collect::<Vec<_>>();
            let first_range = vec.get(0).unwrap();
            let second_range = vec.get(1).unwrap();
            if first_range.start() <= second_range.end()
                && second_range.start() <= first_range.end()
            {
                overlapping_assignment_pairs += 1;
            }
        }
        Some(overlapping_assignment_pairs)
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day04};
    use std::str::Lines;

    const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    fn test_input(input: &str) -> Lines {
        input.lines()
    }

    fn test_part1(input: &str) -> Option<i32> {
        Day04::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<i32> {
        Day04::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(2), test_part1(INPUT));
    }

    #[test]
    fn test2() {
        assert_eq!(Some(4), test_part2(INPUT));
    }
}
