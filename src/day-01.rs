mod error;

use crate::error::AocError;
use crate::error::AocError::ValueError;
use crate::Turn::{Left, Right};
use std::str::FromStr;

#[derive(Debug)]
enum Turn {
    Left(i32),
    Right(i32),
}

impl FromStr for Turn {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("L") {
            Ok(Left(s[1..].parse()?))
        } else if s.starts_with("R") {
            Ok(Right(s[1..].parse()?))
        } else {
            Err(ValueError("Invalid turn".into()))
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Turn>, AocError> {
    input.lines().map(|l| l.parse()).collect()
}

pub fn process_part1(_input: &str) -> Result<String, AocError> {
    let turns = parse_input(_input)?;

    let result = turns
        .iter()
        .scan(50_i32, |b, x| {
            let next_pos = match x {
                Left(turn) => *b - turn,
                Right(turn) => *b + turn,
            }
            .rem_euclid(100);

            *b = next_pos;
            Some(next_pos)
        })
        .filter(|p| *p == 0)
        .count();

    Ok(result.to_string())
}

pub fn process_part2(_input: &str) -> Result<String, AocError> {
    let turns = parse_input(_input)?;

    let result = turns
        .iter()
        .scan(50_i32, |pos, x| {
            let next_virtual_pos = match x {
                Left(turn) => *pos - turn,
                Right(turn) => *pos + turn,
            };

            let mut clicks = match next_virtual_pos {
                ..=0 => 1 + (-next_virtual_pos).div_euclid(100),
                100.. => next_virtual_pos.div_euclid(100),
                _ => 0,
            };

            if *pos == 0 && next_virtual_pos < 0 {
                clicks -= 1;
            }

            let next_pos = next_virtual_pos.rem_euclid(100);
            *pos = next_pos;
            Some(clicks)
        })
        .sum::<i32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() -> Result<(), AocError> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("3", process_part1(input)?);
        Ok(())
    }

    #[test]
    fn test_process_part2() -> Result<(), AocError> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("6", process_part2(input)?);
        Ok(())
    }
}

fn main() -> Result<(), AocError> {
    let input = include_str!("../day-01.txt");
    let result = process_part1(input);
    println!("Part 1: {}", result?);

    let result = process_part2(input);
    println!("Part 2: {}", result?);
    Ok(())
}
