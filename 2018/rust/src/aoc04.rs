use std::io::Error;

pub fn main() {
    match read_input() {
       Ok(input) => {
           let records = sort_records(&input);
           println!("{}", records.join("\n"))
       },
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

fn solve_part1(records: &[&str]) -> ! {
    todo!()
}

fn solve_part2(records: &[&str]) -> ! {
    todo!()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../04.txt")
}
