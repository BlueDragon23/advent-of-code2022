use std::fs::File;
use std::io::{BufRead, BufReader};

use advent_of_code2022::Range;
use reformation::Reformation;

#[derive(Reformation, Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[reformation(r"{first},{second}")]
struct Input {
    first: Range,
    second: Range,
}

fn main() {
    let f = File::open("input/day4.txt").unwrap();
    let reader = BufReader::new(f);
    let result = reader
        .lines()
        .map(|line| Input::parse(&line.unwrap()).unwrap())
        .filter(|input| input.first.overlap(input.second))
        .count();

    println!("Result: {}", result);
}
