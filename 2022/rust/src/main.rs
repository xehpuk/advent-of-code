use std::env::args;
use std::num::ParseIntError;

mod aoc01;

fn main() -> Result<(), ParseIntError> {
    match args().nth(1) {
        Some(day) => match day.parse::<u8>()? {
            1 => aoc01::main(),
            n => eprintln!("Number must be in 1..=1, {} clearly isn't.", n),
        },
        None => {
            println!(
                "Running latest exercise (day 1). You may provide a number, e.g.: cargo run -- 2"
            );
            aoc01::main();
        }
    }

    Ok(())
}
