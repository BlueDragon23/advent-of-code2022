use std::collections::HashMap;

use itertools::Itertools;
use petgraph::{graph::DiGraph, stable_graph::NodeIndex, visit::DfsPostOrder};
use reformation::Reformation;

#[derive(Debug, Reformation, Clone)]
#[reformation()]
enum Command {
    #[reformation(r"\$ cd {}")]
    Cd(String),
    #[reformation(r"\$ ls")]
    Ls,
    #[reformation(r"dir {}")]
    Dir(String),
    #[reformation(r"{} {}")]
    File(u64, String),
}

#[derive(Debug, Clone)]
struct File {
    size: u64,
    name: String,
    is_dir: bool,
}

const TOTAL_SIZE: u64 = 70_000_000;
const REQUIRED_FREE_SIZE: u64 = 30_000_000;

fn main() -> color_eyre::Result<()> {
    let commands = parse_input(include_str!("../../input/day7.txt"))?;
    println!("Part 1: {}", solve_part1(&commands).unwrap());
    println!("Part 2: {}", solve_part2(&commands));
    Ok(())
}

fn parse_input(input: &str) -> color_eyre::Result<Vec<Command>> {
    input
        .lines()
        .map(|line| Ok(Command::parse(line)?))
        .collect()
}

fn build_graph(commands: &[Command]) -> (DiGraph<File, u64>, NodeIndex<u32>) {
    let mut graph: DiGraph<File, u64> = DiGraph::default();
    let mut current_node = graph.add_node(File {
        name: "/".to_owned(),
        size: 0,
        is_dir: true,
    });
    let root = current_node;
    for command in commands {
        match command {
            Command::Cd(path) => {
                match path.as_str() {
                    "/" => {
                        // do nothing
                    }
                    ".." => {
                        // back to parent
                        current_node = graph
                            .neighbors_undirected(current_node)
                            .find(|neighbor| graph.find_edge(*neighbor, current_node).is_some())
                            .unwrap();
                    }
                    name => {
                        let new_node = graph.add_node(File {
                            name: name.to_owned(),
                            size: 0,
                            is_dir: true,
                        });
                        graph.add_edge(current_node, new_node, 0);
                        current_node = new_node;
                    }
                }
            }
            Command::Ls => {
                // do nothing
            }
            Command::Dir(_) => {
                // add directories when we cd instead
            }
            Command::File(size, name) => {
                let new_node = graph.add_node(File {
                    name: name.to_owned(),
                    size: *size,
                    is_dir: false,
                });
                graph.add_edge(current_node, new_node, 0);
            }
        }
    }
    (graph, root)
}

fn solve_part1(commands: &[Command]) -> Option<u64> {
    let (graph, root) = build_graph(commands);
    // find size of each node
    let results = get_file_sizes(graph, root);
    Some(
        results
            .values()
            .filter(|&file| file.is_dir && file.size <= 100_000)
            .map(|file| file.size)
            .sum(),
    )
}

fn get_file_sizes(graph: DiGraph<File, u64>, root: NodeIndex) -> HashMap<NodeIndex, File> {
    let mut visitor = DfsPostOrder::new(&graph, root);
    let mut results: HashMap<NodeIndex, File> = HashMap::new();
    while let Some(node) = visitor.next(&graph) {
        let file = graph.node_weight(node).unwrap();
        if file.size == 0 {
            // directory
            let total_weight = graph
                .neighbors(node)
                .fold(0, |sum, n| (results.get(&n).unwrap()).size + sum);
            results.insert(
                node,
                File {
                    name: file.name.clone(),
                    size: total_weight,
                    is_dir: true,
                },
            );
        } else {
            // file
            results.insert(node, file.clone());
        }
    }
    results
}

fn solve_part2(commands: &[Command]) -> u64 {
    let (graph, root) = build_graph(commands);
    // find size of each node
    let results = get_file_sizes(graph, root);
    let consumed_space = results.values().find(|file| file.name == "/").unwrap().size;
    let desired_usage = TOTAL_SIZE - REQUIRED_FREE_SIZE;
    let required_to_delete = consumed_space - desired_usage;
    results.values().filter(|file| file.is_dir && file.size >= required_to_delete).map(|file| file.size).sorted().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parse_input(include_str!("../../input/day7.test.txt"))?;
        let result = solve_part1(&input).expect("Result should be found");
        assert_eq!(result, 95437);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parse_input(include_str!("../../input/day7.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 24933642);
        Ok(())
    }
}
