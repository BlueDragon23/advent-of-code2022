use std::fs::File;
use std::io::BufReader;

use advent_of_code2022::group_file_by_empty_lines;
use itertools::Itertools;

fn main() {
    let f = File::open("input/day1.txt").unwrap();
    let reader = BufReader::new(f);
    let result = group_file_by_empty_lines(reader)
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .collect::<Vec<i32>>();

    println!("Part 1: {:?}", result.clone().into_iter().max().unwrap());
    println!(
        "Part 2: {:?}",
        result.into_iter().sorted().rev().take(3).sum::<i32>()
    );
}
