use super::Day;
use std::collections::HashSet;
use std::str::FromStr;

use Direction::{Down, Left, Right, Up};

pub struct Day09;

struct Motion {
    direction: Direction,
    steps: i32,
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct MotionParseError;

impl FromStr for Direction {
    type Err = MotionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Left),
            "R" => Ok(Right),
            "U" => Ok(Up),
            "D" => Ok(Down),
            &_ => Err(MotionParseError),
        }
    }
}

impl FromStr for Motion {
    type Err = MotionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let direction = iter.next().ok_or(MotionParseError)?.parse()?;
        let steps = iter
            .next()
            .ok_or(MotionParseError)?
            .parse()
            .or(Err(MotionParseError))?;

        Ok(Motion { direction, steps })
    }
}

impl<'a, I: Clone + Iterator<Item = &'a str>> Day<'a, I, usize> for Day09 {
    fn part1(input: I) -> Option<usize> {
        let mut head = (0, 0);
        let mut tail = (0, 0);
        let mut tail_positions = HashSet::new();

        tail_positions.insert(tail);

        let mut step = |direction: Direction| match direction {
            Left => {
                if tail.0 > head.0 {
                    tail = (tail.0 - 1, head.1);
                    tail_positions.insert(tail);
                }
                head.0 -= 1;
            }
            Right => {
                if tail.0 < head.0 {
                    tail = (tail.0 + 1, head.1);
                    tail_positions.insert(tail);
                }
                head.0 += 1;
            }
            Up => {
                if tail.1 < head.1 {
                    tail = (head.0, tail.1 + 1);
                    tail_positions.insert(tail);
                }
                head.1 += 1;
            }
            Down => {
                if tail.1 > head.1 {
                    tail = (head.0, tail.1 - 1);
                    tail_positions.insert(tail);
                }
                head.1 -= 1;
            }
        };

        for motion in input.map(str::parse::<Motion>) {
            let Motion { direction, steps } = motion.ok()?;
            for _ in 0..steps {
                step(direction);
            }
        }

        Some(tail_positions.len())
    }

    fn part2(input: I) -> Option<usize> {
        let start = (0, 0);
        let mut rope = Vec::from([start; 10]);
        let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();

        tail_positions.insert(start);

        let mut step = |direction: Direction| {
            let mut head = *rope.first().unwrap();
            for (i, knot) in rope.iter_mut().enumerate() {
                match direction {
                    Left => {
                        if i == 0 {
                            knot.0 -= 1;
                        } else {
                            let new_x = knot.0 - 1;
                            if head.0 < new_x {
                                *knot = (new_x, head.1);
                            }
                        }
                    }
                    Right => {
                        if i == 0 {
                            knot.0 += 1;
                        } else {
                            let new_x = knot.0 + 1;
                            if head.0 > new_x {
                                *knot = (new_x, head.1);
                            }
                        }
                    }
                    Up => {
                        if i == 0 {
                            knot.1 += 1;
                        } else {
                            let new_y = knot.1 + 1;
                            if head.1 > new_y {
                                *knot = (head.0, new_y);
                            }
                        }
                    }
                    Down => {
                        if i == 0 {
                            knot.1 -= 1;
                        } else {
                            let new_y = knot.1 - 1;
                            if head.1 < new_y {
                                *knot = (head.0, new_y);
                            }
                        }
                    }
                }
                head = *knot;
            }
            tail_positions.insert(head);
        };

        for motion in input.map(str::parse::<Motion>) {
            let Motion { direction, steps } = motion.ok()?;
            for _ in 0..steps {
                step(direction);
            }
        }

        println!("positions: {:?}", tail_positions);

        Some(tail_positions.len())
    }
}

#[cfg(test)]
mod tests {
    use super::{Day, Day09};
    use std::str::Split;

    const INPUT1: &str = "R 4,U 4,L 3,D 1,R 4,D 1,L 5,R 2";
    const INPUT2: &str = "R 5,U 8,L 8,D 3,R 17,D 10,L 25,U 20";

    fn test_input(input: &str) -> Split<char> {
        input.split(',')
    }

    fn test_part1(input: &str) -> Option<usize> {
        Day09::part1(test_input(input))
    }

    fn test_part2(input: &str) -> Option<usize> {
        Day09::part2(test_input(input))
    }

    #[test]
    fn test1() {
        assert_eq!(Some(13), test_part1(INPUT1));
    }

    #[test]
    fn test2() {
        assert_eq!(Some(36), test_part2(INPUT2));
    }
}
