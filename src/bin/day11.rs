use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    let input = include_str!("../../input/example.txt");
    let input_processed = parse_input(input);
    solve_part1(&input_processed);
    solve_part2(&input_processed);
    Ok(())
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .collect_vec()
}

fn solve_part1(input: &Vec<str>) -> bool {
    input
        .lines();
    true
}

fn solve_part2(input: &Vec<str>) -> bool {
    input
        .lines();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/example.test.txt");
        let result = solve_part1(input);
        assert!(result == true);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/example.test.txt");
        let result = solve_part2(input);
        assert!(result == true);
    }
}