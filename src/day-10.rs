use good_lp;
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{one_of, u32, usize};
use nom::multi::{many0, separated_list0, separated_list1};
use nom::sequence::delimited;
use nom::Parser;

mod error;

use crate::error::AocError;
use crate::error::AocError::ValueError;

fn parse_input(input: &str) -> Result<Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<u32>)>, AocError> {
    let (_, result) = separated_list1(
        tag::<&str, &str, ()>("\n"),
        (
            delimited(tag("["), many0(one_of(".#").map(|c| c == '#')), tag("]")),
            delimited(
                tag(" "),
                separated_list0(
                    tag(" "),
                    delimited(tag("("), separated_list0(tag(","), usize), tag(")")),
                ),
                tag(" "),
            ),
            delimited(tag("{"), separated_list0(tag(","), u32), tag("}")),
        ),
    )
    .parse(input)
    .map_err(|_| ValueError("Invalid input".into()))?;

    Ok(result)
}

pub fn process_part1(_input: &str) -> Result<String, AocError> {
    let machines = parse_input(_input)?;

    let result: usize = machines
        .iter()
        .map(|(indicators, buttons, _)| {
            for num_buttons_pressed in 1..=buttons.len() {
                let target_reached = buttons
                    .iter()
                    .cloned()
                    .combinations(num_buttons_pressed)
                    .any(|buttons_pressed| {
                        *indicators
                            == buttons_pressed.iter().flatten().fold(
                                vec![false; indicators.len()],
                                |mut acc, &x| {
                                    acc[x] = !acc[x];
                                    acc
                                },
                            )
                    });

                if target_reached {
                    return Ok(num_buttons_pressed);
                }
            }

            return Err(ValueError("Target not reachable".into()));
        })
        .sum::<Result<usize, AocError>>()?;

    Ok(result.to_string())
}

pub fn process_part2(_input: &str) -> Result<String, AocError> {
    let machines = parse_input(_input)?;

    let result: usize = machines
        .iter()
        .map(|(_, buttons, joltage)| {
            let mut vars = variables!();
            let x = (0..buttons.len())
                .map(|_| vars.add(variable().integer().min(0)))
                .collect::<Vec<_>>();
            let objective: Expression = x.iter().sum();

            let constraints = joltage
                .into_iter()
                .enumerate()
                .map(|(joltage_index, &joltage_val)| {
                    constraint!(
                        x.iter()
                            .enumerate()
                            .filter_map(|(button_index, var)| {
                                buttons
                                    .get(button_index)
                                    .unwrap()
                                    .contains(&joltage_index)
                                    .then_some(var)
                            })
                            .sum::<Expression>()
                            == joltage_val
                    )
                })
                .collect::<Vec<_>>();

            let solution = constraints
                .into_iter()
                .fold(
                    vars.minimise(objective).using(default_solver),
                    |sol, constraint| sol.with(constraint),
                )
                .solve()
                .map_err(|_| ValueError("No solution found".into()))?;

            Ok(x.into_iter()
                .map(|var| solution.value(var) as usize)
                .sum::<usize>())
        })
        .sum::<Result<usize, AocError>>()?;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() -> Result<(), AocError> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!("7", process_part1(input)?);
        Ok(())
    }

    #[test]
    fn test_process_part2() -> Result<(), AocError> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!("33", process_part2(input)?);
        Ok(())
    }
}

fn main() -> Result<(), AocError> {
    let input = include_str!("../day-10.txt");
    let result = process_part1(input);
    println!("Part 1: {}", result?);

    let result = process_part2(input);
    println!("Part 2: {}", result?);
    Ok(())
}
