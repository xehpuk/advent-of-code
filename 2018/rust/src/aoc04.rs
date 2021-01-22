use std::collections::HashMap;
use std::io::Error;

pub fn main() {
    match read_input() {
        Ok(input) => {
            let records = sort_records(&input);
            // println!("{}", records.join("\n"));
            let stats = create_stats(&records);
            // println!("{:#?}", stats);
            print_solution(1, solve_part1(&stats));
            print_solution(2, solve_part2(&stats));
        }
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn solve_part1(stats: &HashMap<u32, [u32; 59]>) -> Option<(u32, (usize, u32), u32)> {
    let strategy1 = |_, minutes_slept_total, (_, _, current_minutes_slept_total)| {
        minutes_slept_total > current_minutes_slept_total
    };

    solve(&stats, strategy1)
}

fn solve_part2(stats: &HashMap<u32, [u32; 59]>) -> Option<(u32, (usize, u32), u32)> {
    let strategy2 = |(_, shifts_slept), _, (_, (_, current_shifts_slept), _)| {
        shifts_slept > current_shifts_slept
    };

    solve(&stats, strategy2)
}

fn print_solution(part: u8, sleepiest_guard: Option<(u32, (usize, u32), u32)>) {
    match sleepiest_guard {
        Some((guard_id, (max_minute, shifts_slept), minutes_slept_total)) => {
            println!(
                "Guard #{} slept the most at minute {} with {} shifts. Total time slept: {} minutes",
                guard_id, max_minute, shifts_slept, minutes_slept_total
            );
            println!(
                "For part {}, this should give an answer of {} * {} = {}.",
                part,
                guard_id,
                max_minute,
                guard_id as usize * max_minute
            );
        }
        None => eprintln!("Couldn't find sleepy guards. Crappy records."),
    }
}

fn sort_records(input: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort_unstable();

    lines
}

// [1518-03-03 00:02] Guard #947 begins shift
// [1518-03-03 00:42] falls asleep
// [1518-03-03 00:55] wakes up
// [1518-03-03 23:48] Guard #557 begins shift
// [1518-03-04 00:02] falls asleep
// [1518-03-04 00:09] wakes up
// [1518-03-04 00:18] falls asleep
// [1518-03-04 00:23] wakes up
// [1518-03-04 00:28] falls asleep
// [1518-03-04 00:38] wakes up
// [1518-03-04 00:43] falls asleep
// [1518-03-04 00:56] wakes up

fn create_stats(records: &[&str]) -> HashMap<u32, [u32; 59]> {
    let mut stats: HashMap<u32, [u32; 59]> = HashMap::new();
    let mut current_guard: Option<&mut [u32; 59]> = None;
    let mut fell_asleep: Option<usize> = None;
    for &record in records {
        match &record[19..24] {
            // first word after timestamp
            "Guard" => current_guard = Some(stats.entry(parse_guard_id(record)).or_insert([0; 59])),
            "falls" => fell_asleep = Some(parse_minute(record)),
            "wakes" => {
                let woke_up = parse_minute(record);
                if let Some(fell_asleep) = fell_asleep {
                    if let Some(ref mut current_guard) = current_guard {
                        for minute in fell_asleep..woke_up {
                            current_guard[minute] += 1;
                        }
                    } else {
                        panic!("404 Guard Not Found")
                    }
                } else {
                    panic!("404 Sleep Not Found")
                }
            }
            unknown => panic!("Record of unknown type: {:?}", unknown),
        }
    }

    stats
}

/// Returns Some<(guard_id, (max_minute, shifts_slept), minutes_slept_total)>
fn solve<F>(stats: &HashMap<u32, [u32; 59]>, strategy: F) -> Option<(u32, (usize, u32), u32)>
where
    F: FnOnce((usize, u32), u32, (u32, (usize, u32), u32)) -> bool + Copy,
{
    let mut sleepiest_guard: Option<(u32, (usize, u32), u32)> = None;
    for (&guard_id, minutes) in stats {
        // (minute, shifts_slept)
        let mut max = (0, 0);
        let mut minutes_slept_total = 0u32;
        for (minute, &shifts_slept) in minutes.iter().enumerate() {
            if shifts_slept > max.1 {
                max = (minute, shifts_slept);
            }
            minutes_slept_total += shifts_slept;
        }
        if let Some(current_sleepiest_guard) = sleepiest_guard {
            if strategy(max, minutes_slept_total, current_sleepiest_guard) {
                sleepiest_guard = Some((guard_id, max, minutes_slept_total));
            }
        } else {
            sleepiest_guard = Some((guard_id, max, minutes_slept_total));
        }
    }

    sleepiest_guard
}

// # is at 25, so ID starts at 26, ends at next space (variable number of digits)
fn parse_guard_id(record: &str) -> u32 {
    // skip first char (can't be space or we'd try to parse an empty string)
    (&record[26..record[27..].find(' ').unwrap() + 27])
        .parse()
        .unwrap()
}

// minute starts at 15, ends at 17 (always two digits)
fn parse_minute(record: &str) -> usize {
    record[15..17].parse().unwrap()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../04.txt")
}

#[cfg(test)]
mod tests {
    use super::{create_stats, solve_part1, solve_part2};
    use std::collections::HashMap;

    const INPUT: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    fn create_test_stats() -> HashMap<u32, [u32; 59]> {
        create_stats(&INPUT.lines().collect::<Vec<_>>())
    }

    #[test]
    fn test1() {
        let stats = create_test_stats();

        let (guard_id, (max_minute, shifts_slept), minutes_slept_total) =
            solve_part1(&stats).unwrap();
        assert_eq!(10, guard_id);
        assert_eq!(50, minutes_slept_total);
        assert_eq!(24, max_minute);
        assert_eq!(2, shifts_slept);
    }

    #[test]
    fn test2() {
        let stats = create_test_stats();

        let (guard_id, (max_minute, shifts_slept), minutes_slept_total) =
            solve_part2(&stats).unwrap();
        assert_eq!(99, guard_id);
        assert_eq!(30, minutes_slept_total);
        assert_eq!(45, max_minute);
        assert_eq!(3, shifts_slept);
    }
}
