use itertools::Itertools;
use nom::Parser;
mod error;

use crate::error::AocError;
use crate::error::AocError::ValueError;
use nom;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;

fn parse_input(input: &str) -> Result<Vec<(u64, u64)>, AocError> {
    let mut parser = separated_list0(
        tag(","),
        separated_pair(
            complete::u64::<&str, ()>,
            tag("-"),
            complete::u64::<&str, ()>,
        ),
    );
    let result = parser.parse(input);

    Ok(result
        .map_err(|_| ValueError("Invalid input".to_string()))?
        .1)
}

pub fn process_part1(_input: &str) -> Result<String, AocError> {
    let ranges = parse_input(_input)?;

    let result = ranges
        .iter()
        .map(|(start, end)| {
            (*start..=*end)
                .filter_map(|n| {
                    let s = n.to_string();
                    let s_len = s.len();

                    if s_len % 2 != 0 {
                        None
                    } else if s[..s_len / 2] == s[s_len / 2..] {
                        Some(n)
                    } else {
                        None
                    }
                })
                .sum::<u64>()
        })
        .sum::<u64>();

    Ok(result.to_string())
}

pub fn process_part2(_input: &str) -> Result<String, AocError> {
    let ranges = parse_input(_input)?;

    let result = ranges
        .iter()
        .map(|(start, end)| {
            (*start..=*end)
                .filter_map(|n| {
                    let s = n.to_string();
                    let s_len = s.len();

                    let funny = (1..=s_len / 2).into_iter().any(|p_len| {
                        if s_len % p_len == 0 {
                            let p = &s[..p_len];
                            s.chars()
                                .chunks(p_len)
                                .into_iter()
                                .all(|chunk| chunk.collect::<String>() == p)
                        } else {
                            false
                        }
                    });

                    if funny {
                        Some(n)
                    } else {
                        None
                    }
                })
                .sum::<u64>()
        })
        .sum::<u64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() -> Result<(), AocError> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process_part1(input)?);
        Ok(())
    }

    #[test]
    fn test_process_part2() -> Result<(), AocError> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("4174379265", process_part2(input)?);
        Ok(())
    }
}

fn main() -> Result<(), AocError> {
    let input = include_str!("../day-02.txt");
    let result = process_part1(input);
    println!("Part 1: {}", result?);

    let result = process_part2(input);
    println!("Part 2: {}", result?);
    Ok(())
}
