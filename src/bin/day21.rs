use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
    time::Instant,
};

use num::abs;

#[derive(Debug, Clone)]
pub struct Input<'a> {
    name: &'a str,
    value: Value<'a>,
}

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Constant(i64),
    Equation(Operation<'a>),
}

#[derive(Debug, Clone)]
pub struct Operation<'a> {
    left: &'a str,
    operator: Operator,
    right: &'a str,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Multiply,
    Subtract,
    Divide,
}

fn main() -> color_eyre::Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day21.txt"))?;
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input),
        time.elapsed().as_millis()
    );
    let time = Instant::now();
    println!(
        "Part 2: {} in {}ms",
        solve_part2(&input).unwrap(),
        time.elapsed().as_millis()
    );
    Ok(())
}

mod parsing {
    use crate::{Operation, Operator, Value};

    use super::Input;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take, take_till},
        combinator::map,
        sequence::{delimited, separated_pair, tuple},
        Finish, IResult,
    };

    fn parse_name(input: &str) -> IResult<&str, &str> {
        take_till(|c| c == ':')(input)
    }

    fn take1(input: &str) -> IResult<&str, &str> {
        take(1usize)(input)
    }

    fn parse_operator(input: &str) -> IResult<&str, Operator> {
        delimited(
            tag(" "),
            map(take1, |c| match c {
                "+" => Operator::Add,
                "*" => Operator::Multiply,
                "-" => Operator::Subtract,
                "/" => Operator::Divide,
                _ => panic!("Invalid character"),
            }),
            tag(" "),
        )(input)
    }

    fn parse_operation(input: &str) -> IResult<&str, Operation> {
        map(
            tuple((take(4usize), parse_operator, take(4usize))),
            |(left, operator, right)| Operation {
                left,
                operator,
                right,
            },
        )(input)
    }

    fn parse_value(input: &str) -> IResult<&str, Value> {
        alt((
            map(nom::character::complete::i64, Value::Constant),
            map(parse_operation, Value::Equation),
        ))(input)
    }

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(
            separated_pair(parse_name, tag(": "), parse_value),
            |(name, value)| Input { name, value },
        )(input)
    }

    pub fn parse_input(input: &str) -> color_eyre::Result<Vec<Input>> {
        Ok(input
            .lines()
            .map(|line| parse_line(line).finish().unwrap().1)
            .collect())
    }
}

fn solve_part1(input: &[Input]) -> i64 {
    let monkey_values: HashMap<&str, Value> = input
        .iter()
        .map(|monkey| (monkey.name, monkey.value.clone()))
        .collect();
    let human_value = if let Value::Constant(x) = monkey_values.get("humn").unwrap() {
        x
    } else {
        panic!("Invalid value");
    };
    find_value_for_monkey("root", &monkey_values, *human_value).unwrap()
}

fn find_value_for_monkey(
    name: &str,
    monkeys: &HashMap<&str, Value>,
    human_value: i64,
) -> Option<i64> {
    if name == "humn" {
        return Some(human_value);
    }
    match monkeys.get(name).unwrap() {
        Value::Constant(x) => Some(*x),
        Value::Equation(operation) => {
            let left = find_value_for_monkey(operation.left, monkeys, human_value)?;
            let right = find_value_for_monkey(operation.right, monkeys, human_value)?;
            match operation.operator {
                Operator::Add => left.checked_add(right),
                Operator::Multiply => left.checked_mul(right),
                Operator::Subtract => left.checked_sub(right),
                Operator::Divide => left.checked_div(right),
            }
        }
    }
}

fn solve_part2(input: &[Input]) -> Option<i64> {
    let monkey_values: HashMap<&str, Value> = input
        .iter()
        .map(|monkey| (monkey.name, monkey.value.clone()))
        .collect();
    let human_value = if let Value::Constant(x) = monkey_values.get("humn").unwrap() {
        *x
    } else {
        panic!("Invalid value");
    };
    let root = monkey_values.get("root").unwrap();
    let left_child: &str;
    let right_child: &str;
    let left_value: i64;
    let right_value: i64;
    let is_left = match root {
        Value::Constant(_) => unreachable!("This never happens"),
        Value::Equation(operation) => {
            left_child = operation.left;
            right_child = operation.right;
            println!(
                "Test output is {} and {}",
                find_value_for_monkey(operation.left, &monkey_values, 3555057453229)?,
                find_value_for_monkey(operation.right, &monkey_values, 3555057453231)?
            );
            left_value = find_value_for_monkey(operation.left, &monkey_values, human_value)?;
            right_value = find_value_for_monkey(operation.right, &monkey_values, human_value)?;
            let modified_left =
                find_value_for_monkey(operation.left, &monkey_values, human_value - 100)?;
            let modified_right =
                find_value_for_monkey(operation.right, &monkey_values, human_value - 100)?;
            dbg!(left_value, modified_left, right_value, modified_right);
            if left_value != modified_left {
                Some(true)
            } else if right_value != modified_right {
                Some(false)
            } else {
                None
            }
        }
    };
    assert!(is_left.is_some());
    match is_left {
        Some(left) => {
            if left {
                Some(binary_search(left_child, &monkey_values, right_value))
            } else {
                Some(binary_search(right_child, &monkey_values, left_value))
            }
        }
        None => panic!("both sides are on the critical path"),
    }
}

fn binary_search(name: &str, monkeys: &HashMap<&str, Value>, target: i64) -> i64 {
    dbg!(target);
    // numbers found experimentally
    let mut min = -100_000_000_000_000;
    let mut max = 100_000_000_000_000;
    let mut human_value = 0;
    let output_base = find_value_for_monkey(name, monkeys, human_value).unwrap();
    let output_increasing = find_value_for_monkey(name, monkeys, 100_000_000_000_000).unwrap();
    let output_decreasing = find_value_for_monkey(name, monkeys, -100_000_000_000_000).unwrap();
    let (increasing_from_zero, positive_correlation) = match (
        output_increasing.cmp(&target),
        output_base.cmp(&target),
        output_decreasing.cmp(&target),
    ) {
        (Greater, Greater, Less) => (false, true),
        (Greater, Less, Less) => (true, true),
        (Less, Less, Greater) => (false, false),
        (Less, Greater, Greater) => (true, false),
        (Equal, _, _) => {
            return max / 4;
        }
        (_, Equal, _) => {
            return human_value;
        }
        (_, _, Equal) => {
            return min / 4;
        }
        comparisons => panic!("Found {:?}", comparisons),
    };
    dbg!(increasing_from_zero);
    if increasing_from_zero {
        min = 0;
    } else {
        max = 0;
    }
    let mut output = Some(output_base);
    while output != Some(target) {
        match (increasing_from_zero, positive_correlation) {
            (true, true) => {
                if output.is_none() {
                    max = human_value;
                } else if output < Some(target) {
                    min = human_value;
                } else {
                    max = human_value;
                }
            }
            (true, false) => {
                if output.is_none() || output < Some(target) {
                    // we overflowed
                    max = human_value;
                } else {
                    min = human_value;
                }
            }
            (false, true) => todo!(),
            (false, false) => todo!(),
        }
        // dbg!(min, max);
        if human_value == (min + max) / 2 {
            panic!("Infinite looping");
        }
        human_value = (min + max) / 2;
        output = find_value_for_monkey(name, monkeys, human_value);
        // dbg!(output);
    }
    human_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day21.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 152);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day21.test.txt"))?;
        let result = solve_part2(&input).unwrap();
        assert_eq!(result, 301);
        Ok(())
    }
}
