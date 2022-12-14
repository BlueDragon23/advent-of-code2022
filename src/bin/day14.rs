use std::{collections::HashMap, iter::repeat};

use advent_of_code2022::{print_coordinates, Coordinate};
use itertools::Itertools;
use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list1, sequence::separated_pair,
    Finish, IResult,
};

#[derive(Clone, Debug)]
struct Input {
    rock_points: Vec<Coordinate<u32>>,
}

#[derive(Debug)]
enum Contents {
    Empty,
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
        .map(|line| Input {
            rock_points: parse_line(line).finish().unwrap().1,
        })
        .collect())
}

fn iterate_points(start: &Coordinate<u32>, end: &Coordinate<u32>) -> Vec<Coordinate<u32>> {
    if start.row == end.row {
        // make sure they're the same order
        if start.col < end.col {
            (start.col..=end.col)
                .zip(repeat(start.row))
                .map(|(col, row)| Coordinate { row, col })
                .collect_vec()
        } else {
            (end.col..=start.col)
                .zip(repeat(start.row))
                .map(|(col, row)| Coordinate { row, col })
                .collect_vec()
        }
    } else {
        if start.row < end.row {
            (start.row..=end.row)
                .zip(repeat(start.col))
                .map(|(row, col)| Coordinate { row, col })
                .collect_vec()
        } else {
            (end.row..=start.row)
                .zip(repeat(start.col))
                .map(|(row, col)| Coordinate { row, col })
                .collect_vec()
        }
    }
}

fn build_grid(input: &Vec<Input>) -> HashMap<Coordinate<u32>, Contents> {
    let mut map = HashMap::new();
    for line in input {
        for (start, end) in line.rock_points.iter().tuple_windows() {
            // every point between the points is rock
            let coords = iterate_points(start, end);
            for c in coords {
                map.insert(c, Contents::Rock);
            }
        }
    }
    map
}

fn access_grid<'a>(
    coordinate: &Coordinate<u32>,
    grid: &'a HashMap<Coordinate<u32>, Contents>,
    lowest: u32,
    part: u32,
) -> Option<&'a Contents> {
    if part == 2 && coordinate.row == lowest + 2 {
        Some(&Contents::Rock)
    } else {
        grid.get(coordinate)
    }
}

fn drop_sand(
    grid: &HashMap<Coordinate<u32>, Contents>,
    lowest: u32,
    part: u32,
) -> Option<Coordinate<u32>> {
    let mut current = SAND_ORIGIN;
    loop {
        if part == 1 && current.row > lowest {
            dbg!(&current);
            return None;
        }
        let contents = access_grid(&current, grid, lowest, part);
        match contents {
            Some(_) => {
                // there's something in the way
                let left_coord = Coordinate {
                    row: current.row,
                    col: current.col - 1,
                };
                let left = access_grid(&left_coord, grid, lowest, part);
                if left.is_none() {
                    current = left_coord;
                } else {
                    let right_coord = Coordinate {
                        row: current.row,
                        col: current.col + 1,
                    };
                    let right = access_grid(&right_coord, grid, lowest, part);
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
    // print_coordinates(&grid.keys().copied().collect_vec(), true);
    // if we go below this, we're off the map
    let lowest = grid.keys().map(|c| c.row).max().unwrap();
    while let Some(c) = drop_sand(&grid, lowest, 1) {
        grid.insert(c, Contents::Sand);
        // print_coordinates(&grid.keys().copied().collect_vec(), true);
    }
    grid.iter()
        .filter(|(_, v)| matches!(v, Contents::Sand))
        .count()
}

fn solve_part2(input: &Vec<Input>) -> usize {
    let mut grid = build_grid(input);
    // print_coordinates(&grid.keys().copied().collect_vec(), true);
    // if we go below this, we're off the map
    let lowest = grid.keys().map(|c| c.row).max().unwrap();
    while let Some(c) = drop_sand(&grid, lowest, 2) {
        grid.insert(c, Contents::Sand);
        if c == SAND_ORIGIN {
            break;
        }
        // print_coordinates(&grid.keys().copied().collect_vec(), true);
    }
    grid.iter()
        .filter(|(_, v)| matches!(v, Contents::Sand))
        .count()
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
