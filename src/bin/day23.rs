use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use advent_of_code2022::{get_adjacent_points_diagonal, Coordinate};
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn delta(&self) -> Coordinate<i32> {
        match &self {
            Direction::North => (-1, 0).into(),
            Direction::South => (1, 0).into(),
            Direction::West => (0, -1).into(),
            Direction::East => (0, 1).into(),
        }
    }
}

fn main() -> color_eyre::Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day23.txt"))?;
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input),
        time.elapsed().as_millis()
    );
    let time = Instant::now();
    println!(
        "Part 2: {} in {}ms",
        solve_part2(&input),
        time.elapsed().as_millis()
    );
    Ok(())
}

mod parsing {
    use advent_of_code2022::Coordinate;
    use std::collections::HashSet;

    pub fn parse_input(input: &str) -> color_eyre::Result<HashSet<Coordinate<i32>>> {
        Ok(input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(col, c)| match c {
                        '#' => Some(Coordinate {
                            row: row as i32,
                            col: col as i32,
                        }),
                        _ => None,
                    })
                    .collect::<HashSet<_>>()
            })
            .fold(HashSet::new(), |mut all, line| {
                all.extend(&line);
                all
            }))
    }
}

fn solve_part1(input: &HashSet<Coordinate<i32>>) -> i32 {
    let mut directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]
    .into_iter()
    .collect::<VecDeque<_>>();
    let mut elves = input.clone();
    for _ in 0..10 {
        elves = run_round(&elves, &directions);
        let first = directions.pop_front().unwrap();
        directions.push_back(first);
    }
    calculate_score(&elves)
}

fn calculate_score(elves: &HashSet<Coordinate<i32>>) -> i32 {
    let row_min = elves.iter().map(|c| c.row).min().unwrap();
    let row_max = elves.iter().map(|c| c.row).max().unwrap();
    let col_min = elves.iter().map(|c| c.col).min().unwrap();
    let col_max = elves.iter().map(|c| c.col).max().unwrap();
    (col_max - col_min + 1) * (row_max - row_min + 1) - elves.len() as i32
}

fn run_round(
    elves: &HashSet<Coordinate<i32>>,
    directions: &VecDeque<Direction>,
) -> HashSet<Coordinate<i32>> {
    let proposed_directions = get_proposals(elves, directions);
    let proposed_coordinates: HashMap<Coordinate<_>, Option<Coordinate<_>>> = proposed_directions
        .iter()
        .map(|(c, maybe_direction)| (*c, maybe_direction.map(|d| *c + d.delta())))
        .collect();
    let valid_moves: HashSet<Coordinate<_>> = proposed_coordinates
        .iter()
        .filter_map(|(_, possible)| *possible)
        .fold(
            HashMap::<Coordinate<_>, u32>::new(),
            |mut total, possible| {
                *total.entry(possible).or_default() += 1;
                total
            },
        )
        .iter()
        .filter_map(|(c, count)| if *count == 1 { Some(*c) } else { None })
        .collect();
    // convert proposed into actual
    proposed_coordinates
        .iter()
        .map(|(&current, proposed)| match proposed {
            Some(p) => {
                if valid_moves.contains(p) {
                    *p
                } else {
                    current
                }
            }
            None => current,
        })
        .collect()
}

fn get_proposals(
    elves: &HashSet<Coordinate<i32>>,
    directions: &VecDeque<Direction>,
) -> HashMap<Coordinate<i32>, Option<Direction>> {
    elves
        .iter()
        .map(|&e| {
            if get_adjacent_points_diagonal(e, i32::MAX, i32::MAX)
                .into_iter()
                .all(|possible| !elves.contains(&possible))
            {
                return (e, None);
            }
            let possible = directions
                .iter()
                .find(|&d| {
                    match d {
                        Direction::North => [(-1, -1).into(), (-1, 0).into(), (-1, 1).into()],
                        Direction::South => [(1, -1).into(), (1, 0).into(), (1, 1).into()],
                        Direction::West => [(-1, -1).into(), (0, -1).into(), (1, -1).into()],
                        Direction::East => [(-1, 1).into(), (0, 1).into(), (1, 1).into()],
                    }
                    .iter()
                    .all(|possible: &Coordinate<i32>| !elves.contains(&(e + *possible)))
                })
                .copied();
            (e, possible)
        })
        .collect()
}

fn solve_part2(input: &HashSet<Coordinate<i32>>) -> i32 {
    let mut directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]
    .into_iter()
    .collect::<VecDeque<_>>();
    let mut elves = input.clone();
    let mut round_number = 1;
    loop {
        let new_elves = run_round(&elves, &directions);
        if elves == new_elves {
            break;
        }
        elves = new_elves;
        let first = directions.pop_front().unwrap();
        directions.push_back(first);
        round_number += 1;
    }
    round_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day23.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 110);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day23.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 20);
        Ok(())
    }
}
