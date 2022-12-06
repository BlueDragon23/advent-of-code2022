use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn main() {
    let f = File::open("input/day6.txt").unwrap();
    let reader = BufReader::new(f);
    let part = 2;
    let line = reader.lines().next().unwrap().unwrap();
    if part == 1 {
        let result = part_1(line);
        println!("Result: {:?}", result);
    } else {
        let result = part_2(line);
        println!("Result: {:?}", result);
    }
}

fn part_2(line: String) -> usize {
    let mut index = 0;
    while index < line.len() {
        let slice = &line[index..index + 14];
        let mut seen: HashMap<char, usize> = HashMap::new();
        for (s_index, c) in slice.chars().enumerate() {
            match seen.get(&c) {
                Some(location) => {
                    index += location + 1;
                    break;
                }
                None => {
                    seen.insert(c, s_index);
                }
            }
        }
        if seen.len() == 14 {
            // 14 for the window size
            return index + 14;
        }
    }
    panic!("Failed to find result");
}

fn part_1(input: String) -> usize {
    input
        .chars()
        .collect_vec()
        .windows(4)
        .enumerate()
        .map_while(|(index, window)| {
            let unique = window.into_iter().collect::<HashSet<_>>();
            if unique.len() == 4 {
                println!("index: {}", index);
                None
            } else {
                Some(index)
            }
        })
        .last()
        .unwrap()
        + 5
    // 1 for 0 indexing, 4 for the window size
}
