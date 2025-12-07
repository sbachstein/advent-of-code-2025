use itertools::{repeat_n, Itertools};
use std::collections::HashSet;

mod error;

use crate::error::AocError;
use crate::error::AocError::ValueError;

fn parse_input(input: &str) -> Result<(usize, usize, Vec<Vec<usize>>), AocError> {
    let first_line = input.lines().nth(0).ok_or(ValueError("No data".into()))?;

    let width = first_line.chars().count();

    let start_position = first_line
        .chars()
        .position(|c| c == 'S')
        .ok_or(ValueError("No start position".into()))?;

    let splitters = input
        .lines()
        .skip(1)
        .map(|line| line.chars().positions(|c| c == '^').collect())
        .collect();

    Ok((width, start_position, splitters))
}

pub fn process_part1(_input: &str) -> Result<String, AocError> {
    let (_, start_position, splitters) = parse_input(_input)?;

    let result = splitters
        .iter()
        .fold((HashSet::from([start_position]), 0_u64), |b, x| {
            let mut num_splits = b.1;
            let new_beams =
                b.0.iter()
                    .flat_map(|&beam_pos| {
                        if x.contains(&beam_pos) {
                            num_splits += 1;
                            vec![beam_pos - 1, beam_pos + 1]
                        } else {
                            vec![beam_pos]
                        }
                    })
                    .collect();

            (new_beams, num_splits)
        })
        .1;

    Ok(result.to_string())
}

pub fn process_part2(_input: &str) -> Result<String, AocError> {
    let (width, start_position, splitters) = parse_input(_input)?;

    let mut num_paths_to = repeat_n(0_u64, width).collect::<Vec<_>>();
    num_paths_to[start_position] = 1;

    let result = splitters
        .iter()
        .fold(num_paths_to, |acc, x| {
            let mut new_num_paths_to = repeat_n(0_u64, width).collect::<Vec<_>>();
            acc.iter().enumerate().for_each(|(pos, &num_paths)| {
                if num_paths > 0 {
                    if x.contains(&pos) {
                        new_num_paths_to[pos - 1] += num_paths;
                        new_num_paths_to[pos + 1] += num_paths;
                    } else {
                        new_num_paths_to[pos] += num_paths;
                    }
                }
            });
            new_num_paths_to
        })
        .iter()
        .sum::<u64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() -> Result<(), AocError> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!("21", process_part1(input)?);
        Ok(())
    }

    #[test]
    fn test_process_part2() -> Result<(), AocError> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!("40", process_part2(input)?);
        Ok(())
    }
}

fn main() -> Result<(), AocError> {
    let input = include_str!("../day-07.txt");
    let result = process_part1(input);
    println!("Part 1: {}", result?);

    let result = process_part2(input);
    println!("Part 2: {}", result?);
    Ok(())
}
