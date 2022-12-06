use std::fs::File;
use std::io::BufReader;
use std::io::{BufRead, Lines};

use itertools::Itertools;
use reformation::Reformation;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Coordinate {
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
    row_count: usize,
    col_count: usize,
) -> Vec<Coordinate> {
    let mut adj = vec![];
    if coordinate.row != 0 {
        adj.push(Coordinate {
            row: coordinate.row - 1,
            col: coordinate.col,
        });
    }
    if coordinate.row != row_count - 1 {
        adj.push(Coordinate {
            row: coordinate.row + 1,
            col: coordinate.col,
        });
    }
    if coordinate.col != 0 {
        adj.push(Coordinate {
            row: coordinate.row,
            col: coordinate.col - 1,
        });
    }
    if coordinate.col != col_count - 1 {
        adj.push(Coordinate {
            row: coordinate.row,
            col: coordinate.col + 1,
        });
    }
    adj
}

pub fn get_adjacent_points_diagonal(
    coordinate: Coordinate,
    row_count: usize,
    col_count: usize,
) -> Vec<Coordinate> {
    let mut adj = get_adjacent_points(coordinate, row_count, col_count);
    if coordinate.row != 0 && coordinate.col != 0 {
        adj.push(Coordinate {
            row: coordinate.row - 1,
            col: coordinate.col - 1,
        });
    }
    if coordinate.row != row_count - 1 && coordinate.col != col_count - 1 {
        adj.push(Coordinate {
            row: coordinate.row + 1,
            col: coordinate.col + 1,
        });
    }
    if coordinate.col != 0 && coordinate.row != row_count - 1 {
        adj.push(Coordinate {
            row: coordinate.row + 1,
            col: coordinate.col - 1,
        });
    }
    if coordinate.col != col_count - 1 && coordinate.row != 0 {
        adj.push(Coordinate {
            row: coordinate.row - 1,
            col: coordinate.col + 1,
        });
    }
    adj
}

pub fn print_matrix(matrix: &Vec<Vec<u32>>) {
    for line in matrix {
        println!("{}", line.into_iter().join(""));
    }
    println!("");
}
