mod error;

use crate::error::AocError;
use crate::error::AocError::ValueError;

fn parse_input(input: &str) -> Result<Vec<Vec<u64>>, AocError> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|n| n as u64)
                        .ok_or(ValueError("Invalid character".to_string()))
                })
                .collect()
        })
        .collect::<Result<Vec<Vec<u64>>, AocError>>()
}

fn get_max_value_and_first_index(slice: &[u64]) -> Option<(usize, u64)> {
    slice
        .iter()
        .enumerate()
        .fold(None, |acc, (index, &val)| match acc {
            None => Some((index, val)),
            Some((_, max_val)) => {
                if val > max_val {
                    Some((index, val))
                } else {
                    acc
                }
            }
        })
}

pub fn process_part1(_input: &str) -> Result<String, AocError> {
    let lines = parse_input(_input)?;

    let result = lines
        .iter()
        .map(|line| {
            let (max_10_pos, max_10) = get_max_value_and_first_index(&line[..line.len() - 1])
                .ok_or(ValueError("Invalid line".to_string()))?;

            let (_, max_1) = get_max_value_and_first_index(&line[max_10_pos + 1..])
                .ok_or(ValueError("Invalid line".to_string()))?;

            let max = 10 * max_10 + max_1;
            Ok(max)
        })
        .sum::<Result<u64, AocError>>()?;

    Ok(result.to_string())
}

pub fn process_part2(_input: &str) -> Result<String, AocError> {
    let lines = parse_input(_input)?;

    let result = lines
        .iter()
        .map(|line| {
            let result = (0..=11).rev().fold(Ok((0usize, 0u64)), |acc, n| {
                let (current_pos, current_value) = acc?;

                let (pos, value) =
                    get_max_value_and_first_index(&line[current_pos..line.len() - n])
                        .ok_or(ValueError("Invalid line".to_string()))?;

                Ok((pos + current_pos + 1, 10 * current_value + value))
            });

            result.map(|res| res.1)
        })
        .sum::<Result<u64, AocError>>()?;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() -> Result<(), AocError> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("357", process_part1(input)?);
        Ok(())
    }

    #[test]
    fn test_process_part2() -> Result<(), AocError> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("3121910778619", process_part2(input)?);
        Ok(())
    }
}

fn main() -> Result<(), AocError> {
    let input = include_str!("../day-03.txt");
    let result = process_part1(input);
    println!("Part 1: {}", result?);

    let result = process_part2(input);
    println!("Part 2: {}", result?);
    Ok(())
}
