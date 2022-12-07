use std::fs::File;
use std::io::{BufRead, BufReader};

use reformation::Reformation;

#[derive(Reformation, Eq, PartialEq, Debug, Clone, Copy)]
#[reformation(r"{opponent} {me}")]
struct Input {
    opponent: Choice,
    me: DesiredResult,
}

#[derive(Reformation, Eq, PartialEq, Debug, Clone, Copy)]
enum Choice {
    #[reformation(r"A")]
    Rock,
    #[reformation(r"B")]
    Paper,
    #[reformation(r"C")]
    Scissors,
}

#[derive(Reformation, Eq, PartialEq, Debug)]
enum MeInput {
    #[reformation(r"X")]
    Rock,
    #[reformation(r"Y")]
    Paper,
    #[reformation(r"Z")]
    Scissors,
}

#[derive(Reformation, Eq, PartialEq, Debug, Clone, Copy)]
enum DesiredResult {
    #[reformation(r"X")]
    Lose,
    #[reformation(r"Y")]
    Draw,
    #[reformation(r"Z")]
    Win,
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

fn main() {
    let f = File::open("input/day2.txt").unwrap();
    let reader = BufReader::new(f);
    let result: i32 = reader
        .lines()
        .map(|line| Input::parse(&line.unwrap()).unwrap())
        .map(get_score)
        .sum();

    println!("Part: {:?}", result);
}

// fn get_score(input: Input) -> i32 {
//     let choice = match input.me {
//         MeInput::Rock => 1,
//         MeInput::Paper => 2,
//         MeInput::Scissors => 3,
//     };
//     let result = match who_wins(input) {
//         GameResult::Win => 6,
//         GameResult::Lose => 0,
//         GameResult::Draw => 3,
//     };
//     choice + result
// }

fn get_score(input: Input) -> i32 {
    let choice = get_choice(input);
    let choice_score = match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };
    let result = match who_wins(input.opponent, choice) {
        GameResult::Win => 6,
        GameResult::Lose => 0,
        GameResult::Draw => 3,
    };
    choice_score + result
}

fn get_choice(input: Input) -> Choice {
    match input.opponent {
        Choice::Rock => match input.me {
            DesiredResult::Lose => Choice::Scissors,
            DesiredResult::Draw => Choice::Rock,
            DesiredResult::Win => Choice::Paper,
        },
        Choice::Paper => match input.me {
            DesiredResult::Lose => Choice::Rock,
            DesiredResult::Draw => Choice::Paper,
            DesiredResult::Win => Choice::Scissors,
        },
        Choice::Scissors => match input.me {
            DesiredResult::Lose => Choice::Paper,
            DesiredResult::Draw => Choice::Scissors,
            DesiredResult::Win => Choice::Rock,
        },
    }
}

fn who_wins(opponent: Choice, me: Choice) -> GameResult {
    match opponent {
        Choice::Rock => match me {
            Choice::Rock => GameResult::Draw,
            Choice::Paper => GameResult::Win,
            Choice::Scissors => GameResult::Lose,
        },
        Choice::Paper => match me {
            Choice::Rock => GameResult::Lose,
            Choice::Paper => GameResult::Draw,
            Choice::Scissors => GameResult::Win,
        },
        Choice::Scissors => match me {
            Choice::Rock => GameResult::Win,
            Choice::Paper => GameResult::Lose,
            Choice::Scissors => GameResult::Draw,
        },
    }
}
