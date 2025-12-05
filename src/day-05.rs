extern crate core;

use nom::Parser;
use std::collections::HashSet;
mod error;

use crate::error::AocError;
use crate::error::AocError::ValueError;
use itertools::Itertools;
use nom::bytes::tag;
use nom::character::complete::u128;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::separated_pair;
use std::ops::RangeInclusive;

fn parse_input(input: &str) -> Result<(Vec<RangeInclusive<u128>>, Vec<u128>), AocError> {
    let (_, result) = separated_pair(
        separated_list1(
            tag("\n"),
            separated_pair(u128::<&str, ()>, tag("-"), u128::<&str, ()>)
                .map(|(start, end)| start..=end),
        ),
        tag("\n\n"),
        separated_list0(tag("\n"), u128::<&str, ()>),
    )
    .parse(input)
    .map_err(|_| ValueError("Invalid input".into()))?;

    Ok(result)
}

fn remove_indices_from_vec<T>(v: &mut Vec<T>, ixs: &HashSet<usize>) -> () {
    ixs.iter().sorted().rev().for_each(|&i| {
        v.remove(i);
    })
}

pub fn process_part1(_input: &str) -> Result<String, AocError> {
    let (ranges, ids) = parse_input(_input)?;

    let number_of_fresh_items = ids
        .iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count();

    Ok(number_of_fresh_items.to_string())
}

pub fn process_part2(_input: &str) -> Result<String, AocError> {
    let (ranges, _) = parse_input(_input)?;

    let disjoint_ranges =
        ranges
            .into_iter()
            .fold(Vec::<RangeInclusive<u128>>::new(), |mut acc, r| {
                let includes_start = acc
                    .iter()
                    .cloned()
                    .find_position(|acc_range| acc_range.contains(r.start()));

                let includes_end = acc
                    .iter()
                    .cloned()
                    .find_position(|acc_range| acc_range.contains(r.end()));

                let mut ranges_to_remove = acc
                    .iter()
                    .enumerate()
                    .filter_map(|(ix, acc_range)| {
                        if r.contains(acc_range.start())
                            & r.contains(acc_range.end())
                            & !(r == *acc_range)
                        {
                            Some(ix)
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<_>>();

                match (includes_start, includes_end) {
                    (Some((inc_start_ix, inc_start_range)), Some((inc_end_ix, inc_end_range))) => {
                        if inc_start_ix != inc_end_ix {
                            ranges_to_remove.insert(inc_start_ix);
                            ranges_to_remove.insert(inc_end_ix);
                            acc.push(*inc_start_range.start()..=*inc_end_range.end())
                        }
                    }
                    (Some((inc_start_ix, inc_start_range)), None) => {
                        ranges_to_remove.insert(inc_start_ix);
                        acc.push(*inc_start_range.start()..=*r.end())
                    }
                    (None, Some((inc_end_ix, inc_end_range))) => {
                        ranges_to_remove.insert(inc_end_ix);
                        acc.push(*r.start()..=*inc_end_range.end())
                    }
                    (None, None) => {
                        acc.push(r);
                    }
                };
                remove_indices_from_vec(&mut acc, &ranges_to_remove);
                acc
            });

    dbg!(&disjoint_ranges);

    let total_length = disjoint_ranges
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum::<u128>();

    Ok(total_length.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() -> Result<(), AocError> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        assert_eq!("3", process_part1(input)?);
        Ok(())
    }

    #[test]
    fn test_process_part2() -> Result<(), AocError> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        assert_eq!("14", process_part2(input)?);
        Ok(())
    }
}

#[test]
fn test_process_part2_test2() -> Result<(), AocError> {
    let input = "3-5
4-6
8-10
7-9
16-20
17-19
30-31
25-35

1
5
8
11
17
32
";
    assert_eq!("24", process_part2(input)?);
    Ok(())
}

#[test]
fn test_process_part2_test3() -> Result<(), AocError> {
    let input = "3-5
4-6
8-10
7-9
16-20
17-19
30-31
25-35
100-110
120-130
140-150
105-145
65-65
65-65
80-81
81-82

0
";
    assert_eq!("79", process_part2(input)?);
    Ok(())
}

fn main() -> Result<(), AocError> {
    let input = include_str!("../day-05.txt");
    let result = process_part1(input);
    println!("Part 1: {}", result?);

    let result = process_part2(input);
    println!("Part 2: {}", result?);
    Ok(())
}
