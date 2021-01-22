use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::Error;
use std::num::ParseIntError;
use std::str::{Chars, FromStr};

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

pub fn main() {
    match read_input().map(|text| parse_claims(&text)) {
        Ok(Ok(claims)) => {
            println!("{:?}", solve_part1(&claims));
            println!("{:?}", solve_part2(&claims));
        }
        err => eprintln!("{:#?}", err),
    }
}

fn parse_claims(input: &str) -> Result<Vec<Claim>, ParseError> {
    input.lines().map(Claim::from_str).collect()
}

fn solve_part1(claims: &[Claim]) -> usize {
    let mut squares = HashSet::new();
    for (i, claim) in claims.iter().enumerate() {
        for claim2 in &claims[i + 1..] {
            if let Some(intersection) = claim.rect.intersect(&claim2.rect) {
                for x in intersection.left..intersection.right() {
                    for y in intersection.top..intersection.bottom() {
                        squares.insert((x, y));
                    }
                }
            }
        }
    }

    squares.len()
}

fn solve_part2(claims: &[Claim]) -> Option<&Claim> {
    'a: for (i, claim) in claims.iter().enumerate() {
        for (_, claim2) in claims.iter().enumerate().filter(|&(j, _)| j != i) {
            if let Some(_) = claim.rect.intersect(&claim2.rect) {
                continue 'a;
            }
        }
        return Some(claim);
    }

    None
}

fn read_input() -> Result<String, Error> {
    std::fs::read_to_string("../03.txt")
}

#[cfg(test)]
mod tests {
    use super::{parse_claims, solve_part1, solve_part2, Claim};

    const INPUT: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    fn parse_test_claims() -> Vec<Claim> {
        parse_claims(INPUT).unwrap()
    }

    #[test]
    fn test1() {
        let claims = parse_test_claims();
        assert_eq!(4, solve_part1(&claims));
    }

    #[test]
    fn test2() {
        let claims = parse_test_claims();
        assert_eq!(3, solve_part2(&claims).unwrap().id);
    }
}
