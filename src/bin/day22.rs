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

    // Return the (target block, target coordinate on that block, new facing)
    fn wrap_from_position_connected(
        &self,
        position: IndexingCoordinate,
        facing: Facing,
        connections: &Connections,
    ) -> (IndexingCoordinate, IndexingCoordinate, Facing) {
        let (block, target_edge) = match facing {
            Facing::Right => connections.right,
            Facing::Down => connections.bottom,
            Facing::Left => connections.left,
            Facing::Up => connections.top,
        };
        let target_facing = match target_edge {
            Facing::Right => Facing::Left,
            Facing::Down => Facing::Up,
            Facing::Left => Facing::Right,
            Facing::Up => Facing::Down,
        };
        let target_position = match (facing, target_edge) {
            (Facing::Right, Facing::Right) => IndexingCoordinate {
                row: self.height - position.row + 1,
                col: position.col,
            },
            (Facing::Right, Facing::Down) => position.transpose(),
            (Facing::Right, Facing::Left) => IndexingCoordinate {
                row: position.row,
                col: self.width - position.col + 1,
            },
            (Facing::Right, Facing::Up) => IndexingCoordinate {
                // should be 1
                row: self.height - position.col + 1,
                col: self.width - position.row + 1,
            },
            (Facing::Down, Facing::Right) => position.transpose(),
            (Facing::Down, Facing::Down) => IndexingCoordinate {
                row: position.row,
                col: self.width - position.col + 1,
            },
            (Facing::Down, Facing::Left) => IndexingCoordinate {
                row: self.height - position.col + 1,
                col: self.width - position.row + 1,
            },
            (Facing::Down, Facing::Up) => IndexingCoordinate {
                row: self.height - position.row + 1,
                col: position.col,
            },
            (Facing::Left, Facing::Right) => IndexingCoordinate {
                row: position.row,
                col: self.width - position.col + 1,
            },
            (Facing::Left, Facing::Down) => IndexingCoordinate {
                row: self.height - position.col + 1,
                col: self.width - position.row + 1,
            },
            (Facing::Left, Facing::Left) => IndexingCoordinate {
                row: self.height - position.row + 1,
                col: position.col,
            },
            (Facing::Left, Facing::Up) => position.transpose(),
            (Facing::Up, Facing::Right) => IndexingCoordinate {
                row: self.height - position.col + 1,
                col: self.width - position.row + 1,
            },
            (Facing::Up, Facing::Down) => IndexingCoordinate {
                row: self.height - position.row + 1,
                col: position.col,
            },
            (Facing::Up, Facing::Left) => position.transpose(),
            (Facing::Up, Facing::Up) => IndexingCoordinate {
                row: position.row,
                col: self.width - position.col + 1,
            },
        };
        (block, target_position, target_facing)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        solve_part2(&input, 50),
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
    get_score(position, facing)
}

fn get_score(position: IndexingCoordinate, facing: Facing) -> usize {
    position.row * 1000
        + position.col * 4
        + match facing {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
}

fn solve_part2(input: &Input, grid_size: usize) -> usize {
    let map = &input.map;
    // divide map into six cube faces
    // get the top left corners of each grid
    let blocks = (1..map.height)
        .step_by(grid_size)
        .cartesian_product((1..map.width).step_by(grid_size))
        .map(|(row, col)| IndexingCoordinate { row, col })
        .filter(|c| {
            matches!(
                map.get_contents(*c),
                Some(Contents::Empty) | Some(Contents::Rock)
            )
        })
        .map(|c| {
            // normalise coordinates
            ((c.row - 1) / grid_size, (c.col - 1) / grid_size)
        })
        .collect_vec();
    assert_eq!(blocks.len(), 6);
    // find adjacent edges
    // we know the coordinates are in row, then col order. Pattern match them
    // map from each block, to the blocks it's connected to
    let connections: HashMap<IndexingCoordinate, Connections> = build_connections(&blocks);
    let maps: HashMap<(usize, usize), Map> = blocks
        .iter()
        .map(|&b| (b, build_map(b, grid_size, map)))
        .collect();

    // position relative to the local map. Need to translate back for the final score
    let mut position = IndexingCoordinate { row: 1, col: 1 };
    let mut facing = Facing::Right;
    let mut visited = HashMap::new();
    let mut current_block = *blocks.first().unwrap();
    let mut current_map = &maps[&current_block];
    for instruction in &input.instructions {
        match instruction {
            Instruction::Move(distance) => {
                for _ in 0..*distance {
                    let next_coord = move_direction(position, facing);
                    let contents = current_map.get_contents(next_coord);
                    match contents {
                        None => {
                            let (new_block, alt_coord, new_facing) = current_map
                                .wrap_from_position_connected(
                                    position,
                                    facing,
                                    connections.get(&current_block.into()).unwrap(),
                                );
                            let new_map = &maps[&(new_block.row, new_block.col)];
                            match new_map.get_contents(alt_coord) {
                                Some(Contents::Rock) => {
                                    break;
                                }
                                Some(Contents::Empty) => {
                                    position = alt_coord;
                                    current_block = (new_block.row, new_block.col);
                                    facing = new_facing;
                                    current_map = new_map;
                                }
                                _ => unreachable!("Invalid contents at {:?}", alt_coord),
                            }
                        }
                        Some(Contents::Rock) => {
                            // we stop
                            break;
                        }
                        Some(Contents::Empty) => position = next_coord,
                        Some(Contents::Void) => unreachable!("There is no void now"),
                    }
                    visited.insert(
                        get_real_position(current_block, grid_size, position),
                        facing,
                    );
                }
            }
            Instruction::Rotate(rotation) => match rotation {
                Rotation::Clockwise => facing = facing.rotate_clockwise(),
                Rotation::CounterClockwise => facing = facing.rotate_counter_clockwise(),
            },
        }
        visited.insert(
            get_real_position(current_block, grid_size, position),
            facing,
        );
    }
    map.print_person(&visited);

    dbg!(current_block, position);
    // convert back to "real" position
    let real_position = get_real_position(current_block, grid_size, position);
    dbg!(real_position);
    get_score(real_position, facing)
}

fn get_real_position(
    block: (usize, usize),
    grid_size: usize,
    position: IndexingCoordinate,
) -> IndexingCoordinate {
    IndexingCoordinate {
        row: block.0 * grid_size + position.row,
        col: block.1 * grid_size + position.col,
    }
}

fn build_map(block: (usize, usize), grid_size: usize, original_map: &Map) -> Map {
    let terrain = original_map
        .terrain
        .iter()
        .chunks(grid_size)
        .into_iter()
        // find the start of our block
        .skip(block.0 * original_map.width + block.1)
        // split into map width number of blocks
        .chunks(original_map.width / grid_size)
        .into_iter()
        // grab the first one (which will be the block column we want)
        .map(|mut chunk| chunk.next().unwrap())
        .take(grid_size)
        .flat_map(|chunk| chunk.collect_vec())
        .copied()
        // pull that bad boy out
        .collect_vec();
    Map {
        width: grid_size,
        height: grid_size,
        terrain,
    }
}

fn build_connections(blocks: &Vec<(usize, usize)>) -> HashMap<IndexingCoordinate, Connections> {
    let connections = match blocks[..] {
        [(0, 2), (1, 0), (1, 1), (1, 2), (2, 2), (2, 3)] => {
            let mut conns = HashMap::new();
            conns.insert(
                (0, 2).into(),
                Connections {
                    top: ((1, 0).into(), Facing::Up),
                    left: ((1, 1).into(), Facing::Up),
                    right: ((2, 3).into(), Facing::Right),
                    bottom: ((1, 2).into(), Facing::Up),
                },
            );
            conns.insert(
                (1, 0).into(),
                Connections {
                    top: ((0, 2).into(), Facing::Up),
                    left: ((2, 3).into(), Facing::Down),
                    right: ((1, 1).into(), Facing::Left),
                    bottom: ((2, 2).into(), Facing::Down),
                },
            );
            conns.insert(
                (1, 1).into(),
                Connections {
                    top: ((0, 2).into(), Facing::Left),
                    left: ((1, 0).into(), Facing::Right),
                    right: ((1, 2).into(), Facing::Left),
                    bottom: ((2, 2).into(), Facing::Left),
                },
            );
            conns.insert(
                (1, 2).into(),
                Connections {
                    top: ((0, 2).into(), Facing::Down),
                    left: ((1, 1).into(), Facing::Right),
                    right: ((2, 3).into(), Facing::Up),
                    bottom: ((2, 2).into(), Facing::Up),
                },
            );
            conns.insert(
                (2, 2).into(),
                Connections {
                    top: ((1, 2).into(), Facing::Down),
                    left: ((1, 1).into(), Facing::Down),
                    right: ((2, 3).into(), Facing::Left),
                    bottom: ((1, 0).into(), Facing::Down),
                },
            );
            conns.insert(
                (2, 3).into(),
                Connections {
                    top: ((1, 2).into(), Facing::Right),
                    left: ((2, 2).into(), Facing::Right),
                    right: ((0, 2).into(), Facing::Right),
                    bottom: ((1, 0).into(), Facing::Left),
                },
            );
            conns
        }
        [(0, 1), (0, 2), (1, 1), (2, 0), (2, 1), (3, 0)] => {
            let mut conns = HashMap::new();
            conns.insert(
                (0, 1).into(),
                Connections {
                    top: ((3, 0).into(), Facing::Left),
                    left: ((2, 0).into(), Facing::Left),
                    right: ((0, 2).into(), Facing::Left),
                    bottom: ((1, 1).into(), Facing::Up),
                },
            );
            conns.insert(
                (0, 2).into(),
                Connections {
                    top: ((3, 0).into(), Facing::Down),
                    left: ((0, 1).into(), Facing::Right),
                    right: ((2, 1).into(), Facing::Right),
                    bottom: ((1, 1).into(), Facing::Right),
                },
            );
            conns.insert(
                (1, 1).into(),
                Connections {
                    top: ((0, 1).into(), Facing::Down),
                    left: ((2, 0).into(), Facing::Up),
                    right: ((0, 2).into(), Facing::Down),
                    bottom: ((2, 1).into(), Facing::Up),
                },
            );
            conns.insert(
                (2, 0).into(),
                Connections {
                    top: ((1, 1).into(), Facing::Left),
                    left: ((0, 1).into(), Facing::Left),
                    right: ((2, 1).into(), Facing::Left),
                    bottom: ((3, 0).into(), Facing::Up),
                },
            );
            conns.insert(
                (2, 1).into(),
                Connections {
                    top: ((1, 1).into(), Facing::Down),
                    left: ((2, 0).into(), Facing::Right),
                    right: ((0, 2).into(), Facing::Right),
                    bottom: ((3, 0).into(), Facing::Right),
                },
            );
            conns.insert(
                (3, 0).into(),
                Connections {
                    top: ((2, 0).into(), Facing::Down),
                    left: ((0, 1).into(), Facing::Up),
                    right: ((2, 1).into(), Facing::Down),
                    bottom: ((0, 2).into(), Facing::Up),
                },
            );
            conns
        }
        _ => panic!("Unknown cube net...pretend there's 9 more here"),
    };
    assert!(blocks.iter().all(|&b| {
        let b_coord: IndexingCoordinate = b.into();
        [Facing::Up, Facing::Right, Facing::Down, Facing::Left]
            .iter()
            .all(|&f| {
                let inner = connections.iter().any(|(_, c)| {
                    c.top == (b_coord, f)
                        || c.left == (b_coord, f)
                        || c.right == (b_coord, f)
                        || c.bottom == (b_coord, f)
                });
                if !inner {
                    dbg!(b, f);
                }
                inner
            })
    }));
    connections
}

#[derive(Debug, Clone, Copy)]
struct Connections {
    top: (IndexingCoordinate, Facing),
    left: (IndexingCoordinate, Facing),
    right: (IndexingCoordinate, Facing),
    bottom: (IndexingCoordinate, Facing),
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
        let result = solve_part2(&input, 4);
        assert_eq!(result, 5031);
        Ok(())
    }
}
