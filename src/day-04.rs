mod error;

use crate::error::AocError;
use glam::IVec2;
use petgraph::Graph;
use std::collections::{HashMap, HashSet};

const ABOVE: IVec2 = IVec2::new(0, -1);
const TOP_LEFT: IVec2 = IVec2::new(-1, -1);
const TOP_RIGHT: IVec2 = IVec2::new(1, -1);
const BELOW: IVec2 = IVec2::new(0, 1);
const BOTTOM_LEFT: IVec2 = IVec2::new(-1, 1);
const BOTTOM_RIGHT: IVec2 = IVec2::new(1, 1);

const LEFT: IVec2 = IVec2::new(-1, 0);
const RIGHT: IVec2 = IVec2::new(1, 0);

const SURROUNDINGS: [IVec2; 8] = [
    ABOVE,
    BELOW,
    LEFT,
    RIGHT,
    TOP_LEFT,
    TOP_RIGHT,
    BOTTOM_LEFT,
    BOTTOM_RIGHT,
];

fn parse_input(input: &str) -> Graph<(), ()> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '@' => Some(IVec2::new(x as i32, y as i32)),
                _ => None,
            })
        })
        .collect::<HashSet<_>>();

    let mut graph = petgraph::Graph::<(), ()>::new();
    let mut index_mapping = HashMap::new();

    for coordinates in &map {
        let nodeindex = graph.add_node(());
        index_mapping.insert(coordinates, nodeindex);
    }

    for coordinates in &map {
        SURROUNDINGS
            .iter()
            .map(|&direction| coordinates + direction)
            .filter(|neighbor| map.contains(neighbor))
            .for_each(|neighbor| {
                match (index_mapping.get(coordinates), index_mapping.get(&neighbor)) {
                    (Some(index), Some(neighbor)) => {
                        graph.add_edge(index.clone(), neighbor.clone(), ());
                    }
                    _ => {}
                }
            })
    }
    graph
}

pub fn process_part1(_input: &str) -> Result<String, AocError> {
    let graph = parse_input(_input);

    let result = graph
        .node_indices()
        .filter(|ix| graph.neighbors(*ix).count() < 4)
        .count();

    Ok(result.to_string())
}

pub fn process_part2(_input: &str) -> Result<String, AocError> {
    let mut graph = parse_input(_input);

    let initial_number_of_rolls = graph.node_count();

    loop {
        let free_rolls = graph
            .node_indices()
            .filter(|ix| graph.neighbors(*ix).count() < 4)
            .collect::<Vec<_>>();

        if free_rolls.is_empty() {
            break;
        }

        graph.retain_nodes(|_, ix| !free_rolls.contains(&ix));
    }

    let result = initial_number_of_rolls - graph.node_count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() -> Result<(), AocError> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("13", process_part1(input)?);
        Ok(())
    }

    #[test]
    fn test_process_part2() -> Result<(), AocError> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("43", process_part2(input)?);
        Ok(())
    }
}

fn main() -> Result<(), AocError> {
    let input = include_str!("../day-04.txt");
    let result = process_part1(input);
    println!("Part 1: {}", result?);

    let result = process_part2(input);
    println!("Part 2: {}", result?);
    Ok(())
}
