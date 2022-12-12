use std::{cmp::Ordering, collections::HashMap};

use advent_of_code2022::{get_adjacent_positive_points, PositiveCoordinate};
use itertools::Itertools;

struct Input {
    start: PositiveCoordinate,
    end: PositiveCoordinate,
    grid: Vec<Vec<u32>>,
}

#[derive(Clone, Copy, Debug, Eq)]
struct Node {
    c: PositiveCoordinate,
    cost: u32,
    previous: Option<PositiveCoordinate>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

fn main() -> color_eyre::Result<()> {
    let input = include_str!("../../input/day12.txt");
    let input_processed = parse_input(input)?;
    println!("Part 1: {}", solve_part1(&input_processed));
    println!("Part 2: {}", solve_part2(&input_processed));
    Ok(())
}

fn parse_input(input: &str) -> color_eyre::Result<Input> {
    let mut start = PositiveCoordinate { row: 0, col: 0 };
    let mut end = PositiveCoordinate { row: 0, col: 0 };
    let grid = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        start = PositiveCoordinate { row, col };
                        0
                    } else if c == 'E' {
                        end = PositiveCoordinate { row, col };
                        25
                    } else {
                        (c as u32) - 'a' as u32
                    }
                })
                .collect_vec()
        })
        .collect();
    Ok(Input { start, end, grid })
}

fn solve_part1(input: &Input) -> u32 {
    find_shortest_path(input.start, input.end, &input.grid).unwrap()
}

fn find_shortest_path(
    start: PositiveCoordinate,
    end: PositiveCoordinate,
    grid: &Vec<Vec<u32>>,
) -> Option<u32> {
    let height = grid.len();
    let width = grid[0].len();
    let mut best_route: HashMap<PositiveCoordinate, Node> = HashMap::new();
    best_route.insert(
        start,
        Node {
            c: start,
            cost: 0,
            previous: None,
        },
    );
    let mut unvisited = vec![Node {
        c: start,
        cost: 0,
        previous: None,
    }];
    while let Some(index) = unvisited.iter().position_min() {
        let current = unvisited.remove(index);
        best_route.insert(current.c, current);
        get_adjacent_positive_points(current.c, 0, 0, height, width)
            .iter()
            .filter(|point| {
                valid_move(
                    grid[current.c.row][current.c.col],
                    grid[point.row][point.col],
                )
            })
            .for_each(|&point| {
                if !best_route.contains_key(&point) {
                    let node = Node {
                        c: point,
                        cost: current.cost + 1,
                        previous: Some(current.c),
                    };
                    if let Some((unvisited_pos, existing)) =
                        unvisited.iter().find_position(|n| n.c == point)
                    {
                        if existing.cost > current.cost + 1 {
                            unvisited.remove(unvisited_pos);
                            unvisited.push(node);
                        }
                    } else {
                        unvisited.push(node);
                    }
                }
            });
    }
    best_route.get(&end).map(|n| n.cost)
}

fn valid_move(current: u32, next: u32) -> bool {
    next <= (current + 1)
}

fn solve_part2(input: &Input) -> u32 {
    let height = input.grid.len();
    let width = input.grid[0].len();
    (0..height)
        .cartesian_product(0..width)
        .map(|(row, col)| PositiveCoordinate { row, col })
        .filter(|coord| input.grid[coord.row][coord.col] == 0)
        .flat_map(|start| find_shortest_path(start, input.end, &input.grid))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day12.test.txt");
        let input_processed = parse_input(input).unwrap();
        let result = solve_part1(&input_processed);
        dbg!(result);
        assert!(result == 31);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day12.test.txt");
        let input_processed = parse_input(input).unwrap();
        let result = solve_part2(&input_processed);
        dbg!(result);
        assert!(result == 29);
    }
}
