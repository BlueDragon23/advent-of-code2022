use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::delimited,
    Finish, IResult,
};

type Pair = (Value, Value);

#[derive(Debug, Clone)]
enum Value {
    Number(u32),
    List(Vec<Value>),
}

fn main() -> color_eyre::Result<()> {
    let input = parse_input_1(include_str!("../../input/day13.txt"));
    println!("Part 1: {}", solve_part1(&input));
    let input_2 = parse_input_2(include_str!("../../input/day13.txt"));
    println!("Part 2: {}", solve_part2(input_2));
    Ok(())
}

fn parse_number(input: &str) -> IResult<&str, Value> {
    map(nom::character::complete::u32, Value::Number)(input)
}

fn parse_list(input: &str) -> IResult<&str, Value> {
    map(
        delimited(tag("["), separated_list0(tag(","), parse_value), tag("]")),
        Value::List,
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((parse_list, parse_number))(input)
}

fn parse_input_1(input: &str) -> Vec<Pair> {
    input
        .split("\n\n")
        .map(|group| {
            let (first, last) = group.split_once('\n').unwrap();
            (
                all_consuming(parse_value)(first).finish().unwrap().1,
                all_consuming(parse_value)(last).finish().unwrap().1,
            )
        })
        .collect()
}

fn solve_part1(input: &[Pair]) -> usize {
    let result = input
        .iter()
        .enumerate()
        .map(|(index, pair)| (index, validate_pair(&pair.0, &pair.1)))
        .filter(|(_, result)| *result == std::cmp::Ordering::Less)
        // 1 indexed indices
        .map(|(index, _)| index + 1)
        .sum();
    result
}

fn validate_pair(left: &Value, right: &Value) -> std::cmp::Ordering {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => l.cmp(r),
        (Value::List(l), Value::List(r)) => {
            let ordering = l
                .iter()
                .zip(r.iter())
                .map(|(a, b)| validate_pair(a, b))
                .find(|&ord| ord != std::cmp::Ordering::Equal);
            if let Some(result) = ordering {
                result
            } else {
                l.len().cmp(&r.len())
            }
        }
        (list @ Value::List(_), Value::Number(num)) => {
            validate_pair(list, &Value::List(vec![Value::Number(*num)]))
        }
        (Value::Number(num), list @ Value::List(_)) => {
            validate_pair(&Value::List(vec![Value::Number(*num)]), list)
        }
    }
}

fn parse_input_2(input: &str) -> Vec<Value> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| all_consuming(parse_value)(line).finish().unwrap().1)
        .collect()
}

fn solve_part2(mut input: Vec<Value>) -> usize {
    // divider packets
    input.push(Value::List(vec![Value::List(vec![Value::Number(2)])]));
    input.push(Value::List(vec![Value::List(vec![Value::Number(6)])]));
    input.sort_by(validate_pair);
    input
        .iter()
        .enumerate()
        .filter(|(_, value)| match value {
            Value::List(child) => {
                if child.len() == 1 {
                    match &child[0] {
                        Value::List(grandchild) => {
                            if grandchild.len() == 1 {
                                matches!(&grandchild[0], Value::Number(2) | Value::Number(6))
                            } else {
                                false
                            }
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            _ => false,
        })
        .map(|(index, _)| index + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parse_input_1(include_str!("../../input/day13.test.txt"));
        let result = solve_part1(&input);
        assert_eq!(result, 13);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parse_input_2(include_str!("../../input/day13.test.txt"));
        let result = solve_part2(input);
        assert_eq!(result, 140);
        Ok(())
    }
}
