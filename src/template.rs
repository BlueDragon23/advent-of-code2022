use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    let input = include_str!("../../input/example.txt");
    solve_part1(input);
    solve_part2(input);
    Ok(())
}

fn solve_part1(input: &str) -> bool {
    input
        .lines();
    true
}

fn solve_part2(input: &str) -> bool {
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