use super::Day;
use std::collections::HashSet;
use std::str::FromStr;

use Direction::{Down, Left, Right, Up};

pub struct Day09;

struct Motion {
    direction: Direction,
    steps: i32,
}

#[derive(Copy, Clone, Debug)]
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
            let mut iter = rope.iter_mut();
            let mut head = iter.next().unwrap();
            match direction {
                Left => head.0 -= 1,
                Right => head.0 += 1,
                Up => head.1 += 1,
                Down => head.1 -= 1,
            }
            for knot in iter {
                if !are_touching(head, knot) {
                    *knot = follow(head, knot);
                }
                head = knot;
            }
            tail_positions.insert(*head);
        };

        for motion in input.map(str::parse::<Motion>) {
            let Motion { direction, steps } = motion.ok()?;
            for _ in 0..steps {
                step(direction);
            }
        }

        Some(tail_positions.len())
    }
}

fn are_touching(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
}

fn follow(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let new_x = if head.0 > tail.0 {
        tail.0 + 1
    } else if head.0 < tail.0 {
        tail.0 - 1
    } else {
        tail.0
    };
    let new_y = if head.1 > tail.1 {
        tail.1 + 1
    } else if head.1 < tail.1 {
        tail.1 - 1
    } else {
        tail.1
    };
    (new_x, new_y)
}

#[cfg(test)]
mod tests {
    use super::{follow, Day, Day09};
    use crate::aoc09::are_touching;
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

    #[test]
    fn test_are_touching() {
        assert_eq!(false, are_touching(&(2, 2), &(0, 0)));
        assert_eq!(false, are_touching(&(2, 1), &(0, 0)));
        assert_eq!(false, are_touching(&(1, 2), &(0, 0)));
        assert_eq!(false, are_touching(&(-2, -2), &(0, 0)));
        assert_eq!(false, are_touching(&(-2, -1), &(0, 0)));
        assert_eq!(false, are_touching(&(-1, -2), &(0, 0)));

        assert_eq!(true, are_touching(&(0, 0), &(0, 0)));
        assert_eq!(true, are_touching(&(0, 1), &(0, 0)));
        assert_eq!(true, are_touching(&(1, 0), &(0, 0)));
        assert_eq!(true, are_touching(&(1, 1), &(0, 0)));
        assert_eq!(true, are_touching(&(0, -1), &(0, 0)));
        assert_eq!(true, are_touching(&(-1, 0), &(0, 0)));
        assert_eq!(true, are_touching(&(-1, -1), &(0, 0)));

        assert_eq!(true, are_touching(&(0, 0), &(-1, -1)));
        assert_eq!(true, are_touching(&(0, -1), &(-1, -1)));
        assert_eq!(true, are_touching(&(-1, 0), &(-1, -1)));
        assert_eq!(true, are_touching(&(-1, -1), &(-1, -1)));
        assert_eq!(true, are_touching(&(-2, -1), &(-1, -1)));
        assert_eq!(true, are_touching(&(-1, -2), &(-1, -1)));
        assert_eq!(true, are_touching(&(-2, -2), &(-1, -1)));
    }

    #[test]
    fn test_follow() {
        assert_eq!((1, 1), follow(&(2, 2), &(0, 0)));
        assert_eq!((1, 1), follow(&(2, 1), &(0, 0)));
        assert_eq!((1, 1), follow(&(1, 2), &(0, 0)));

        assert_eq!((-1, -1), follow(&(-2, -2), &(0, 0)));
        assert_eq!((-1, -1), follow(&(-2, -1), &(0, 0)));
        assert_eq!((-1, -1), follow(&(-1, -2), &(0, 0)));

        assert_eq!((0, 0), follow(&(1, 1), &(-1, -1)));
        assert_eq!((0, 0), follow(&(1, 0), &(-1, -1)));
        assert_eq!((0, 0), follow(&(0, 1), &(-1, -1)));

        assert_eq!((-2, -2), follow(&(-3, -3), &(-1, -1)));
        assert_eq!((-2, -2), follow(&(-3, -2), &(-1, -1)));
        assert_eq!((-2, -2), follow(&(-2, -3), &(-1, -1)));

        assert_eq!((-10, -4), follow(&(-11, -4), &(-9, -5)));
    }
}
