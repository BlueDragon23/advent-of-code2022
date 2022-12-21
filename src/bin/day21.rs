use std::{
    collections::HashMap,
    time::Instant, fmt::Display,
};

#[derive(Debug, Clone)]
pub struct Input<'a> {
    name: &'a str,
    value: Value<'a>,
}

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Constant(i64),
    Equation(Operation<'a>),
    Name(&'a str)
}

impl <'a> Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Constant(x) => write!(f, "{}", x),
            Value::Equation(e) => write!(f, "({})", e),
            Value::Name(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Operation<'a> {
    left: Box<Value<'a>>,
    operator: Operator,
    right: Box<Value<'a>>,
}

impl <'a> Display for Operation<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Multiply,
    Subtract,
    Divide,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Operator::Add => " + ",
            Operator::Multiply => " * ",
            Operator::Subtract => " - ",
            Operator::Divide => " / ",
        })
    }
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
                left: Box::new(Value::Name(left)),
                operator,
                right: Box::new(Value::Name(right)),
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
    if let Value::Constant(x) = find_value_for_monkey("root", &monkey_values, 1).unwrap() {
        x
    } else {
        panic!("Invalid result")
    }
}

fn find_value_for_monkey<'a>(
    name: &str,
    monkeys: &HashMap<&str, Value>,
    part: u32
) -> Option<Value<'a>> {
    if name == "humn" && part == 2 {
        return Some(Value::Name("humn"));
    }
    match monkeys.get(name).unwrap() {
        Value::Constant(x) => Some(Value::Constant(*x)),
        Value::Equation(operation) => {
            if let Value::Name(left_name) = operation.left.as_ref() {
                if let Value::Name(right_name) = operation.right.as_ref() {
                    let left = find_value_for_monkey(left_name, monkeys, part)?;
                    let right = find_value_for_monkey(right_name, monkeys, part)?;
                    match (left, right) {
                        (Value::Constant(l), Value::Constant(r)) => {
                            match operation.operator {
                                Operator::Add => l.checked_add(r),
                                Operator::Multiply => l.checked_mul(r),
                                Operator::Subtract => l.checked_sub(r),
                                Operator::Divide => l.checked_div(r),
                            }.map(|result| Value::Constant(result))
                        },
                        (value_left, value_right) => Some(Value::Equation(Operation {
                            left: Box::new(value_left),
                            operator: operation.operator,
                            right: Box::new(value_right)
                        }))
                    }
                } else {
                    unreachable!("Not a valid input")
                }
            } else {
                unreachable!("Not a valid input")
            }
        }
        _ => unreachable!("There are no names in the input")
    }
}

fn solve_part2(input: &[Input]) -> Option<i64> {
    let monkey_values: HashMap<&str, Value> = input
        .iter()
        .map(|monkey| (monkey.name, monkey.value.clone()))
        .collect();
    let end_value = find_value_for_monkey("root", &monkey_values, 2);
    println!("{}", end_value.unwrap());
    Some(1)
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
