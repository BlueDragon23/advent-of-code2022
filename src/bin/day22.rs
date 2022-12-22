use std::{collections::HashMap, time::Instant};

use advent_of_code2022::IndexingCoordinate;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Input {
    map: Map,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct Map {
    width: usize,
    height: usize,
    terrain: Vec<Contents>,
}

impl Map {
    fn get_contents(&self, coordinate: IndexingCoordinate) -> Option<Contents> {
        if coordinate.row < 1
            || coordinate.col < 1
            || coordinate.row > self.height
            || coordinate.col > self.width
        {
            None
        } else {
            Some(self.terrain[(coordinate.row - 1) * self.width + (coordinate.col - 1)])
        }
    }

    fn print(&self) {
        for row in 1..=self.height {
            for col in 1..=self.width {
                let c = self.get_contents(IndexingCoordinate { row, col }).unwrap();
                match c {
                    Contents::Rock => print!("#"),
                    Contents::Empty => print!("."),
                    Contents::Void => print!(" "),
                }
            }
            println!()
        }
        println!()
    }

    fn print_person(&self, visited: &HashMap<IndexingCoordinate, Facing>) {
        for row in 1..=self.height {
            for col in 1..=self.width {
                let pos = IndexingCoordinate { row, col };
                if let Some(facing) = visited.get(&pos) {
                    match facing {
                        Facing::Right => print!(">"),
                        Facing::Down => print!("v"),
                        Facing::Left => print!("<"),
                        Facing::Up => print!("^"),
                    }
                } else {
                    let c = self.get_contents(pos).unwrap();
                    match c {
                        Contents::Rock => print!("#"),
                        Contents::Empty => print!("."),
                        Contents::Void => print!(" "),
                    }
                }
            }
            println!()
        }
        println!()
    }

    // find the column of the first non void element in the row
    fn first_in_row(&self, row: usize) -> usize {
        // fucking 1-indexing
        (1..=self.width)
            .find(|col| {
                !matches!(
                    self.get_contents(IndexingCoordinate { row, col: *col })
                        .unwrap(),
                    Contents::Void
                )
            })
            .unwrap()
    }

    fn last_in_row(&self, row: usize) -> usize {
        (1..=self.width)
            .rev()
            .find(|col| {
                !matches!(
                    self.get_contents(IndexingCoordinate { row, col: *col })
                        .unwrap(),
                    Contents::Void
                )
            })
            .unwrap()
    }

    fn first_in_col(&self, col: usize) -> usize {
        (1..=self.height)
            .find(|row| {
                !matches!(
                    self.get_contents(IndexingCoordinate { row: *row, col })
                        .unwrap(),
                    Contents::Void
                )
            })
            .unwrap()
    }

    fn last_in_col(&self, col: usize) -> usize {
        (1..=self.height)
            .rev()
            .find(|row| {
                !matches!(
                    self.get_contents(IndexingCoordinate { row: *row, col })
                        .unwrap(),
                    Contents::Void
                )
            })
            .unwrap()
    }

    fn wrap_from_position(
        &self,
        position: IndexingCoordinate,
        facing: Facing,
    ) -> IndexingCoordinate {
        match facing {
            Facing::Right => IndexingCoordinate {
                col: self.first_in_row(position.row),
                ..position
            },
            Facing::Down => IndexingCoordinate {
                row: self.first_in_col(position.col),
                ..position
            },
            Facing::Left => IndexingCoordinate {
                col: self.last_in_row(position.row),
                ..position
            },
            Facing::Up => IndexingCoordinate {
                row: self.last_in_col(position.col),
                ..position
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Contents {
    Rock,
    Empty,
    Void,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Move(u32),
    Rotate(Rotation),
}

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone, Copy)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn rotate_clockwise(&self) -> Facing {
        match self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }
    fn rotate_counter_clockwise(&self) -> Facing {
        match self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        }
    }
}

fn main() -> color_eyre::Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day22.txt"))?;
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
    use std::iter::repeat;

    use crate::{Contents, Instruction, Map, Rotation};

    use super::Input;
    use itertools::Itertools;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        combinator::{map, map_opt},
        multi::{many1, separated_list1},
        sequence::separated_pair,
        Finish, IResult,
    };

    fn parse_char(input: &str) -> IResult<&str, Contents> {
        map_opt(take1, |c| match c {
            " " => Some(Contents::Void),
            "." => Some(Contents::Empty),
            "#" => Some(Contents::Rock),
            "\n" => None,
            _ => panic!("Unexpected input {}", c),
        })(input)
    }

    fn parse_line(input: &str) -> IResult<&str, Vec<Contents>> {
        many1(parse_char)(input)
    }

    fn parse_map(input: &str) -> IResult<&str, Map> {
        map(separated_list1(tag("\n"), parse_line), |list_of_lists| {
            let max_length = list_of_lists.iter().map(|l| l.len()).max().unwrap();
            Map {
                width: max_length,
                height: list_of_lists.len(),
                terrain: list_of_lists
                    .into_iter()
                    .flat_map(|mut l| {
                        if l.len() < max_length {
                            // extend the array to the length of the longest row
                            l.extend(repeat(Contents::Void).take(max_length - l.len()));
                        }
                        assert_eq!(l.len(), max_length);
                        l
                    })
                    .collect_vec(),
            }
        })(input)
    }

    fn take1(input: &str) -> IResult<&str, &str> {
        take(1usize)(input)
    }

    fn parse_rotation(input: &str) -> IResult<&str, Instruction> {
        map(alt((tag("R"), tag("L"))), |c| match c {
            "R" => Instruction::Rotate(Rotation::Clockwise),
            "L" => Instruction::Rotate(Rotation::CounterClockwise),
            _ => unreachable!("We only match those two"),
        })(input)
    }

    fn parse_move(input: &str) -> IResult<&str, Instruction> {
        map(nom::character::complete::u32, Instruction::Move)(input)
    }

    fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        many1(alt((parse_rotation, parse_move)))(input)
    }

    fn parse_data(input: &str) -> IResult<&str, Input> {
        map(
            separated_pair(parse_map, tag("\n\n"), parse_instructions),
            |(map, instructions)| Input { map, instructions },
        )(input)
    }

    pub fn parse_input(input: &str) -> color_eyre::Result<Input> {
        Ok(parse_data(input).finish().unwrap().1)
    }
}

fn move_direction(coordinate: IndexingCoordinate, facing: Facing) -> IndexingCoordinate {
    match facing {
        Facing::Right => IndexingCoordinate {
            col: coordinate.col + 1,
            ..coordinate
        },
        Facing::Down => IndexingCoordinate {
            row: coordinate.row + 1,
            ..coordinate
        },
        Facing::Left => IndexingCoordinate {
            col: coordinate.col - 1,
            ..coordinate
        },
        Facing::Up => IndexingCoordinate {
            row: coordinate.row - 1,
            ..coordinate
        },
    }
}

fn solve_part1(input: &Input) -> usize {
    let map = &input.map;
    map.print();
    let start_col = map.first_in_row(1);
    let mut position = IndexingCoordinate {
        row: 1,
        col: start_col,
    };
    let mut facing = Facing::Right;
    let mut visited = HashMap::new();
    for instruction in &input.instructions {
        match instruction {
            Instruction::Move(distance) => {
                for _ in 0..*distance {
                    let next_coord = move_direction(position, facing);
                    let contents = map.get_contents(next_coord);
                    match contents {
                        None => {
                            let alt_coord = map.wrap_from_position(position, facing);
                            dbg!(alt_coord);
                            match map.get_contents(alt_coord) {
                                Some(Contents::Rock) => {
                                    break;
                                }
                                Some(Contents::Empty) => {
                                    position = alt_coord;
                                }
                                _ => unreachable!("Invalid contents"),
                            }
                        }
                        Some(Contents::Void) => {
                            let alt_coord = map.wrap_from_position(position, facing);
                            match map.get_contents(alt_coord) {
                                Some(Contents::Rock) => {
                                    break;
                                }
                                Some(Contents::Empty) => {
                                    position = alt_coord;
                                }
                                _ => unreachable!("Invalid contents"),
                            }
                        }
                        Some(Contents::Rock) => {
                            // we stop
                            break;
                        }
                        Some(Contents::Empty) => position = next_coord,
                    }
                    visited.insert(position, facing);
                }
            }
            Instruction::Rotate(rotation) => match rotation {
                Rotation::Clockwise => facing = facing.rotate_clockwise(),
                Rotation::CounterClockwise => facing = facing.rotate_counter_clockwise(),
            },
        }
        visited.insert(position, facing);
    }
    map.print_person(&visited);
    dbg!(position, facing);
    position.row * 1000
        + position.col * 4
        + match facing {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
}

fn solve_part2(input: &Input) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day22.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 6032);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day22.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 1);
        Ok(())
    }
}
