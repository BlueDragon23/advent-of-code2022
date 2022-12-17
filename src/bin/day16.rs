use core::prelude::v1;
use std::{
    cell::RefCell,
    collections::{hash_map::DefaultHasher, BTreeSet, HashMap},
    hash::{Hash, Hasher},
};

use itertools::Itertools;
use nom::{
    branch::alt,
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

// The second field is a hash of the open valve vec
type State<'a> = (NodeIndex<u32>, u64, u32);
type State2<'a> = (NodeIndex<u32>, NodeIndex<u32>, u64, u32);

#[derive(PartialEq, Eq, Debug)]
enum Decision {
    Valve(NodeIndex<u32>),
    Move(NodeIndex<u32>),
}

fn main() -> color_eyre::Result<()> {
    let input = parse_input(include_str!("../../input/day16.txt"))?;
    let (root, graph) = build_graph(input);
    println!("Part 1: {}", solve_part1(&root, &graph));
    println!("Part 2: {}", solve_part2(&root, &graph));
    Ok(())
}

fn parse_name(input: &str) -> IResult<&str, &str> {
    preceded(tag("Valve "), take(2u32))(input)
}

fn parse_flow(input: &str) -> IResult<&str, u32> {
    preceded(tag(" has flow rate="), nom::character::complete::u32)(input)
}

fn parse_connected(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), take(2u32)),
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

fn build_graph(nodes: Vec<Input>) -> (NodeIndex<u32>, UnGraph<u32, u32>) {
    // first add nodes
    let mut root = None;
    let (g, map) = nodes.iter().fold(
        (
            Graph::<u32, u32, Undirected>::new_undirected(),
            HashMap::new(),
        ),
        |(mut g, mut map), input| {
            let node_index = g.add_node(input.flow);
            if input.name == "AA" {
                root = Some(node_index);
            }
            map.insert(input.name, node_index);
            (g, map)
        },
    );
    // now add edges. This avoids having to maybe add nodes
    (
        root.unwrap(),
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
            .0,
    )
}

fn solve_part1(root: &NodeIndex<u32>, input: &UnGraph<u32, u32>) -> u32 {
    // do graph search things
    find_max_pressure(
        input,
        root,
        &BTreeSet::new(),
        1,
        &RefCell::new(HashMap::new()),
    )
    .unwrap()
}

fn hash_valves(valves: &BTreeSet<NodeIndex<u32>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    valves.hash(&mut hasher);
    hasher.finish()
}

fn get_flow(graph: &UnGraph<u32, u32>, open_valves: &BTreeSet<NodeIndex<u32>>) -> u32 {
    graph
        .node_references()
        .filter_map(|(index, weight)| {
            if open_valves.contains(&index) {
                Some(weight)
            } else {
                None
            }
        })
        .sum()
}

fn print_node_name_test(index: &NodeIndex<u32>) -> String {
    match index.index() {
        0 => "AA",
        1 => "BB",
        2 => "CC",
        3 => "DD",
        4 => "EE",
        5 => "FF",
        6 => "GG",
        7 => "HH",
        8 => "II",
        9 => "JJ",
        x => {
            dbg!(x);
            "ZZ"
        }
    }
    .to_owned()
}

fn find_max_pressure(
    graph: &UnGraph<u32, u32>,
    current: &NodeIndex<u32>,
    open_valves: &BTreeSet<NodeIndex<u32>>,
    time_passed: u32,
    memoising: &RefCell<HashMap<State, u32>>,
) -> Option<u32> {
    // println!(
    //     "minute: {:?}, current: {:?}, open: {:?}",
    //     time_passed,
    //     print_node_name_test(current),
    //     open_valves.iter().map(print_node_name_test).collect_vec()
    // );
    // memoisation
    if let Some((_, value)) =
        memoising
            .borrow()
            .get_key_value(&(*current, hash_valves(open_valves), time_passed))
    {
        // println!("Returning cached value {}", value);
        return Some(*value);
    }

    // calculation
    let flow_this_minute = get_flow(graph, open_valves);
    // println!("Currently flowing at {}/min", flow_this_minute);
    if time_passed == 30 {
        // println!(
        //     "Caching value {} at minute {}",
        //     flow_this_minute, time_passed
        // );
        memoising.borrow_mut().insert(
            (*current, hash_valves(open_valves), time_passed),
            flow_this_minute,
        );
        // count the last minute of flow
        return Some(flow_this_minute);
    }
    // at every node you have choices
    let mut results: Vec<(State, u32)> = vec![];
    // if valve is not open, open valve
    if !open_valves.contains(current) && *graph.node_weight(*current)? > 0 {
        // try opening this one then moving
        let mut new_valves = open_valves.clone();
        new_valves.insert(*current);
        let child = find_max_pressure(graph, current, &new_valves, time_passed + 1, memoising)?;
        results.push((
            (*current, hash_valves(open_valves), time_passed),
            flow_this_minute + child,
        ));
    }
    // try just moving to adjacent valve
    results.extend(
        graph
            .neighbors(*current)
            .map(|neighbour| {
                (
                    (neighbour, hash_valves(open_valves), time_passed + 1),
                    find_max_pressure(graph, &neighbour, open_valves, time_passed + 1, memoising),
                )
            })
            .map(|(state, child)| (state, flow_this_minute + child.unwrap())),
    );
    let actual_result = results.into_iter().max_by_key(|(_, value)| *value);
    // println!(
    //     "Caching value {} at node {} at minute {}",
    //     actual_result?.1,
    //     print_node_name_test(&actual_result?.0 .0),
    //     time_passed
    // );
    memoising.borrow_mut().insert(
        (*current, hash_valves(open_valves), time_passed),
        actual_result?.1,
    );
    Some(actual_result.unwrap().1)
}

fn solve_part2(root: &NodeIndex<u32>, input: &UnGraph<u32, u32>) -> u32 {
    // do graph search things
    find_max_pressure_2(
        input,
        root,
        root,
        &BTreeSet::new(),
        // 4 minutes have passed
        5,
        &RefCell::new(HashMap::new()),
    )
    .unwrap()
}

fn find_max_pressure_2(
    graph: &UnGraph<u32, u32>,
    current: &NodeIndex<u32>,
    current_elephant: &NodeIndex<u32>,
    open_valves: &BTreeSet<NodeIndex<u32>>,
    time_passed: u32,
    memoising: &RefCell<HashMap<State2, u32>>,
) -> Option<u32> {
    // println!(
    //     "minute: {:?}, current: {:?}, open: {:?}",
    //     time_passed,
    //     print_node_name_test(current),
    //     open_valves.iter().map(print_node_name_test).collect_vec()
    // );
    // memoisation
    if let Some((_, value)) = memoising.borrow().get_key_value(&(
        *current,
        *current_elephant,
        hash_valves(open_valves),
        time_passed,
    )) {
        // println!("Returning cached value {}", value);
        return Some(*value);
    }

    // calculation
    let flow_this_minute = get_flow(graph, open_valves);
    // println!("Currently flowing at {}/min", flow_this_minute);
    if time_passed == 30 {
        // println!(
        //     "Caching value {} at minute {}",
        //     flow_this_minute, time_passed
        // );
        memoising.borrow_mut().insert(
            (
                *current,
                *current_elephant,
                hash_valves(open_valves),
                time_passed,
            ),
            flow_this_minute,
        );
        // count the last minute of flow
        return Some(flow_this_minute);
    }
    // at every node you have choices
    let choices = get_choices(graph, current, open_valves);
    let temp = get_choices(graph, current_elephant, open_valves);
    let elephant_choices = temp
        .iter()
        .filter(|choice| match choice {
            // make sure they don't open the same valve
            v @ Decision::Valve(_) => !choices.contains(v),
            _ => true,
        })
        .collect_vec();

    let best_result = choices
        .iter()
        .cartesian_product(elephant_choices)
        .flat_map(|(c1, c2)| match (c1, c2) {
            (Decision::Valve(v1), Decision::Valve(v2)) => {
                let mut new_valves = open_valves.clone();
                new_valves.insert(*v1);
                new_valves.insert(*v2);
                find_max_pressure_2(
                    graph,
                    current,
                    current_elephant,
                    &new_valves,
                    time_passed + 1,
                    memoising,
                )
            }
            (Decision::Valve(v), Decision::Move(p)) => {
                let mut new_valves = open_valves.clone();
                new_valves.insert(*v);
                find_max_pressure_2(graph, current, p, &new_valves, time_passed + 1, memoising)
            }
            (Decision::Move(p), Decision::Valve(v)) => {
                let mut new_valves = open_valves.clone();
                new_valves.insert(*v);
                find_max_pressure_2(
                    graph,
                    p,
                    current_elephant,
                    &new_valves,
                    time_passed + 1,
                    memoising,
                )
            }
            (Decision::Move(p1), Decision::Move(p2)) => {
                find_max_pressure_2(graph, p1, p2, open_valves, time_passed + 1, memoising)
            }
        })
        .map(|child| child + flow_this_minute)
        .max()?;
    memoising.borrow_mut().insert(
        (
            *current,
            *current_elephant,
            hash_valves(open_valves),
            time_passed,
        ),
        best_result,
    );
    Some(best_result)
}

fn get_choices(
    graph: &UnGraph<u32, u32>,
    current: &NodeIndex<u32>,
    open_valves: &BTreeSet<NodeIndex<u32>>,
) -> Vec<Decision> {
    let mut decisions = vec![];
    // if valve is not open, open valve
    if !open_valves.contains(current) && *graph.node_weight(*current).unwrap() > 0 {
        // try opening this one then moving
        decisions.push(Decision::Valve(*current));
    }
    // try just moving to adjacent valve
    decisions.extend(graph.neighbors(*current).map(Decision::Move));
    decisions
}

#[cfg(test)]
mod tests {
    use super::*;

    // expected behaviour
    // 00 AA []
    // 01 DD []
    // 02 DD [DD]
    // 03 CC [DD]
    // 04 BB [DD]
    // 05 BB [DD, BB]
    // 06 AA [DD, BB]
    // 07 II [DD, BB]
    // 08 JJ [DD, BB]
    // 09 JJ [DD, BB, JJ]
    // 10 II [DD, BB, JJ]
    // 11 AA [DD, BB, JJ]
    // 12 DD [DD, BB, JJ]
    // 13 EE [DD, BB, JJ]
    // 14 FF [DD, BB, JJ]
    // 15 GG [DD, BB, JJ]
    // 16 HH [DD, BB, JJ]
    // 17 HH [DD, BB, JJ, HH]
    // 18 GG [DD, BB, JJ, HH]
    // 19 FF [DD, BB, JJ, HH]
    // 20 EE [DD, BB, JJ, HH]
    // 21 EE [DD, BB, JJ, HH, EE]
    // 22 DD [DD, BB, JJ, HH, EE]
    // 23 CC [DD, BB, JJ, HH, EE]
    // 24 CC [DD, BB, JJ, HH, EE, CC]
    // 25 CC [DD, BB, JJ, HH, EE, CC]
    // 26 CC [DD, BB, JJ, HH, EE, CC]
    // 27 CC [DD, BB, JJ, HH, EE, CC]
    // 28 CC [DD, BB, JJ, HH, EE, CC]
    // 29 CC [DD, BB, JJ, HH, EE, CC]
    // 30 CC [DD, BB, JJ, HH, EE, CC]
    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parse_input(include_str!("../../input/day16.test.txt"))?;
        let (root, graph) = build_graph(input);
        let result = solve_part1(&root, &graph);
        assert_eq!(result, 1651);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parse_input(include_str!("../../input/day16.test.txt"))?;
        let (root, graph) = build_graph(input);
        let result = solve_part2(&root, &graph);
        assert_eq!(result, 1707);
        Ok(())
    }
}
