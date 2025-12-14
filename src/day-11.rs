use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::Parser;
use petgraph::algo::{all_simple_paths, is_cyclic_directed};
use petgraph::graphmap::DiGraphMap;
use petgraph::visit::Topo;
use std::collections::HashMap;
use std::hash::RandomState;

mod error;

use crate::error::AocError;
use crate::error::AocError::ValueError;

fn parse_input(input: &str) -> Result<HashMap<&str, Vec<&str>>, AocError> {
    let (_, connections) = separated_list0(
        tag("\n"),
        separated_pair(
            alpha1::<&str, ()>,
            tag(": "),
            separated_list0(tag(" "), alpha1),
        ),
    )
    .parse(input)
    .map_err(|_| ValueError("Invalid input".into()))?;

    Ok(connections.into_iter().collect())
}

pub fn process_part1(_input: &str) -> Result<String, AocError> {
    let connections = parse_input(_input)?;

    let mut graph: DiGraphMap<&str, ()> = DiGraphMap::new();

    connections.into_iter().for_each(|(machine, outputs)| {
        outputs.into_iter().for_each(|output| {
            graph.add_edge(machine, output, ());
        })
    });

    let number_of_paths = all_simple_paths::<Vec<_>, &DiGraphMap<&str, ()>, RandomState>(
        &graph, "you", "out", 0, None,
    )
    .count();

    Ok(number_of_paths.to_string())
}

fn number_of_paths_between(graph: &DiGraphMap<&str, ()>, start: &str, end: &str) -> usize {
    let mut path_counts = HashMap::new();
    path_counts.insert(start, 1);

    let mut topo = Topo::new(graph);

    while let Some(node) = topo.next(graph) {
        if let Some(&count) = path_counts.get(&node) {
            for neighbor in graph.neighbors(node) {
                *path_counts.entry(neighbor).or_insert(0) += count;
            }
        }
    }

    *path_counts.get(&end).unwrap_or(&0)
}

pub fn process_part2(_input: &str) -> Result<String, AocError> {
    let connections = parse_input(_input)?;

    let mut graph: DiGraphMap<&str, ()> = DiGraphMap::new();

    connections.into_iter().for_each(|(machine, outputs)| {
        outputs.into_iter().for_each(|output| {
            graph.add_edge(machine, output, ());
        })
    });

    if is_cyclic_directed(&graph) {
        return Err(ValueError("Cycle detected".into()));
    }

    let srv_dac = number_of_paths_between(&graph, "srv", "dac");
    let srv_fft = number_of_paths_between(&graph, "svr", "fft");
    let dac_fft = number_of_paths_between(&graph, "dac", "fft");
    let fft_dac = number_of_paths_between(&graph, "fft", "dac");
    let dac_out = number_of_paths_between(&graph, "dac", "out");
    let fft_out = number_of_paths_between(&graph, "fft", "out");

    let number_of_paths = srv_dac * dac_fft * fft_out + srv_fft * fft_dac * dac_out;

    Ok(number_of_paths.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() -> Result<(), AocError> {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!("5", process_part1(input)?);
        Ok(())
    }

    #[test]
    fn test_process_part2() -> Result<(), AocError> {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!("2", process_part2(input)?);
        Ok(())
    }
}

fn main() -> Result<(), AocError> {
    let input = include_str!("../day-11.txt");
    let result = process_part1(input);
    println!("Part 1: {}", result?);

    let result = process_part2(input);
    println!("Part 2: {}", result?);
    Ok(())
}
