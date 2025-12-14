use nom::bytes::complete::tag;
use nom::character::complete::{one_of, u64, usize};
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, separated_pair};
use nom::Parser;

mod error;

use crate::error::AocError;
use crate::error::AocError::ValueError;

fn parse_input(
    input: &str,
) -> Result<(Vec<Vec<Vec<bool>>>, Vec<((usize, usize), Vec<usize>)>), AocError> {
    let (_, result) = separated_pair(
        separated_list1(
            tag("\n\n"),
            preceded(
                (u64::<&str, ()>, tag(":\n")),
                separated_list1(tag("\n"), many1(one_of("#.").map(|c| c == '#'))),
            ),
        ),
        tag("\n\n"),
        separated_list1(
            tag("\n"),
            separated_pair(
                separated_pair(usize, tag("x"), usize),
                tag(": "),
                separated_list1(tag(" "), usize),
            ),
        ),
    )
    .parse(input)
    .map_err(|_| ValueError("Invalid input".into()))?;

    Ok(result)
}

pub fn process_part1(_input: &str) -> Result<String, AocError> {
    let (presents, regions) = parse_input(_input)?;

    let present_sizes = presents
        .iter()
        .map(|present| present.iter().flatten().filter(|&&v| v).count())
        .collect::<Vec<usize>>();

    let result = regions
        .iter()
        .filter(|((width, height), present_counts)| {
            width * height
                >= present_counts
                    .iter()
                    .zip(present_sizes.iter())
                    .map(|(count, size)| count * size)
                    .sum()
        })
        .count();

    Ok(result.to_string())
}

pub fn process_part2(_input: &str) -> Result<String, AocError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() -> Result<(), AocError> {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        assert_eq!("2", process_part1(input)?);
        Ok(())
    }
}

fn main() -> Result<(), AocError> {
    let input = include_str!("../day-12.txt");
    let result = process_part1(input);
    println!("Part 1: {}", result?);

    Ok(())
}
