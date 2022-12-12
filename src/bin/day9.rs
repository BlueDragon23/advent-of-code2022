use std::collections::HashSet;

use advent_of_code2022::{get_adjacent_points_diagonal, Coordinate};
use reformation::Reformation;

#[derive(Reformation, Debug, Clone, Copy)]
enum Direction {
    #[reformation("R {}")]
    Right(u32),
    #[reformation("D {}")]
    Down(u32),
    #[reformation("U {}")]
    Up(u32),
    #[reformation("L {}")]
    Left(u32),
}

#[derive(Debug)]
struct State {
    head_position: Coordinate<i32>,
    tail_position: Coordinate<i32>,
    visited: HashSet<Coordinate<i32>>,
}

#[derive(Debug)]
struct State2 {
    head_position: Coordinate<i32>,
    knot_positions: Vec<Coordinate<i32>>,
    visited: HashSet<Coordinate<i32>>,
}

fn main() -> color_eyre::Result<()> {
    let input = include_str!("../../input/day9.txt");
    dbg!(solve_part1(input));
    dbg!(solve_part2(input));
    Ok(())
}

fn solve_part1(input: &str) -> usize {
    let state = input
        .lines()
        .map(|line| Direction::parse(line).unwrap())
        .fold(
            State {
                head_position: Coordinate { row: 0, col: 0 },
                tail_position: Coordinate { row: 0, col: 0 },
                visited: vec![Coordinate { row: 0, col: 0 }].into_iter().collect(),
            },
            |mut state, command| {
                match command {
                    Direction::Right(distance) => {
                        for _ in 0..distance {
                            state.head_position = Coordinate {
                                col: state.head_position.col + 1,
                                ..state.head_position
                            };
                            state.tail_position =
                                resolve_tail_position(state.head_position, state.tail_position);
                            state.visited.insert(state.tail_position);
                        }
                    }
                    Direction::Down(distance) => {
                        for _ in 0..distance {
                            state.head_position = Coordinate {
                                row: state.head_position.row - 1,
                                ..state.head_position
                            };
                            state.tail_position =
                                resolve_tail_position(state.head_position, state.tail_position);
                            state.visited.insert(state.tail_position);
                        }
                    }
                    Direction::Up(distance) => {
                        for _ in 0..distance {
                            state.head_position = Coordinate {
                                row: state.head_position.row + 1,
                                ..state.head_position
                            };
                            state.tail_position =
                                resolve_tail_position(state.head_position, state.tail_position);
                            state.visited.insert(state.tail_position);
                        }
                    }
                    Direction::Left(distance) => {
                        for _ in 0..distance {
                            state.head_position = Coordinate {
                                col: state.head_position.col - 1,
                                ..state.head_position
                            };
                            state.tail_position =
                                resolve_tail_position(state.head_position, state.tail_position);
                            state.visited.insert(state.tail_position);
                        }
                    }
                }
                state
            },
        );
    // print_coordinates(&state.visited);
    state.visited.len()
}

fn resolve_tail_position(
    head_position: Coordinate<i32>,
    tail_position: Coordinate<i32>,
) -> Coordinate<i32> {
    if get_adjacent_points_diagonal(head_position, i32::MIN, i32::MIN, i32::MAX, i32::MAX)
        .contains(&tail_position)
    {
        tail_position
    } else {
        // column is different
        let new_col = match head_position.col.cmp(&tail_position.col) {
            std::cmp::Ordering::Less => tail_position.col - 1,
            std::cmp::Ordering::Equal => tail_position.col,
            std::cmp::Ordering::Greater => tail_position.col + 1,
        };
        let new_row = match head_position.row.cmp(&tail_position.row) {
            std::cmp::Ordering::Less => tail_position.row - 1,
            std::cmp::Ordering::Equal => tail_position.row,
            std::cmp::Ordering::Greater => tail_position.row + 1,
        };
        Coordinate {
            row: new_row,
            col: new_col,
        }
    }
}

fn solve_part2(input: &str) -> usize {
    let state = input
        .lines()
        .map(|line| Direction::parse(line).unwrap())
        .fold(
            State2 {
                head_position: Coordinate { row: 0, col: 0 },
                knot_positions: vec![Coordinate { row: 0, col: 0 }; 9],
                visited: vec![Coordinate { row: 0, col: 0 }].into_iter().collect(),
            },
            |mut state, command| {
                match command {
                    Direction::Right(distance) => {
                        for _ in 0..distance {
                            state.head_position = Coordinate {
                                col: state.head_position.col + 1,
                                ..state.head_position
                            };
                            state.knot_positions =
                                update_knot_positions(state.head_position, &state.knot_positions);
                            validate_state(&state.knot_positions);
                            state.visited.insert(*state.knot_positions.last().unwrap());
                        }
                    }
                    Direction::Down(distance) => {
                        for _ in 0..distance {
                            state.head_position = Coordinate {
                                row: state.head_position.row - 1,
                                ..state.head_position
                            };
                            state.knot_positions =
                                update_knot_positions(state.head_position, &state.knot_positions);
                            validate_state(&state.knot_positions);
                            state.visited.insert(*state.knot_positions.last().unwrap());
                        }
                    }
                    Direction::Up(distance) => {
                        for _ in 0..distance {
                            state.head_position = Coordinate {
                                row: state.head_position.row + 1,
                                ..state.head_position
                            };
                            state.knot_positions =
                                update_knot_positions(state.head_position, &state.knot_positions);
                            validate_state(&state.knot_positions);
                            state.visited.insert(*state.knot_positions.last().unwrap());
                        }
                    }
                    Direction::Left(distance) => {
                        for _ in 0..distance {
                            state.head_position = Coordinate {
                                col: state.head_position.col - 1,
                                ..state.head_position
                            };
                            state.knot_positions =
                                update_knot_positions(state.head_position, &state.knot_positions);
                            validate_state(&state.knot_positions);
                            state.visited.insert(*state.knot_positions.last().unwrap());
                        }
                    }
                }
                // print_coordinates(&state.knot_positions.clone().into_iter().chain(iter::once(state.head_position)).collect());
                state
            },
        );
    // print_coordinates(&state.visited);
    state.visited.len()
}

fn update_knot_positions(
    head_position: Coordinate<i32>,
    current_positions: &Vec<Coordinate<i32>>,
) -> Vec<Coordinate<i32>> {
    current_positions
        .iter()
        .fold(
            (head_position, vec![]),
            |(position, mut result), &next_knot| {
                let next_position = resolve_tail_position(position, next_knot);
                result.push(next_position);
                (next_position, result)
            },
        )
        .1
}

fn validate_state(knot_positions: &Vec<Coordinate<i32>>) {
    if !(knot_positions
        .iter()
        .take(knot_positions.len() - 1)
        .zip(knot_positions.iter().skip(1))
        .all(|(a, b)| {
            if a == b
                || get_adjacent_points_diagonal(*a, i32::MIN, i32::MIN, i32::MAX, i32::MAX)
                    .contains(b)
            {
                true
            } else {
                dbg!(a, b);
                false
            }
        }))
    {
        dbg!(knot_positions);
        panic!("Found invalid positions");
        // print_coordinates(&knot_positions.clone().into_iter().collect());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day9.test.txt");
        let result = solve_part1(input);
        dbg!(result);
        assert!(result == 13);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day9.test.txt");
        let result = solve_part2(input);
        assert!(result == 1);
    }

    #[test]
    fn test_part2_2() {
        let input = include_str!("../../input/day9.test.2.txt");
        let result = solve_part2(input);
        assert!(result == 36);
    }
}
