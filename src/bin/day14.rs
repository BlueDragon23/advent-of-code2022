use std::{
    cmp::{max, min},
    collections::HashMap,
    iter::repeat,
};

use advent_of_code2022::Coordinate;
use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list1, sequence::separated_pair,
    Finish, IResult,
};

type Input = Vec<Coordinate<u32>>;

#[derive(Debug)]
enum Contents {
    Rock,
    Sand,
}

const SAND_ORIGIN: Coordinate<u32> = Coordinate { row: 0, col: 500 };

fn main() -> color_eyre::Result<()> {
    let input = parse_input(include_str!("../../input/day14.txt"))?;
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
    Ok(())
}

fn parse_coord(input: &str) -> IResult<&str, Coordinate<u32>> {
    map(
        separated_pair(
            nom::character::complete::u32,
            tag(","),
            nom::character::complete::u32,
        ),
        // they're in column first order for some reason
        |(col, row)| Coordinate { row, col },
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Coordinate<u32>>> {
    separated_list1(tag(" -> "), parse_coord)(input)
}

fn parse_input(input: &str) -> color_eyre::Result<Vec<Input>> {
    Ok(input
        .lines()
        .map(|line| parse_line(line).finish().unwrap().1)
        .collect())
}

fn iterate_points(start: &Coordinate<u32>, end: &Coordinate<u32>) -> Vec<Coordinate<u32>> {
    if start.row == end.row {
        (min(start.col, end.col)..=(max(start.col, end.col)))
            .zip(repeat(start.row))
            .map(|(col, row)| Coordinate { row, col })
            .collect_vec()
    } else {
        (min(start.row, end.row)..=(max(start.row, end.row)))
            .zip(repeat(start.col))
            .map(|(row, col)| Coordinate { row, col })
            .collect_vec()
    }
}

fn build_grid(input: &Vec<Input>) -> HashMap<Coordinate<u32>, Contents> {
    let mut map = HashMap::new();
    for line in input {
        for (start, end) in line.iter().tuple_windows() {
            // every point between the points is rock
            let coords = iterate_points(start, end);
            for c in coords {
                map.insert(c, Contents::Rock);
            }
        }
    }
    map
}

fn drop_sand(
    grid: &HashMap<Coordinate<u32>, Contents>,
    lowest: u32,
    part: u32,
) -> Option<Coordinate<u32>> {
    let mut current = SAND_ORIGIN;
    loop {
        if part == 1 && current.row > lowest {
            return None;
        }
        let contents = grid.get(&current);
        match contents {
            Some(_) => {
                // there's something in the way
                let left_coord = Coordinate {
                    row: current.row,
                    col: current.col - 1,
                };
                let left = grid.get(&left_coord);
                if left.is_none() {
                    current = left_coord;
                } else {
                    let right_coord = Coordinate {
                        row: current.row,
                        col: current.col + 1,
                    };
                    let right = grid.get(&right_coord);
                    if right.is_none() {
                        current = right_coord;
                    } else {
                        // we stop one row above
                        return Some(Coordinate {
                            row: current.row - 1,
                            col: current.col,
                        });
                    }
                }
            }
            None => {
                current = Coordinate {
                    row: current.row + 1,
                    ..current
                };
            }
        }
    }
}

fn solve_part1(input: &Vec<Input>) -> usize {
    let mut grid = build_grid(input);
    // if we go below this, we're off the map
    let lowest = grid.keys().map(|c| c.row).max().unwrap();
    while let Some(c) = drop_sand(&grid, lowest, 1) {
        grid.insert(c, Contents::Sand);
    }
    count_grains(&grid)
}

fn count_grains(grid: &HashMap<Coordinate<u32>, Contents>) -> usize {
    grid.iter()
        .filter(|(_, v)| matches!(v, Contents::Sand))
        .count()
}

fn solve_part2(input: &Vec<Input>) -> usize {
    let mut grid = build_grid(input);
    // if we go below this, we're off the map
    let lowest = grid.keys().map(|c| c.row).max().unwrap();
    let floor = lowest + 2;
    // widest shape is a triangle
    let width = 2 * floor + 1;
    for col in (SAND_ORIGIN.col - width / 2)..=(SAND_ORIGIN.col + width / 2) {
        grid.insert(Coordinate { row: floor, col }, Contents::Rock);
    }

    while let Some(c) = drop_sand(&grid, lowest, 2) {
        grid.insert(c, Contents::Sand);
        if c == SAND_ORIGIN {
            break;
        }
    }
    count_grains(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parse_input(include_str!("../../input/day14.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 24);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parse_input(include_str!("../../input/day14.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 93);
        Ok(())
    }
}
