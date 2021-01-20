use std::env::args;
use std::num::ParseIntError;

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;

fn main() -> Result<(), ParseIntError> {
    match args().nth(1) {
        Some(day) => match day.parse::<u8>()? {
            1 => aoc01::main(),
            2 => aoc02::main(),
            3 => aoc03::main(),
            4 => aoc04::main(),
            n => eprintln!("Number must be in 1..=4, {} clearly isn't.", n)
        },
        None => eprintln!("Provide a number, e.g.: cargo run -- 2")
    }

    Ok(())
}
