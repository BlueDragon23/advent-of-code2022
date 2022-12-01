use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;

fn main() {
    let f = File::open("input/day1.txt").unwrap();
    let reader = BufReader::new(f);
    let (result, _count) = reader
        .lines()
        .map(|line| line.unwrap())
        .fold((vec![0], 0), |(mut result, index), line| {
            if line.trim().is_empty() {
                result.push(0);
                (result, index + 1)
            } else {
                result[index] += line.parse::<i32>().unwrap();
                (result, index)
            }
        });

    println!("{:?}", result.into_iter().sorted().rev().take(3).sum::<i32>());
}