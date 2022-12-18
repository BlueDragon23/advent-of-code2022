use std::{collections::HashSet, time::Instant};

use advent_of_code2022::PosCoordinate;
use itertools::Itertools;
use shapes::{Cross, HLine, Rock, ShapeType, Square, VLine, L};

#[derive(Clone, Debug)]
pub struct Input {
    jets: Vec<Jet>,
}

#[derive(Clone, Debug)]
pub enum Jet {
    Left,
    Right,
}

fn main() -> color_eyre::Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day17.txt"));
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

    use super::{Input, Jet};
    use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::many0, Finish, IResult};

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(
            many0(map(alt((tag("<"), tag(">"))), |c| match c {
                "<" => Jet::Left,
                ">" => Jet::Right,
                _ => unreachable!(""),
            })),
            |jets| Input { jets },
        )(input)
    }

    pub fn parse_input(input: &str) -> Input {
        input
            .lines()
            .next()
            .map(|line| parse_line(line).finish().unwrap().1)
            .unwrap()
    }
}

mod shapes {
    use std::collections::HashSet;

    use advent_of_code2022::PosCoordinate;

    const MAX_COL: u64 = 8;

    #[derive(Clone, Copy)]
    pub enum ShapeType {
        HLine,
        Cross,
        L,
        VLine,
        Square,
    }

    pub struct Rock {
        pub shape: Box<dyn Shape>,
        pub shape_type: ShapeType,
    }

    pub trait Shape {
        fn move_left(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>>;
        fn move_right(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>>;
        fn descend(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>>;
        fn get_coordinates(&self) -> Vec<PosCoordinate>;
        fn get_highest(&self) -> PosCoordinate;
    }

    // new functions take the coordinate two units from the left wall, three units up from the highest rock

    #[derive(Clone, Copy, Debug)]
    pub struct HLine {
        bottom: PosCoordinate,
        centre_bottom: PosCoordinate,
        centre_top: PosCoordinate,
        top: PosCoordinate,
    }

    impl HLine {
        pub fn new(coordinate: PosCoordinate) -> Self {
            HLine {
                bottom: coordinate,
                centre_bottom: PosCoordinate {
                    col: coordinate.col + 1,
                    ..coordinate
                },
                centre_top: PosCoordinate {
                    col: coordinate.col + 2,
                    ..coordinate
                },
                top: PosCoordinate {
                    col: coordinate.col + 3,
                    ..coordinate
                },
            }
        }
    }

    impl Shape for HLine {
        fn move_left(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let new_left = PosCoordinate {
                row: self.bottom.row,
                col: self.bottom.col - 1,
            };
            if new_left.col == 0 || occupied.contains(&new_left) {
                return None;
            }
            Some(Box::new(HLine {
                bottom: new_left,
                centre_bottom: self.bottom,
                centre_top: self.centre_bottom,
                top: self.centre_top,
            }))
        }

        fn move_right(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let new_right = PosCoordinate {
                row: self.top.row,
                col: self.top.col + 1,
            };
            if new_right.col == MAX_COL || occupied.contains(&new_right) {
                return None;
            }
            Some(Box::new(HLine {
                bottom: self.centre_bottom,
                centre_bottom: self.centre_top,
                centre_top: self.top,
                top: new_right,
            }))
        }

        fn descend(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let new_self = HLine {
                bottom: PosCoordinate {
                    row: self.bottom.row - 1,
                    col: self.bottom.col,
                },
                centre_bottom: PosCoordinate {
                    row: self.bottom.row - 1,
                    col: self.centre_bottom.col,
                },
                centre_top: PosCoordinate {
                    row: self.bottom.row - 1,
                    col: self.centre_top.col,
                },
                top: PosCoordinate {
                    row: self.bottom.row - 1,
                    col: self.top.col,
                },
            };
            if new_self.bottom.row == 0
                || occupied.contains(&new_self.bottom)
                || occupied.contains(&new_self.centre_bottom)
                || occupied.contains(&new_self.centre_top)
                || occupied.contains(&new_self.top)
            {
                return None;
            }
            Some(Box::new(new_self))
        }

        fn get_coordinates(&self) -> Vec<PosCoordinate> {
            vec![self.bottom, self.centre_bottom, self.centre_top, self.top]
        }

        fn get_highest(&self) -> PosCoordinate {
            self.top
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Cross {
        left: PosCoordinate,
        top: PosCoordinate,
        centre: PosCoordinate,
        right: PosCoordinate,
        bottom: PosCoordinate,
    }

    impl Cross {
        pub fn new(coordinate: PosCoordinate) -> Self {
            Cross {
                left: PosCoordinate {
                    row: coordinate.row + 1,
                    ..coordinate
                },
                centre: PosCoordinate {
                    row: coordinate.row + 1,
                    col: coordinate.col + 1,
                },
                right: PosCoordinate {
                    row: coordinate.row + 1,
                    col: coordinate.col + 2,
                },
                bottom: PosCoordinate {
                    row: coordinate.row,
                    col: coordinate.col + 1,
                },
                top: PosCoordinate {
                    row: coordinate.row + 2,
                    col: coordinate.col + 1,
                },
            }
        }
    }

    impl Shape for Cross {
        fn move_left(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let left = PosCoordinate {
                row: self.left.row,
                col: self.left.col - 1,
            };
            let top = PosCoordinate {
                row: self.top.row,
                col: self.top.col - 1,
            };
            let bottom = PosCoordinate {
                row: self.bottom.row,
                col: self.bottom.col - 1,
            };
            if left.col == 0
                || occupied.contains(&left)
                || occupied.contains(&top)
                || occupied.contains(&bottom)
            {
                return None;
            }
            Some(Box::new(Cross {
                left,
                top,
                bottom,
                right: self.centre,
                centre: self.left,
            }))
        }

        fn move_right(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let right = PosCoordinate {
                row: self.right.row,
                col: self.right.col + 1,
            };
            let top = PosCoordinate {
                row: self.top.row,
                col: self.top.col + 1,
            };
            let bottom = PosCoordinate {
                row: self.bottom.row,
                col: self.bottom.col + 1,
            };
            if right.col == MAX_COL
                || occupied.contains(&right)
                || occupied.contains(&top)
                || occupied.contains(&bottom)
            {
                return None;
            }
            Some(Box::new(Cross {
                right,
                top,
                bottom,
                left: self.centre,
                centre: self.right,
            }))
        }

        fn descend(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            // check left, bottom, right
            let left = PosCoordinate {
                row: self.left.row - 1,
                col: self.left.col,
            };
            let bottom = PosCoordinate {
                row: self.bottom.row - 1,
                col: self.bottom.col,
            };
            let right = PosCoordinate {
                row: self.right.row - 1,
                col: self.right.col,
            };
            if bottom.col == 0
                || occupied.contains(&left)
                || occupied.contains(&bottom)
                || occupied.contains(&right)
            {
                return None;
            }
            Some(Box::new(Cross {
                right,
                top: self.centre,
                bottom,
                left,
                centre: self.bottom,
            }))
        }

        fn get_coordinates(&self) -> Vec<PosCoordinate> {
            vec![self.left, self.centre, self.right, self.top, self.bottom]
        }

        fn get_highest(&self) -> PosCoordinate {
            self.top
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct L {
        bottom_left: PosCoordinate,
        bottom_centre: PosCoordinate,
        bottom_right: PosCoordinate,
        centre_right: PosCoordinate,
        top_right: PosCoordinate,
    }

    impl L {
        pub fn new(coordinate: PosCoordinate) -> Self {
            L {
                bottom_left: coordinate,
                bottom_centre: PosCoordinate {
                    col: coordinate.col + 1,
                    ..coordinate
                },
                bottom_right: PosCoordinate {
                    col: coordinate.col + 2,
                    ..coordinate
                },
                centre_right: PosCoordinate {
                    row: coordinate.row + 1,
                    col: coordinate.col + 2,
                },
                top_right: PosCoordinate {
                    row: coordinate.row + 2,
                    col: coordinate.col + 2,
                },
            }
        }
    }

    impl Shape for L {
        fn move_left(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let bottom_left = PosCoordinate {
                row: self.bottom_left.row,
                col: self.bottom_left.col - 1,
            };
            let top_right = PosCoordinate {
                row: self.top_right.row,
                col: self.top_right.col - 1,
            };
            let centre_right = PosCoordinate {
                row: self.centre_right.row,
                col: self.centre_right.col - 1,
            };
            if bottom_left.col == 0
                || occupied.contains(&bottom_left)
                || occupied.contains(&top_right)
                || occupied.contains(&centre_right)
            {
                return None;
            }
            Some(Box::new(L {
                bottom_left,
                bottom_centre: self.bottom_left,
                bottom_right: self.bottom_centre,
                centre_right,
                top_right,
            }))
        }

        fn move_right(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let bottom_right = PosCoordinate {
                row: self.bottom_right.row,
                col: self.bottom_right.col + 1,
            };
            let top_right = PosCoordinate {
                row: self.top_right.row,
                col: self.top_right.col + 1,
            };
            let centre_right = PosCoordinate {
                row: self.centre_right.row,
                col: self.centre_right.col + 1,
            };
            if bottom_right.col == MAX_COL
                || occupied.contains(&bottom_right)
                || occupied.contains(&top_right)
                || occupied.contains(&centre_right)
            {
                return None;
            }
            Some(Box::new(L {
                bottom_right,
                bottom_centre: self.bottom_right,
                bottom_left: self.bottom_centre,
                centre_right,
                top_right,
            }))
        }

        fn descend(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let bottom_right = PosCoordinate {
                row: self.bottom_right.row - 1,
                col: self.bottom_right.col,
            };
            let bottom_left = PosCoordinate {
                row: self.bottom_left.row - 1,
                col: self.bottom_left.col,
            };
            let bottom_centre = PosCoordinate {
                row: self.bottom_centre.row - 1,
                col: self.bottom_centre.col,
            };
            if bottom_right.row == 0
                || occupied.contains(&bottom_right)
                || occupied.contains(&bottom_left)
                || occupied.contains(&bottom_centre)
            {
                return None;
            }
            Some(Box::new(L {
                bottom_right,
                bottom_centre,
                bottom_left,
                centre_right: self.bottom_right,
                top_right: self.centre_right,
            }))
        }

        fn get_coordinates(&self) -> Vec<PosCoordinate> {
            vec![
                self.bottom_left,
                self.bottom_centre,
                self.bottom_right,
                self.centre_right,
                self.top_right,
            ]
        }

        fn get_highest(&self) -> PosCoordinate {
            self.top_right
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct VLine {
        top: PosCoordinate,
        centre_top: PosCoordinate,
        centre_bottom: PosCoordinate,
        bottom: PosCoordinate,
    }

    impl VLine {
        pub fn new(coordinate: PosCoordinate) -> Self {
            VLine {
                bottom: coordinate,
                centre_bottom: PosCoordinate {
                    row: coordinate.row + 1,
                    ..coordinate
                },
                centre_top: PosCoordinate {
                    row: coordinate.row + 2,
                    ..coordinate
                },
                top: PosCoordinate {
                    row: coordinate.row + 3,
                    ..coordinate
                },
            }
        }
    }

    impl Shape for VLine {
        fn move_left(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let new_self = VLine {
                bottom: PosCoordinate {
                    row: self.bottom.row,
                    col: self.bottom.col - 1,
                },
                centre_bottom: PosCoordinate {
                    row: self.centre_bottom.row,
                    col: self.centre_bottom.col - 1,
                },
                centre_top: PosCoordinate {
                    row: self.centre_top.row,
                    col: self.centre_top.col - 1,
                },
                top: PosCoordinate {
                    row: self.top.row,
                    col: self.top.col - 1,
                },
            };
            if new_self.bottom.col == 0
                || occupied.contains(&new_self.bottom)
                || occupied.contains(&new_self.centre_bottom)
                || occupied.contains(&new_self.centre_top)
                || occupied.contains(&new_self.top)
            {
                return None;
            }
            Some(Box::new(new_self))
        }

        fn move_right(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let new_self = VLine {
                bottom: PosCoordinate {
                    row: self.bottom.row,
                    col: self.bottom.col + 1,
                },
                centre_bottom: PosCoordinate {
                    row: self.centre_bottom.row,
                    col: self.centre_bottom.col + 1,
                },
                centre_top: PosCoordinate {
                    row: self.centre_top.row,
                    col: self.centre_top.col + 1,
                },
                top: PosCoordinate {
                    row: self.top.row,
                    col: self.top.col + 1,
                },
            };
            if new_self.bottom.col == MAX_COL
                || occupied.contains(&new_self.bottom)
                || occupied.contains(&new_self.centre_bottom)
                || occupied.contains(&new_self.centre_top)
                || occupied.contains(&new_self.top)
            {
                return None;
            }
            Some(Box::new(new_self))
        }

        fn descend(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let bottom = PosCoordinate {
                row: self.bottom.row - 1,
                col: self.bottom.col,
            };
            if bottom.row == 0 || occupied.contains(&bottom) {
                return None;
            }
            Some(Box::new(VLine {
                bottom,
                centre_bottom: self.bottom,
                centre_top: self.centre_bottom,
                top: self.centre_top,
            }))
        }

        fn get_coordinates(&self) -> Vec<PosCoordinate> {
            vec![self.top, self.centre_top, self.centre_bottom, self.bottom]
        }

        fn get_highest(&self) -> PosCoordinate {
            self.top
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Square {
        top_left: PosCoordinate,
        top_right: PosCoordinate,
        bottom_left: PosCoordinate,
        bottom_right: PosCoordinate,
    }

    impl Square {
        pub fn new(coordinate: PosCoordinate) -> Self {
            Square {
                top_left: PosCoordinate {
                    row: coordinate.row + 1,
                    ..coordinate
                },
                top_right: PosCoordinate {
                    row: coordinate.row + 1,
                    col: coordinate.col + 1,
                },
                bottom_left: coordinate,
                bottom_right: PosCoordinate {
                    col: coordinate.col + 1,
                    ..coordinate
                },
            }
        }
    }

    impl Shape for Square {
        fn move_left(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let top_left = PosCoordinate {
                row: self.top_left.row,
                col: self.top_left.col - 1,
            };
            let bottom_left = PosCoordinate {
                row: self.bottom_left.row,
                col: self.bottom_left.col - 1,
            };
            if top_left.col == 0 || occupied.contains(&top_left) || occupied.contains(&bottom_left)
            {
                return None;
            }
            Some(Box::new(Square {
                top_left,
                bottom_left,
                top_right: self.top_left,
                bottom_right: self.bottom_left,
            }))
        }

        fn move_right(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let top_right = PosCoordinate {
                row: self.top_right.row,
                col: self.top_right.col + 1,
            };
            let bottom_right = PosCoordinate {
                row: self.bottom_right.row,
                col: self.bottom_right.col + 1,
            };
            if top_right.col == MAX_COL
                || occupied.contains(&top_right)
                || occupied.contains(&bottom_right)
            {
                return None;
            }
            Some(Box::new(Square {
                top_right,
                bottom_right,
                top_left: self.top_right,
                bottom_left: self.bottom_right,
            }))
        }

        fn descend(&self, occupied: &HashSet<PosCoordinate>) -> Option<Box<dyn Shape>> {
            let bottom_right = PosCoordinate {
                row: self.bottom_right.row - 1,
                col: self.bottom_right.col,
            };
            let bottom_left = PosCoordinate {
                row: self.bottom_left.row - 1,
                col: self.bottom_left.col,
            };
            if bottom_right.row == 0
                || occupied.contains(&bottom_right)
                || occupied.contains(&bottom_left)
            {
                return None;
            }
            Some(Box::new(Square {
                top_left: self.bottom_left,
                bottom_left,
                top_right: self.bottom_right,
                bottom_right,
            }))
        }

        fn get_coordinates(&self) -> Vec<PosCoordinate> {
            vec![
                self.top_left,
                self.bottom_left,
                self.bottom_right,
                self.top_right,
            ]
        }

        fn get_highest(&self) -> PosCoordinate {
            self.top_right
        }
    }
}

fn get_next_shape(current: Rock, coordinate: PosCoordinate) -> Rock {
    match current.shape_type {
        ShapeType::HLine => Rock {
            shape: Box::new(Cross::new(coordinate)),
            shape_type: ShapeType::Cross,
        },
        ShapeType::Cross => Rock {
            shape: Box::new(L::new(coordinate)),
            shape_type: ShapeType::L,
        },
        ShapeType::L => Rock {
            shape: Box::new(VLine::new(coordinate)),
            shape_type: ShapeType::VLine,
        },
        ShapeType::VLine => Rock {
            shape: Box::new(Square::new(coordinate)),
            shape_type: ShapeType::Square,
        },
        ShapeType::Square => Rock {
            shape: Box::new(HLine::new(coordinate)),
            shape_type: ShapeType::HLine,
        },
    }
}

fn get_starting_coordinate(highest_row: u64) -> PosCoordinate {
    PosCoordinate {
        row: highest_row + 4,
        // always 2 from the left wall
        col: 3,
    }
}

fn solve_part1(input: &Input) -> u64 {
    solve(input, 2022)
}

// remove occupied coordinate below a certain row
fn prune_occupied(occupied: HashSet<PosCoordinate>, highest_row: u64) -> HashSet<PosCoordinate> {
    // find the highest row that is entirely occupied
    let mut highest_occupied = None;
    for row in (0..=highest_row).rev() {
        let row_complete = (1..=7).all(|col| occupied.contains(&PosCoordinate { row, col }));
        if row_complete {
            highest_occupied = Some(row);
            break;
        }
    }
    if let Some(highest) = highest_occupied {
        occupied.into_iter().filter(|&c| c.row >= highest).collect()
    } else {
        occupied
    }
}

fn solve(input: &Input, rock_count: u64) -> u64 {
    let mut jets = input.jets.iter().cycle();
    // initial shape
    let mut shape = Rock {
        shape: Box::new(HLine::new(PosCoordinate { row: 4, col: 3 })),
        shape_type: ShapeType::HLine,
    };
    let mut highest_row = 0;
    let mut occupied = HashSet::new();
    let mut time = Instant::now();
    // print_state(&occupied, &shape);
    for num in 0..rock_count {
        // initial move
        let direction = jets.next().unwrap();
        if let Some(next_shape) = match direction {
            Jet::Left => shape.shape.move_left(&occupied),
            Jet::Right => shape.shape.move_right(&occupied),
        } {
            shape.shape = next_shape;
        }

        // Descend rock
        while let Some(next_shape) = shape.shape.descend(&occupied) {
            shape.shape = next_shape;
            let direction = jets.next().unwrap();
            if let Some(next_shape) = match direction {
                Jet::Left => shape.shape.move_left(&occupied),
                Jet::Right => shape.shape.move_right(&occupied),
            } {
                shape.shape = next_shape;
            }
        }

        // shape stopped where it landed
        occupied.extend(shape.shape.get_coordinates());
        if shape.shape.get_highest().row > highest_row {
            highest_row = shape.shape.get_highest().row;
        }
        shape = get_next_shape(shape, get_starting_coordinate(highest_row));

        if num % 1_000_000 == 0 {
            occupied = prune_occupied(occupied, highest_row);
            println!(
                "Dropped {} rocks in {}ms. Occupied size is {}",
                num,
                time.elapsed().as_millis(),
                occupied.len()
            );
            time = Instant::now();
        }
    }
    highest_row
}

#[allow(dead_code)]
fn print_state(occupied: &HashSet<PosCoordinate>, shape: &Rock) {
    // highest point a rock can be
    let start = shape.shape.get_highest().row;
    let rock_coords: HashSet<PosCoordinate> = shape.shape.get_coordinates().into_iter().collect();
    for row in (0..=start).rev() {
        for col in 0..=8 {
            if row == 0 {
                if col == 0 || col == 8 {
                    print!("+");
                    continue;
                } else {
                    print!("-");
                    continue;
                }
            } else if col == 0 || col == 8 {
                print!("|");
                continue;
            }
            let c = PosCoordinate { row, col };
            if occupied.contains(&c) {
                print!("#");
            } else if rock_coords.contains(&c) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn solve_part2(input: &Input) -> u64 {
    solve(input, 1_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day17.test.txt"));
        let result = solve_part1(&input);
        assert_eq!(result, 3068);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day17.test.txt"));
        let result = solve_part2(&input);
        assert_eq!(result, 1514285714288);
        Ok(())
    }
}
