use std::collections::{VecDeque, HashSet, HashMap};
use std::fs::File;
use std::io::{BufReader, BufRead, Read};

use itertools::Itertools;
use reformation::Reformation;

fn main() {
    let f = File::open("input/day6.txt").unwrap();
    let reader = BufReader::new(f);
    let part = 2;
    let line = reader
        .lines()
        .next()
        .unwrap()
        .unwrap();
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
        let slice = &line[index..index+14];
        let mut seen: HashMap<char, usize> = HashMap::new();
        for (s_index, c) in slice.chars().enumerate() {
            match seen.get(&c) {
                Some(location) => {
                    index += location + 1;
                    break
                },
                None => {
                    seen.insert(c, s_index);
                },
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
    .tuple_windows()
    .enumerate()
    .map_while(|(index, (a, b, c, d))| {
        let mut unique = HashSet::new();
        unique.insert(a);
        unique.insert(b);
        unique.insert(c);
        unique.insert(d);
        if unique.len() == 4 {
            println!("index: {}", index);
            println!("{},{},{},{}", a, b, c, d);
            None
        } else {
            Some(index)
        }
    })
    .last()
    .unwrap() + 5
    // 1 for 0 indexing, 4 for the window size
}