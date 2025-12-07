use nom::Parser;
mod error;

use crate::error::AocError;
use crate::error::AocError::ValueError;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::u64;
use nom::multi::{many0, many1, many_m_n};
use nom::sequence::{delimited, pair, terminated};

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

fn parse_input_part1(input: &str) -> Result<(Vec<Vec<u64>>, Vec<Operation>), AocError> {
    let num_lines = input.lines().count();

    let (_, result) = pair(
        many_m_n(
            num_lines - 1,
            num_lines - 1,
            terminated(
                many1(delimited(many0(tag(" ")), u64::<&str, ()>, many0(tag(" ")))),
                tag("\n"),
            ),
        ),
        many1(delimited(many0(tag(" ")), anychar, many0(tag(" ")))),
    )
    .parse(input)
    .map_err(|_| ValueError("Invalid data".into()))?;

    let numbers = result.0;

    let operations = result
        .1
        .iter()
        .map(|op| match op {
            '+' => Ok(Operation::Add),
            '*' => Ok(Operation::Multiply),
            _ => Err(ValueError("Invalid operation".into())),
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((numbers, operations))
}

pub fn process_part1(_input: &str) -> Result<String, AocError> {
    let (numbers, operations) = parse_input_part1(_input)?;

    let result = numbers
        .into_iter()
        .reduce(|acc, new| {
            acc.into_iter()
                .zip(new.iter())
                .zip(operations.iter())
                .map(|((acc_v, new_v), op)| match op {
                    Operation::Add => acc_v + *new_v,
                    Operation::Multiply => acc_v * *new_v,
                })
                .collect::<Vec<_>>()
        })
        .ok_or(ValueError("Missing data".into()))?
        .iter()
        .sum::<u64>();
    Ok(result.to_string())
}

fn parse_input_part2(input: &str) -> Result<(Vec<Vec<u64>>, Vec<Operation>), AocError> {
    let num_lines = input.lines().count();
    let number_lines = input
        .lines()
        .take(num_lines - 1)
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    let operation_line = input.lines().rev().take(1).collect::<String>();

    let number_line_length = number_lines[0].len();
    let number_lines_transposed = (0..number_line_length)
        .map(|i| {
            number_lines
                .iter()
                .map(|line| {
                    line.chars()
                        .nth(i)
                        .ok_or(ValueError("Different length lines".into()))
                })
                .collect::<Result<String, _>>()
                .map(|s| s.trim().to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;

    let numbers = number_lines_transposed
        .iter()
        .fold(vec![vec![]], |mut acc, line| {
            match line.parse::<u64>() {
                Ok(n) => acc.last_mut().unwrap().push(n),
                Err(_) => acc.push(vec![]),
            }
            acc
        });

    let operations = operation_line
        .trim()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|op| match op {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(ValueError("Invalid operation".into())),
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((numbers, operations))
}

pub fn process_part2(_input: &str) -> Result<String, AocError> {
    let (numbers, operations) = parse_input_part2(_input)?;

    let result = numbers
        .into_iter()
        .zip(operations)
        .map(|(num_set, op)| {
            num_set
                .into_iter()
                .reduce(|acc, new| match op {
                    Operation::Add => acc + new,
                    Operation::Multiply => acc * new,
                })
                .ok_or(ValueError("Missing data".into()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let sum = result.iter().sum::<u64>();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() -> Result<(), AocError> {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        assert_eq!("4277556", process_part1(input)?);
        Ok(())
    }

    #[test]
    fn test_process_part2() -> Result<(), AocError> {
        let input = "123 328  51 640
 45 64  387 230
  6 98  215 314
*   +   *   +  ";
        assert_eq!("3263827", process_part2(input)?);
        Ok(())
    }
}

fn main() -> Result<(), AocError> {
    let input = include_str!("../day-06.txt");
    let result = process_part1(input);
    println!("Part 1: {}", result?);

    let result = process_part2(input);
    println!("Part 2: {}", result?);
    Ok(())
}
