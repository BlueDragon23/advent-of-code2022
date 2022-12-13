use nom::{IResult, bytes::complete::{tag}, sequence::delimited, branch::alt, combinator::{map, all_consuming}, Finish, multi::{many1, separated_list1, separated_list0}};

type Pair = (Packet, Packet);

#[derive(Debug, Clone)]
struct Packet {
    value: Value
}

#[derive(Debug, Clone)]
enum Value {
    Number(u32),
    List(Vec<Value>)
}

fn main() -> color_eyre::Result<()> {
    let input = parse_input(include_str!("../../input/day13.txt"));
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
    Ok(())
}

fn parse_number(input: &str) -> IResult<&str, Value> {
    map(nom::character::complete::u32, Value::Number)(input)
}

fn parse_list(input: &str) -> IResult<&str, Value> {
    map(delimited(tag("["), separated_list0(tag(","), parse_value), tag("]")), Value::List)(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((parse_list, parse_number))(input)
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    map(parse_value, |value| Packet {value})(input)
}

fn parse_input(input: &str) -> Vec<Pair> {
    input.split("\n\n").map(|group| {
        let (first, last) = group.split_once('\n').unwrap();
        (all_consuming(parse_packet)(first).finish().unwrap().1, all_consuming(parse_packet)(last).finish().unwrap().1)
    }).collect()
}

fn solve_part1(input: &[Pair]) -> u32 {
    dbg!(&input[0]);
    1
}

fn solve_part2(input: &[Pair]) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parse_input(include_str!("../../input/day13.test.txt"));
        let result = solve_part1(&input);
        assert!(result == 1);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parse_input(include_str!("../../input/day13.test.txt"));
        let result = solve_part2(&input);
        assert!(result == 1);
        Ok(())
    }
}
