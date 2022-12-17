use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::{tag, take},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};
use petgraph::{
    prelude::UnGraph, stable_graph::NodeIndex, visit::IntoNodeReferences, Graph, Undirected,
};

struct Input<'a> {
    name: &'a str,
    flow: u32,
    connected: Vec<&'a str>,
}

fn main() -> color_eyre::Result<()> {
    let input = parse_input(include_str!("../../input/day16.txt"))?;
    let graph = build_graph(input);
    println!("Part 1: {}", solve_part1(&graph));
    println!("Part 2: {}", solve_part2(&graph));
    Ok(())
}

fn parse_name(input: &str) -> IResult<&str, &str> {
    preceded(tag("Valve "), take(2 as u32))(input)
}

fn parse_flow(input: &str) -> IResult<&str, u32> {
    preceded(tag(" has flow rate="), nom::character::complete::u32)(input)
}

fn parse_connected(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        tag("; tunnels lead to valves "),
        separated_list1(tag(", "), take(2 as u32)),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Input> {
    map(
        tuple((parse_name, parse_flow, parse_connected)),
        |(name, flow, connected)| Input {
            name,
            flow,
            connected,
        },
    )(input)
}

fn parse_input(input: &str) -> color_eyre::Result<Vec<Input>> {
    Ok(input
        .lines()
        .map(|line| parse_line(line).finish().unwrap().1)
        .collect())
}

fn build_graph(nodes: Vec<Input>) -> UnGraph<u32, u32> {
    // first add nodes
    let (g, map) = nodes.iter().fold(
        (
            Graph::<u32, u32, Undirected>::new_undirected(),
            HashMap::new(),
        ),
        |(mut g, mut map), input| {
            let node_index = g.add_node(input.flow);
            map.insert(input.name, node_index);
            (g, map)
        },
    );
    // now add edges. This avoids having to maybe add nodes
    nodes
        .iter()
        .fold((g, map), |(mut g, map), input| {
            let source_index = map.get(input.name).unwrap();
            input.connected.iter().for_each(|target| {
                let target_index = map.get(target).unwrap();
                g.update_edge(*source_index, *target_index, 1);
            });
            (g, map)
        })
        .0
}

fn solve_part1(input: &UnGraph<u32, u32>) -> u32 {
    let root = input.node_references().find(|n| *(n.1) == 0).unwrap().0;
    // do graph search things
    find_max_pressure(input, &root, &HashSet::new(), 30)
}

fn find_max_pressure(
    graph: &UnGraph<u32, u32>,
    current: &NodeIndex<u32>,
    open_valves: &HashSet<NodeIndex<u32>>,
    remaining_time: u32,
) -> u32 {
    if remaining_time == 0 {
        // count the last minute of flow
        return graph
            .node_references()
            .filter_map(|(index, weight)| {
                if open_valves.contains(&index) {
                    Some(weight)
                } else {
                    None
                }
            })
            .sum();
    }
    // at every node you have choices
    let mut results: Vec<u32> = vec![];
    // if valve is not open, open valve
    if !open_valves.contains(current) {
        let mut new_valves = open_valves.clone();
        new_valves.insert(*current);
    }
    // and move to adjacent valve
    results.iter().sum()
}

fn solve_part2(input: &UnGraph<u32, u32>) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parse_input(include_str!("../../input/day16.test.txt"))?;
        let graph = build_graph(input);
        let result = solve_part1(&graph);
        assert_eq!(result, 1651);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parse_input(include_str!("../../input/day16.test.txt"))?;
        let graph = build_graph(input);
        let result = solve_part2(&graph);
        assert_eq!(result, 1);
        Ok(())
    }
}
