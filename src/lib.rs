use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::{BufRead, Lines};

use itertools::Itertools;
use reformation::Reformation;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub row: i32,
    pub col: i32,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PositiveCoordinate {
    pub row: usize,
    pub col: usize,
}

#[derive(Reformation, Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[reformation("{lower}-{upper}")]
pub struct Range {
    pub lower: i32,
    pub upper: i32,
}

impl Range {
    pub fn is_subrange(&self, other: Range) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }

    pub fn overlap(&self, other: Range) -> bool {
        (self.lower >= other.lower && self.lower <= other.upper)
            || (self.upper <= other.upper && self.upper >= other.lower)
            || self.is_subrange(other)
            || other.is_subrange(*self)
    }
}

// Example union input
#[derive(Reformation, Eq, PartialEq, Debug)]
#[allow(dead_code)]
enum Ant {
    #[reformation(r"Queen\({}\)")]
    Queen(String),
    #[reformation(r"Worker\({}\)")]
    Worker(i32),
    #[reformation(r"Warrior")]
    Warrior,
}

// Example struct input
#[derive(Reformation, Debug)]
#[reformation(r"{year}-{month}-{day} {hour}:{minute}")]
#[allow(dead_code)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

pub fn group_file_by_empty_lines(reader: BufReader<File>) -> Vec<Vec<String>> {
    reader
        .lines()
        .map(|line| line.unwrap())
        .fold(vec![vec![]], |mut result, line| {
            if line.trim().is_empty() {
                result.push(Vec::new());
                result
            } else {
                result.last_mut().unwrap().push(line);
                result
            }
        })
}

// Create a method for parsing a line of ints
pub fn parse_line_to_num(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec()
}

// Create a method for parsing lines of a file to ints
pub fn parse_lines_to_nums(lines: Lines<BufReader<File>>) -> Vec<i32> {
    lines
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect_vec()
}

// #[allow(dead_code)]
// fn parse_dates(reader: BufReader<File>) -> Vec<Date> {
//     parse_lines_to_struct::<Date>(reader)
// }

// // Create a method for parsing lines of a file to a particular struct using reformation
// #[allow(dead_code)]
// pub fn parse_lines_to_struct<'a, T: Reformation<'a>>(reader: BufReader<File>) -> Vec<T> {
//     reader
//         .lines()
//         .map(|line| T::parse(&line.unwrap()).unwrap())
//         .collect_vec()
// }

pub fn get_adjacent_points(
    coordinate: Coordinate,
    min_row: i32,
    min_col: i32,
    max_row: i32,
    max_col: i32,
) -> Vec<Coordinate> {
    let mut adj = vec![];
    if coordinate.row != min_row {
        adj.push(Coordinate {
            row: coordinate.row - 1,
            col: coordinate.col,
        });
    }
    if coordinate.row != max_row - 1 {
        adj.push(Coordinate {
            row: coordinate.row + 1,
            col: coordinate.col,
        });
    }
    if coordinate.col != min_col {
        adj.push(Coordinate {
            row: coordinate.row,
            col: coordinate.col - 1,
        });
    }
    if coordinate.col != max_col - 1 {
        adj.push(Coordinate {
            row: coordinate.row,
            col: coordinate.col + 1,
        });
    }
    adj
}

pub fn get_adjacent_points_diagonal(
    coordinate: Coordinate,
    min_row: i32,
    min_col: i32,
    max_row: i32,
    max_col: i32,
) -> Vec<Coordinate> {
    let mut adj = get_adjacent_points(coordinate, min_row, min_col, max_row, max_col);
    if coordinate.row != min_row && coordinate.col != min_col {
        adj.push(Coordinate {
            row: coordinate.row - 1,
            col: coordinate.col - 1,
        });
    }
    if coordinate.row != max_row - 1 && coordinate.col != max_col - 1 {
        adj.push(Coordinate {
            row: coordinate.row + 1,
            col: coordinate.col + 1,
        });
    }
    if coordinate.col != min_col && coordinate.row != max_row - 1 {
        adj.push(Coordinate {
            row: coordinate.row + 1,
            col: coordinate.col - 1,
        });
    }
    if coordinate.col != max_col - 1 && coordinate.row != min_row {
        adj.push(Coordinate {
            row: coordinate.row - 1,
            col: coordinate.col + 1,
        });
    }
    adj
}

pub fn print_matrix(matrix: &Vec<Vec<u32>>) {
    for line in matrix {
        println!("{}", line.iter().join(""));
    }
    println!();
}

pub fn print_coordinates(matrix: &[Coordinate], origin_top_left: bool) {
    let min_row = matrix.iter().map(|c| c.row).min().unwrap();
    let max_row = matrix.iter().map(|c| c.row).max().unwrap();
    let min_col = matrix.iter().map(|c| c.col).min().unwrap();
    let max_col = matrix.iter().map(|c| c.col).max().unwrap();
    let row_iter = if origin_top_left {
        (min_row..=max_row).collect_vec()
    } else {
        (min_row..=max_row).rev().collect_vec()
    };
    for row in row_iter {
        for col in min_col..=max_col {
            let c = Coordinate { row, col };
            if matrix.contains(&c) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
