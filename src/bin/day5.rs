use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use reformation::Reformation;

#[derive(Reformation, Debug, Clone, Copy)]
#[reformation(r"move {count} from {source} to {destination}")]
struct Command {
    count: u32,
    source: usize,
    destination: usize,
}

fn main() {
    let f = File::open("input/day5.txt").unwrap();
    let reader = BufReader::new(f);
    let part = 2;
    let result = reader
        .lines()
        .skip(10)
        .map(|line| Command::parse(&line.unwrap()).unwrap())
        .fold(get_initial_state(), |mut state, command| {
            if part == 1 {
                for _ in 0..command.count {
                    let char = state[command.source].pop().unwrap();
                    state[command.destination].push(char);
                }
            } else {
                let mut transfer = VecDeque::new();
                for _ in 0..command.count {
                    transfer.push_back(state[command.source].pop().unwrap());
                }
                for _ in 0..command.count {
                    state[command.destination].push(transfer.pop_back().unwrap());
                }
            }
            state
        })
        .into_iter()
        // ignore that dummy vector
        .skip(1)
        .map(|vec| *vec.last().unwrap())
        .join("");

    println!("Result: {:?}", result);
}

fn get_initial_state() -> Vec<Vec<char>> {
    vec![
        vec![], // dummy vec for 0 indexing
        vec!['D', 'L', 'V', 'T', 'M', 'H', 'F'],
        vec!['H', 'Q', 'G', 'J', 'C', 'T', 'N', 'P'],
        vec!['R', 'S', 'D', 'M', 'P', 'H'],
        vec!['L', 'B', 'V', 'F'],
        vec!['N', 'H', 'G', 'L', 'Q'],
        vec!['W', 'B', 'D', 'G', 'R', 'M', 'P'],
        vec!['G', 'M', 'N', 'R', 'C', 'H', 'L', 'Q'],
        vec!['C', 'L', 'W'],
        vec!['R', 'D', 'L', 'Q', 'J', 'Z', 'M', 'T'],
    ]
}
