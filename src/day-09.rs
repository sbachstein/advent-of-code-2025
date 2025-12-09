use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::Parser;
use std::cmp::{max, min};
use std::collections::HashSet;

mod error;

use crate::error::AocError;
use crate::error::AocError::ValueError;

fn parse_input(input: &str) -> Result<Vec<(i64, i64)>, AocError> {
    let (_, red_tiles) = separated_list0(
        tag("\n"),
        separated_pair(i64::<&str, ()>, tag(","), i64::<&str, ()>),
    )
    .parse(input)
    .map_err(|_| ValueError("Invalid input".into()))?;

    Ok(red_tiles)
}

pub fn process_part1(_input: &str) -> Result<String, AocError> {
    let red_tiles = parse_input(_input)?;

    red_tiles
        .iter()
        .tuple_combinations()
        .map(|(tile1, tile2)| ((tile1.0 - tile2.0).abs() + 1) * ((tile1.1 - tile2.1).abs() + 1))
        .max()
        .map(|res| res.to_string())
        .ok_or(ValueError("No data".into()))
}

pub fn process_part2(_input: &str) -> Result<String, AocError> {
    let red_tiles = parse_input(_input)?;

    let edge_tiles = red_tiles.iter().circular_tuple_windows().fold(
        HashSet::new(),
        |mut edges, (tile1, tile2)| {
            if tile1.0 == tile2.0 {
                let line = (min(tile1.1, tile2.1)..=max(tile1.1, tile2.1)).map(|y| (tile1.0, y));
                edges.extend(line);
            } else if tile1.1 == tile2.1 {
                let line = (min(tile1.0, tile2.0)..=max(tile1.0, tile2.0)).map(|x| (x, tile1.1));
                edges.extend(line);
            }
            edges
        },
    );

    red_tiles
        .iter()
        .tuple_combinations()
        .map(|(tile1, tile2)| {
            let area = ((tile1.0 - tile2.0).abs() + 1) * ((tile1.1 - tile2.1).abs() + 1);
            (area, tile1, tile2)
        })
        .sorted()
        .rev()
        .find(|(_, &tile1, &tile2)| {
            let left = min(tile1.0, tile2.0);
            let right = max(tile1.0, tile2.0);
            let top = min(tile1.1, tile2.1);
            let bottom = max(tile1.1, tile2.1);

            !edge_tiles
                .iter()
                .any(|e| e.0 > left && e.0 < right && e.1 > top && e.1 < bottom)
        })
        .map(|res| res.0.to_string())
        .ok_or(ValueError("No data".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() -> Result<(), AocError> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!("50", process_part1(input)?);
        Ok(())
    }

    #[test]
    fn test_process_part2() -> Result<(), AocError> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!("24", process_part2(input)?);
        Ok(())
    }
}

fn main() -> Result<(), AocError> {
    let input = include_str!("../day-09.txt");
    let result = process_part1(input);
    println!("Part 1: {}", result?);

    let result = process_part2(input);
    println!("Part 2: {}", result?);
    Ok(())
}
