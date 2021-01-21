use std::collections::HashMap;
use std::io::Error;

pub fn main() {
    match read_input() {
        Ok(input) => {
            let records = sort_records(&input);
            // println!("{}", records.join("\n"));
            match solve_part1(&records) {
                Some((guard_id, (max_minute, minutes_slept))) => {
                    println!(
                        "Guard #{} slept the most at minute {} with {} minutes over all their shifts.",
                        guard_id, max_minute, minutes_slept
                    );
                    // FIXME: 1459 * 39 = 56901 is wrong :(
                    println!(
                        "This should give an answer of {} * {} = {}.",
                        guard_id,
                        max_minute,
                        guard_id as usize * max_minute
                    )
                }
                None => eprintln!("Couldn't find sleepy guards. Crappy records."),
            }
            solve_part2(&records);
        }
        Err(err) => eprintln!("{:#?}", err),
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

/// Returns Some<(guard_id, (max_minute, minutes_slept))>
fn solve_part1(records: &[&str]) -> Option<(u32, (usize, u32))> {
    let mut map: HashMap<u32, [u32; 59]> = HashMap::new();
    let mut current_guard: Option<&mut [u32; 59]> = None;
    let mut fell_asleep: Option<usize> = None;
    for &record in records {
        match &record[19..24] {
            // first word after timestamp
            "Guard" => current_guard = Some(map.entry(parse_guard_id(record)).or_insert([0; 59])),
            "falls" => fell_asleep = Some(parse_minute(record)),
            "wakes" => {
                let woke_up = parse_minute(record);
                if let Some(fell_asleep) = fell_asleep {
                    if let Some(ref mut current_guard) = current_guard {
                        for minute in fell_asleep..woke_up {
                            current_guard[minute] += 1;
                        }
                    }
                }
            }
            unknown => panic!("Record of unknown type: {:?}", unknown),
        }
    }

    let mut sleepiest_guard: Option<(u32, (usize, u32))> = None;
    for (&guard_id, minutes) in &map {
        let mut max = (0, 0);
        for (minute, &minutes_slept) in minutes.iter().enumerate() {
            if minutes_slept > max.1 {
                max = (minute, minutes_slept);
            }
        }
        if let Some(current_sleepiest_guard) = sleepiest_guard {
            if max.1 > current_sleepiest_guard.1 .1 {
                sleepiest_guard = Some((guard_id, max));
            }
        } else {
            sleepiest_guard = Some((guard_id, max));
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

fn solve_part2(_records: &[&str]) -> ! {
    todo!()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../04.txt")
}
