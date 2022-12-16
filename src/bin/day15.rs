use std::{collections::HashSet, hash::Hash, ops::Sub};

use advent_of_code2022::Coordinate;
use itertools::Itertools;
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

struct BoundingBox {
    left: Coordinate<i32>,
    right: Coordinate<i32>,
    // top is smaller than bottom
    top: Coordinate<i32>,
    bottom: Coordinate<i32>,
    width: i32,
    height: i32,
    center: Coordinate<i32>,
}

fn main() -> color_eyre::Result<()> {
    let input = parse_input(include_str!("../../input/day15.txt"))?;
    println!("Part 1: {}", solve_part1(&input, 2_000_000));
    println!("Part 2: {}", solve_part2_set(&input, 4_000_000));
    Ok(())
}

fn parse_assignment(input: &str) -> IResult<&str, i32> {
    preceded(take(2 as u32), nom::character::complete::i32)(input)
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

fn solve_part1(input: &Vec<Input>, row: i32) -> i32 {
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

fn solve_part2(input: &Vec<Input>, max_coord: i32) -> i32 {
    let min_coord = 0;
    let beacon_locations: HashSet<_> = input.iter().map(|i| i.beacon).collect();
    let result = (min_coord..=max_coord)
        .cartesian_product(min_coord..=max_coord)
        .map(|(row, col)| Coordinate { row, col })
        .find(|c| {
            input
                .iter()
                .find(|i| manhattan_distance(*c, i.sensor) <= i.distance)
                .is_none()
                && !beacon_locations.contains(c)
        })
        .unwrap();
    dbg!(result);
    result.col * 4000000 + result.row
}

fn generate_coordinates(input: &Input) -> HashSet<Coordinate<i32>> {
    let mut coords = HashSet::new();
    for row in (input.sensor.row - input.distance)..=(input.sensor.row + input.distance) {
        let diff = abs(input.sensor.row - row);
        let start_col = (input.sensor.col - input.distance) + diff;
        let end_col = (input.sensor.col + input.distance) - diff;
        for col in start_col..=end_col {
            coords.insert(Coordinate { row, col });
        }
    }
    coords
}

fn solve_part2_set(inputs: &Vec<Input>, max_coord: i32) -> i32 {
    let min_coord = 0;
    // build sets
    let entire_region: HashSet<_> = (min_coord..=max_coord)
        .cartesian_product(min_coord..=max_coord)
        .map(|(row, col)| Coordinate { row, col })
        .collect();
    let undetected = inputs.iter().fold(entire_region, |remaining, input| {
        remaining.sub(&generate_coordinates(input))
    });
    dbg!(&undetected);

    let result = undetected.iter().next().unwrap();
    result.col * 4000000 + result.row
}

impl BoundingBox {
    fn new(
        left: Coordinate<i32>,
        right: Coordinate<i32>,
        top: Coordinate<i32>,
        bottom: Coordinate<i32>,
    ) -> BoundingBox {
        BoundingBox {
            left,
            right,
            top,
            bottom,
            width: right.col - left.col,
            height: bottom.row - top.row,
            // we know it's a square diamond, so we can cheat
            // TODO: it's not a square diamond once we start subdividing
            center: Coordinate {
                row: left.row,
                col: top.col,
            },
        }
    }

    fn is_subset_of(&self, other: &BoundingBox) -> bool {
        // all points of self inside other
        self.left.col >= other.left.col
            && self.right.col <= other.right.col
            && self.top.row >= other.top.col
            && self.bottom.row <= other.bottom.row
    }

    fn contains(&self, other: &Coordinate<i32>) -> bool {
        let dx = abs(other.row - self.center.row);
        let dy = abs(other.col - self.center.col);
        (dx / self.width + dy / self.height) <= 1
    }

    fn get_subset_corners(&self, other: &BoundingBox) -> Vec<Coordinate<i32>> {
        vec![other.left, other.right, other.top, other.bottom]
            .iter()
            .filter(|c| self.contains(c))
            .copied()
            .collect_vec()
    }

    fn divide_around(&self, coordinate: Coordinate<i32>) -> Vec<BoundingBox> {
        // create four new boxes
        // TODO: it's not a square diamond once we start subdividing

        vec![]
    }
}

fn merge_bounding_box(a: BoundingBox, b: BoundingBox) -> Vec<BoundingBox> {
    // if subset, return the larger box
    if a.is_subset_of(&b) {
        vec![b]
    } else if b.is_subset_of(&a) {
        vec![a]
    } else {
        // divide into unique boxes
        let a_subset = a.get_subset_corners(&b);
        let b_subset = b.get_subset_corners(&a);
        match a_subset.len() {
            0 => {
                // no corners of b are inside a, how about the other
                match b_subset.len() {
                    // no overlap, leave them alone
                    0 => vec![a, b],
                    2 => vec![],
                    _ => unreachable!("This should be impossible"),
                }
            }
            1 => {
                // this is symmetrical. Divide into 7 new bounding boxes
                // this doesn't work because the in between coordinates could be floats
                unimplemented!("oof")
            }
            2 => unimplemented!("oof"),
            _ => unreachable!("This should be impossible"),
        }
    }
}

fn solve_part2_bounding(input: &Vec<Input>, max_coord: i32) -> i32 {
    let min_coord = 0;
    let beacon_locations: HashSet<_> = input.iter().map(|i| i.beacon).collect();
    let bounding_boxes: Vec<BoundingBox> = input
        .iter()
        .map(|i| {
            let (left, right, top, bottom) = [
                (0, -i.distance),
                (0, i.distance),
                (-i.distance, 0),
                (i.distance, 0),
            ]
            .iter()
            .map(|(drow, dcol)| Coordinate {
                row: i.sensor.row + drow,
                col: i.sensor.col + dcol,
            })
            .collect_tuple()
            .unwrap();
            BoundingBox::new(left, right, top, bottom)
        })
        .collect();

    let result = (min_coord..=max_coord)
        .cartesian_product(min_coord..=max_coord)
        .map(|(row, col)| Coordinate { row, col })
        .find(|c| {
            input
                .iter()
                .find(|i| manhattan_distance(*c, i.sensor) <= i.distance)
                .is_none()
                && !beacon_locations.contains(c)
        })
        .unwrap();
    dbg!(result);
    result.col * 4000000 + result.row
}

#[cfg(test)]
mod tests {
    use std::ops::Sub;

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
    fn test_part2_set() -> color_eyre::Result<()> {
        let input = parse_input(include_str!("../../input/day15.test.txt"))?;
        let result = solve_part2_set(&input, 20);
        assert_eq!(result, 56000011);
        Ok(())
    }

    #[test]
    fn test_generate_coordinates() {
        let input = Input {
            sensor: Coordinate { row: 2, col: 2 },
            beacon: Coordinate::default(),
            distance: 2,
        };
        let result = generate_coordinates(&input);
        let expected: HashSet<Coordinate<i32>> = vec![
            Coordinate::new(0, 2),
            Coordinate::new(1, 1),
            Coordinate::new(1, 2),
            Coordinate::new(1, 3),
            Coordinate::new(2, 0),
            Coordinate::new(2, 1),
            Coordinate::new(2, 2),
            Coordinate::new(2, 3),
            Coordinate::new(2, 4),
            Coordinate::new(3, 1),
            Coordinate::new(3, 2),
            Coordinate::new(3, 3),
            Coordinate::new(4, 2),
        ]
        .into_iter()
        .collect();
        dbg!(expected.sub(&result));
        dbg!(result.sub(&expected));
        assert_eq!(result, expected);
    }
}
