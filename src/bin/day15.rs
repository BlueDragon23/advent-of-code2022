use std::{
    collections::{HashMap, HashSet},
    ops::ControlFlow,
    time::Instant,
};

use advent_of_code2022::{Coordinate, Range};
use nom::{
    bytes::complete::{tag, take},
    combinator::map,
    sequence::{pair, preceded, separated_pair},
    Finish, IResult,
};
use num::abs;

#[derive(Debug, Clone)]
struct Input {
    sensor: Coordinate<i32>,
    beacon: Coordinate<i32>,
    distance: i32,
}

fn main() -> color_eyre::Result<()> {
    let input = parse_input(include_str!("../../input/day15.txt"))?;
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input, 2_000_000),
        time.elapsed().as_millis()
    );
    let time = Instant::now();
    println!(
        "Part 2: {} in {}ms",
        solve_part2(&input, 4_000_000),
        time.elapsed().as_millis()
    );
    Ok(())
}

fn parse_assignment(input: &str) -> IResult<&str, i32> {
    preceded(take(2u32), nom::character::complete::i32)(input)
}

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate<i32>> {
    map(
        separated_pair(parse_assignment, tag(", "), parse_assignment),
        |(x, y)| Coordinate { row: y, col: x },
    )(input)
}

fn parse_sensor(input: &str) -> IResult<&str, Coordinate<i32>> {
    preceded(tag("Sensor at "), parse_coordinate)(input)
}

fn parse_beacon(input: &str) -> IResult<&str, Coordinate<i32>> {
    preceded(tag(": closest beacon is at "), parse_coordinate)(input)
}

fn parse_line(input: &str) -> IResult<&str, Input> {
    map(pair(parse_sensor, parse_beacon), |(sensor, beacon)| Input {
        sensor,
        beacon,
        distance: manhattan_distance(sensor, beacon),
    })(input)
}

fn parse_input(input: &str) -> color_eyre::Result<Vec<Input>> {
    Ok(input
        .lines()
        .map(|line| parse_line(line).finish().unwrap().1)
        .collect())
}

fn manhattan_distance(start: Coordinate<i32>, end: Coordinate<i32>) -> i32 {
    abs(end.row - start.row) + abs(end.col - start.col)
}

fn solve_part1(input: &[Input], row: i32) -> i32 {
    let min_col = input
        .iter()
        .flat_map(|i| [i.beacon.col, i.sensor.col])
        .min()
        .unwrap()
        - 10_000_000;
    let max_col = input
        .iter()
        .flat_map(|i| [i.beacon.col, i.sensor.col])
        .max()
        .unwrap()
        + 10_000_000;
    let beacon_locations: HashSet<_> = input.iter().map(|i| i.beacon).collect();

    let mut count = 0;
    for col in min_col..=max_col {
        // find closest sensor
        let coordinate = Coordinate { row, col };
        if input
            .iter()
            .find(|i| manhattan_distance(coordinate, i.sensor) <= i.distance)
            .is_some()
            && !beacon_locations.contains(&coordinate)
        {
            count += 1
        }
    }
    count
}

fn solve_part2(input: &[Input], max_bound: i32) -> i64 {
    let total_range = Range {
        lower: 0,
        upper: max_bound,
    };
    let every_range = input
        .iter()
        .map(generate_ranges)
        // get a ordered list of ranges in each row
        .fold(
            HashMap::<i32, Vec<Range>>::new(),
            |mut all_ranges, subranges| {
                subranges.into_iter().for_each(|(row, range)| {
                    if row >= 0 && row <= max_bound && range.overlap(&total_range) {
                        all_ranges.entry(row).or_default().push(range);
                    }
                });
                all_ranges
            },
        );
    if let Some((row, ranges)) = every_range
        .iter()
        .map(|(row, ranges)| {
            let mut new_ranges = ranges.clone();
            new_ranges.sort_by_key(|r| r.lower);
            (row, new_ranges)
        })
        .filter_map(|(row, ranges)| {
            let (head, rest) = ranges.split_at(1);
            let final_range = rest.iter().try_fold(head[0], |total, r| {
                if total.overlap_or_adjacent(r) {
                    Some(total.merge(r))
                } else {
                    None
                }
            });
            if final_range.is_none() {
                Some((row, ranges))
            } else {
                None
            }
        })
        .next()
    {
        // find the one with a gap
        let (head, rest) = ranges.split_at(1);
        if let ControlFlow::Break(r) = rest.iter().try_fold(head[0], |total, r| {
            if total.overlap_or_adjacent(r) {
                ControlFlow::Continue(total.merge(r))
            } else {
                ControlFlow::Break(*r)
            }
        }) {
            println!("x={}, y={}", r.lower - 1, row);
            ((r.lower - 1) as i64) * 4000000 + (*row as i64)
        } else {
            panic!("Failed to find a result")
        }
    } else {
        panic!("Failed to find a result")
    }
}

// get a map from row to range occupied
fn generate_ranges(input: &Input) -> HashMap<i32, Range> {
    // make the diamond shaped ranges
    ((input.sensor.row - input.distance)..=(input.sensor.row + input.distance))
        .map(|row| {
            (
                row,
                Range {
                    lower: (input.sensor.col - input.distance) + abs(input.sensor.row - row),
                    upper: (input.sensor.col + input.distance) - abs(input.sensor.row - row),
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parse_input(include_str!("../../input/day15.test.txt"))?;
        let result = solve_part1(&input, 10);
        assert_eq!(result, 26);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parse_input(include_str!("../../input/day15.test.txt"))?;
        let result = solve_part2(&input, 20);
        assert_eq!(result, 56000011);
        Ok(())
    }

    #[test]
    fn test_generate_ranges() {
        let ranges = generate_ranges(&Input {
            sensor: Coordinate { row: 2, col: 2 },
            beacon: Coordinate { row: 2, col: 2 },
            distance: 2,
        });
        let mut expected_ranges = HashMap::new();
        expected_ranges.insert(4, Range { lower: 2, upper: 2 });
        expected_ranges.insert(3, Range { lower: 1, upper: 3 });
        expected_ranges.insert(2, Range { lower: 0, upper: 4 });
        expected_ranges.insert(1, Range { lower: 1, upper: 3 });
        expected_ranges.insert(0, Range { lower: 2, upper: 2 });
        assert_eq!(ranges, expected_ranges);
    }
}
