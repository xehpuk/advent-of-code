use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::Error;
use std::num::ParseIntError;
use std::str::Chars;
use std::str::FromStr;

#[derive(Debug)]
struct Rect {
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Claim {
    id: u32,
    rect: Rect,
}

#[derive(Debug)]
enum ZeroError {
    Width,
    Height,
}

#[derive(Debug)]
enum ParseError {
    MissingId,
    MissingSpace,
    MissingAt,
    MissingComma,
    MissingColon,
    MissingX,
    DanglingInput,
    Int(ParseIntError),
    Invalid(ZeroError),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        Self::Int(err)
    }
}

impl Rect {
    fn new(left: u32, top: u32, width: u32, height: u32) -> Result<Self, ZeroError> {
        if width == 0 {
            return Err(ZeroError::Width);
        }
        if height == 0 {
            return Err(ZeroError::Height);
        }

        Ok(Self {
            left,
            top,
            width,
            height,
        })
    }

    fn right(&self) -> u32 {
        self.left + self.width
    }

    fn bottom(&self) -> u32 {
        self.top + self.height
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        let left_x = max(self.left, other.left);
        let right_x = min(self.right(), other.right());
        let top_y = max(self.top, other.top);
        let bottom_y = min(self.bottom(), other.bottom());

        if left_x < right_x && top_y < bottom_y {
            Some(Self {
                left: left_x,
                top: top_y,
                width: right_x - left_x,
                height: bottom_y - top_y,
            })
        } else {
            None
        }
    }
}

impl Claim {
    fn new(id: u32, left: u32, top: u32, width: u32, height: u32) -> Result<Self, ZeroError> {
        Ok(Self {
            id,
            rect: Rect::new(left, top, width, height)?,
        })
    }
}

impl FromStr for Claim {
    type Err = ParseError;

    /// Parses "[#]id[ @ ]left[,]top[: ]width[x]height",
    /// e.g. "#1287 @ 9,101: 23x21"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let parse_u32 = |chars: &mut Chars| -> Result<(u32, Option<char>), Self::Err> {
            let mut s = String::new();
            while let Some(c) = chars.next() {
                if c.is_ascii_digit() {
                    s.push(c);
                } else {
                    let n = s.parse()?;
                    return Ok((n, Some(c)));
                }
            }
            let n = s.parse()?;
            Ok((n, None))
        };
        if Some('#') != chars.next() {
            return Err(Self::Err::MissingId);
        }
        let (id, next) = parse_u32(&mut chars)?;
        if Some(' ') != next {
            return Err(Self::Err::MissingSpace);
        }
        if Some('@') != chars.next() {
            return Err(Self::Err::MissingAt);
        }
        if Some(' ') != chars.next() {
            return Err(Self::Err::MissingSpace);
        }
        let (left, next) = parse_u32(&mut chars)?;
        if Some(',') != next {
            return Err(Self::Err::MissingComma);
        }
        let (top, next) = parse_u32(&mut chars)?;
        if Some(':') != next {
            return Err(Self::Err::MissingColon);
        }
        if Some(' ') != chars.next() {
            return Err(Self::Err::MissingSpace);
        }
        let (width, next) = parse_u32(&mut chars)?;
        if Some('x') != next {
            return Err(Self::Err::MissingX);
        }
        let (height, next) = parse_u32(&mut chars)?;
        if None != next {
            return Err(Self::Err::DanglingInput);
        }

        Self::new(id, left, top, width, height).map_err(|err| ParseError::Invalid(err))
    }
}

fn main() {
    match read_input() {
        Ok(text) => match solve_part1(&text) {
            Ok(count) => println!("{:?}", count),
            Err(err) => eprintln!("{:#?}", err),
        },
        Err(err) => eprintln!("{:#?}", err),
    }
}

fn solve_part1(input: &str) -> Result<usize, ParseError> {
    let rects = input
        .lines()
        .map(Claim::from_str)
        .collect::<Result<Vec<Claim>, _>>()?;

    let mut squares = HashSet::new();
    for (i, claim) in rects.iter().enumerate() {
        for claim2 in &rects[i + 1..] {
            if let Some(intersection) = claim.rect.intersect(&claim2.rect) {
                for x in intersection.left..intersection.right() {
                    for y in intersection.top..intersection.bottom() {
                        squares.insert((x, y));
                    }
                }
            }
        }
    }

    Ok(squares.len())
}

#[allow(dead_code)]
fn solve_part2(_input: &str) -> ! {
    todo!()
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../03.txt")
}
