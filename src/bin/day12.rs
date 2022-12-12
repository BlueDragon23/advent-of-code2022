use itertools::Itertools;

struct Input {

}

fn main() -> color_eyre::Result<()> {
    let input = include_str!("../../input/day12.txt");
    let input_processed = parse_input(input)?;
    println!("Part 1: {}", solve_part1(&input_processed));
    println!("Part 2: {}", solve_part2(&input_processed));
    Ok(())
}

fn parse_input(input: &str) -> color_eyre::Result<Vec<Input>> {
    input
        .lines()
        .map(|line| Ok(Input{}))
        .collect()
}

fn solve_part1(input: &Vec<Input>) -> u32 {
    1
}

fn solve_part2(input: &Vec<Input>) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day12.test.txt");
        let input_processed = parse_input(input).unwrap();
        let result = solve_part1(&input_processed);
        assert!(result == 1);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day12.test.txt");
        let input_processed = parse_input(input).unwrap();
        let result = solve_part2(&input_processed);
        assert!(result == 1);
    }
}