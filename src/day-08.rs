use glam::I64Vec3;
use itertools::Itertools;
use petgraph::algo::tarjan_scc;
use petgraph::graphmap::UnGraphMap;
use petgraph::prelude::GraphMap;
use petgraph::{Graph, Undirected};

mod error;

use crate::error::AocError;
use crate::error::AocError::ValueError;

fn parse_input(input: &str) -> Result<Vec<I64Vec3>, AocError> {
    input
        .lines()
        .map(|line| {
            let coords: Result<[i64; 3], _> = line
                .split(",")
                .map(|n| {
                    n.parse::<i64>()
                        .map_err(|_| ValueError("Invalid number".into()))
                })
                .collect::<Result<Vec<_>, _>>()
                .and_then(|res| {
                    res.try_into()
                        .map_err(|_| ValueError("Missing x, y or z coordinate".into()))
                });
            coords.map(|c| I64Vec3::from(c))
        })
        .collect()
}

pub fn process_part1(_input: &str, num_connections: usize) -> Result<String, AocError> {
    let coordinates = parse_input(_input)?;

    let connections = coordinates
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((ix1, c1), (ix2, c2))| (c1.distance_squared(*c2), (ix1, ix2)))
        .sorted()
        .take(num_connections)
        .map(|(_, boxes)| boxes)
        .collect::<Vec<_>>();

    let graph: Graph<(), (), Undirected, usize> = Graph::from_edges(
        connections
            .into_iter()
            .chain((0..coordinates.len()).map(|ix| (ix, ix))),
    );

    let components = tarjan_scc(&graph);
    let component_sizes_multiplied = components
        .iter()
        .map(|component| component.len() as u128)
        .sorted()
        .rev()
        .take(3)
        .product::<u128>();

    Ok(component_sizes_multiplied.to_string())
}

pub fn process_part2(_input: &str) -> Result<String, AocError> {
    let coordinates = parse_input(_input)?;

    let mut graph: UnGraphMap<usize, ()> =
        GraphMap::from_edges((0..coordinates.len()).map(|ix| (ix, ix)));

    let last_junction_boxes = coordinates
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((ix1, c1), (ix2, c2))| (c1.distance_squared(*c2), (ix1, ix2)))
        .sorted()
        .map(|(_, boxes)| boxes)
        .find(|(ix1, ix2)| {
            graph.add_edge(*ix1, *ix2, ());
            tarjan_scc(&graph).len() == 1
        })
        .ok_or(ValueError("No coordinates".into()))?;

    let result = coordinates.get(last_junction_boxes.0).unwrap().x
        * coordinates.get(last_junction_boxes.1).unwrap().x;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() -> Result<(), AocError> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!("40", process_part1(input, 10)?);
        Ok(())
    }

    #[test]
    fn test_process_part2() -> Result<(), AocError> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!("25272", process_part2(input)?);
        Ok(())
    }
}

fn main() -> Result<(), AocError> {
    let input = include_str!("../day-08.txt");
    let result = process_part1(input, 1000);
    println!("Part 1: {}", result?);

    let result = process_part2(input);
    println!("Part 2: {}", result?);
    Ok(())
}
