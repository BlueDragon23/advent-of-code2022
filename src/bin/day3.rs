use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn main() {
    let f = File::open("input/day3.txt").unwrap();
    let reader = BufReader::new(f);
    let result = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect::<HashSet<char>>())
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            *chunk
                .next()
                .unwrap()
                .intersection(&chunk.next().unwrap())
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&chunk.next().unwrap())
                .next()
                .unwrap()
        })
        .map(|c| match c {
            'a'..='z' => c as u32 - 96,
            'A'..='Z' => c as u32 - 38,
            _ => panic!("Invalid"),
        })
        .sum::<u32>();

    println!("Part 1: {:?}", result);
    println!("Part 2: {:?}", result);
}

#[allow(dead_code)]
fn part1() {
    let f = File::open("input/day3.txt").unwrap();
    let reader = BufReader::new(f);
    let result = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let chars = line.chars().collect_vec();
            let (first, second) = chars.split_at(chars.len() / 2);
            let mut first_set: HashSet<char> = HashSet::new();
            first_set.extend(first);
            let mut second_set = HashSet::new();
            second_set.extend(second);
            *first_set
                .intersection(&second_set)
                .into_iter()
                .next()
                .unwrap()
        })
        .map(|c| match c {
            'a'..='z' => c as u32 - 96,
            'A'..='Z' => c as u32 - 38,
            _ => panic!("Invalid"),
        })
        .sum::<u32>();
    println!("Part 1: {}", result);
}
