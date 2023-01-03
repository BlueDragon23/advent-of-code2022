use std::{collections::VecDeque, time::Instant};

use itertools::Itertools;
use num::pow;

#[derive(Debug, Clone)]
pub struct Input {
    snafu: Vec<i64>,
}

fn main() -> color_eyre::Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day25.txt"))?;
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input),
        time.elapsed().as_millis()
    );
    Ok(())
}

mod parsing {
    use super::Input;
    use nom::{bytes::complete::take, combinator::map, multi::many1, Finish, IResult};

    fn take1(input: &str) -> IResult<&str, &str> {
        take(1usize)(input)
    }

    fn parse_digit(input: &str) -> IResult<&str, i64> {
        map(take1, |c| match c {
            "2" => 2,
            "1" => 1,
            "0" => 0,
            "-" => -1,
            "=" => -2,
            _ => panic!("Unexpected char"),
        })(input)
    }

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(many1(parse_digit), |digits| Input { snafu: digits })(input)
    }

    pub fn parse_input(input: &str) -> color_eyre::Result<Vec<Input>> {
        Ok(input
            .lines()
            .map(|line| parse_line(line).finish().unwrap().1)
            .collect())
    }
}

fn snafu_to_decimal(snafu: &[i64]) -> i64 {
    snafu
        .iter()
        .rev()
        .enumerate()
        .fold(0, |total, (power, digit)| total + digit * pow(5, power))
}

fn digit_to_snafu(number: i64) -> char {
    match number {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!("Invalid snafu digit {}", number),
    }
}

fn decimal_to_snafu(number: i64) -> String {
    let mut string: VecDeque<char> = VecDeque::new();
    let mut next = number;
    while next != 0 {
        let mut remainder = next % 5;
        next /= 5;
        if remainder > 2 {
            remainder -= 5;
            next += 1;
        }
        string.push_front(digit_to_snafu(remainder));
    }
    string.iter().join("")
}

fn solve_part1(input: &[Input]) -> String {
    let result: i64 = input
        .iter()
        .map(|input| snafu_to_decimal(&input.snafu))
        .sum();
    decimal_to_snafu(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_to_decimal() {
        let input = vec![2, -2, 1];
        let result = snafu_to_decimal(&input);
        assert_eq!(result, 41)
    }

    #[test]
    fn test_decimal_to_snafu() {
        let data = vec![(1, "1"), (3, "1="), (5, "10"), (9, "2-"), (15, "1=0")];
        for (input, expected) in data {
            let actual = decimal_to_snafu(input);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day25.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, "2=-1=0");
        Ok(())
    }
}
