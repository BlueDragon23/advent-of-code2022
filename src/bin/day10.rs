use advent_of_code2022::{print_coordinates, Coordinate};
use itertools::Itertools;
use reformation::Reformation;

#[derive(Reformation, Debug, Clone)]
enum Instruction {
    #[reformation("addx {}")]
    Add(i32),
    #[reformation("noop")]
    Noop,
}

const WIDTH: i32 = 40;

fn main() -> color_eyre::Result<()> {
    let input = include_str!("../../input/day10.txt");
    let instructions = parse_input(input);
    println!("{}", solve_part1(&instructions));
    print_coordinates(&solve_part2(&instructions), true);
    Ok(())
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::parse(line).unwrap())
        .collect_vec()
}

fn solve_part1(input: &Vec<Instruction>) -> i32 {
    let mut cycle = 1;
    let mut register = 1;
    let mut result = 0;
    for instruction in input {
        match instruction {
            Instruction::Add(count) => {
                result += add_value(cycle, register);
                cycle += 1;
                result += add_value(cycle, register);
                cycle += 1;
                register += count;
            }
            Instruction::Noop => {
                result += add_value(cycle, register);
                cycle += 1;
            }
        }
    }
    result
}

fn add_value(cycle: i32, register: i32) -> i32 {
    if (cycle + 20) % 40 == 0 {
        return cycle * register;
    }
    0
}

struct State {
    results: Vec<Coordinate>,
    cycle: i32,
    register: i32,
}

fn solve_part2(input: &Vec<Instruction>) -> Vec<Coordinate> {
    input
        .iter()
        .fold(
            State {
                results: vec![],
                cycle: 1,
                register: 1,
            },
            |mut state, instruction| {
                match instruction {
                    Instruction::Add(count) => {
                        draw_pixel(state.cycle, state.register)
                            .map(|coord| state.results.push(coord));
                        state.cycle += 1;
                        draw_pixel(state.cycle, state.register)
                            .map(|coord| state.results.push(coord));
                        state.cycle += 1;
                        state.register += count;
                    }
                    Instruction::Noop => {
                        draw_pixel(state.cycle, state.register)
                            .map(|coord| state.results.push(coord));
                        state.cycle += 1;
                    }
                };
                state
            },
        )
        .results
}

fn draw_pixel(cycle: i32, register: i32) -> Option<Coordinate> {
    let row = cycle.div_euclid(WIDTH);
    let col = (cycle - 1) % WIDTH;

    let active = vec![register - 1, register, register + 1];
    if active.contains(&col) {
        Some(Coordinate { row, col })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day10.test.txt");
        let result = solve_part1(&parse_input(input));
        dbg!(result);
        assert!(result == 13140);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day10.test.txt");
        let result = solve_part2(&parse_input(input));
        dbg!(&result);
        print_coordinates(&result, true);
    }
}
