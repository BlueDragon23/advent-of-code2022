use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
    time::Instant,
};

use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day20.txt"))?;
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input),
        time.elapsed().as_millis()
    );
    println!(
        "Part 1 alt: {} in {}ms",
        solve_part1_alt(&input),
        time.elapsed().as_millis()
    );
    let time = Instant::now();
    println!(
        "Part 2: {} in {}ms",
        solve_part2(&input),
        time.elapsed().as_millis()
    );
    Ok(())
}

mod parsing {
    use nom::{Finish, IResult};

    fn parse_line(input: &str) -> IResult<&str, i32> {
        (nom::character::complete::i32)(input)
    }

    pub fn parse_input(input: &str) -> color_eyre::Result<Vec<i32>> {
        Ok(input
            .lines()
            .map(|line| parse_line(line).finish().unwrap().1)
            .collect())
    }
}

fn solve_part1(input: &[i32]) -> i32 {
    let size = input.len();
    let mut output: HashMap<(i32, usize), usize> = build_map(input);
    // move numbers in the order they appear in the input
    for target in input
        .iter()
        .enumerate()
        .map(|(index, value)| (*value, index))
    {
        move_element_hash(&mut output, target, size as i32);
    }

    let start_index = output
        .iter()
        // find the _value_ 0
        .find_map(|(&value, index)| if value.0 == 0 { Some(index) } else { None })
        .unwrap();
    output
        .iter()
        .filter(|(_, &index)| {
            index == ((start_index + 1000) % size)
                || index == ((start_index + 2000) % size)
                || index == ((start_index + 3000) % size)
        })
        .map(|(value, _)| value.0)
        .sum()
}

fn solve_part1_alt(input: &[i32]) -> i32 {
    let size = input.len();
    let mut output: Vec<(usize, i32)> = input.iter().copied().enumerate().collect_vec();
    // move numbers in the order they appear in the input
    for target in input.iter().copied().enumerate() {
        move_element(&mut output, target, size as i32);
    }

    let start_index = output
        .iter()
        // find the _value_ 0
        .find_position(|(_, value)| *value == 0)
        .unwrap()
        .0;
    output[(start_index + 1000) % size].1
        + output[(start_index + 2000) % size].1
        + output[(start_index + 3000) % size].1
}

fn move_element(output: &mut Vec<(usize, i32)>, target: (usize, i32), size: i32) {
    let start = output.iter().find_position(|&n| *n == target).unwrap().0;
    let end = ((start as i32) + target.1).rem_euclid(size);
    output.remove(start);
    if target.1 < 0 && end > (start as i32) {
        // Insert before the target element if we wrap around
        output.insert((end - 1) as usize, target);
    } else if target.1 >= 0 && end < (start as i32) {
        // Insert before the target element if we wrap around
        output.insert((end + 1) as usize, target);
    } else {
        output.insert(end as usize, target);
    }
}

fn build_map(input: &[i32]) -> HashMap<(i32, usize), usize> {
    input
        .iter()
        .enumerate()
        .map(|(index, value)| ((*value, index), index))
        .collect()
}

fn move_element_hash(output: &mut HashMap<(i32, usize), usize>, target: (i32, usize), size: i32) {
    let original_index = output[&target];
    let new_index = *output
        .entry(target)
        .and_modify(|index| {
            let end_index = ((*index as i32) + target.0).rem_euclid(size) as usize;
            if target.0 < 0 && end_index > original_index {
                *index = end_index - 1;
            } else if target.0 >= 0 && end_index < original_index {
                *index = end_index + 1;
            } else {
                *index = end_index;
            }
        })
        .or_default();
    output.iter_mut().for_each(|(&value, index)| {
        if value == target {
            // don't modify the one we just put in
            return;
        }
        match ((*index).cmp(&new_index), (*index).cmp(&original_index)) {
            (Greater, Less) => *index += 1,
            (Equal, Less) => *index += 1,
            (Less, Greater) => *index -= 1,
            (Equal, Greater) => *index -= 1,
            _ => (),
        }
    })
}

fn solve_part2(_input: &[i32]) -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day20.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 3);
        Ok(())
    }

    #[test]
    fn test_part1_alt() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day20.test.txt"))?;
        let result = solve_part1_alt(&input);
        assert_eq!(result, 3);
        Ok(())
    }

    #[test]
    fn test_move_position_forwards_hash() {
        let mut input = build_map(&[2, 1, -3, 3, -2, 0, 4]);
        dbg!(&input);

        move_element_hash(&mut input, (2, 0), 7);

        let expected = build_map(&[1, -3, 2, 3, -2, 0, 4]);

        assert_eq!(
            input.iter().sorted_by_key(|(_, v)| *v).collect_vec(),
            expected.iter().sorted_by_key(|(_, v)| *v).collect_vec()
        );
    }

    #[test]
    fn test_move_position_forwards_wrap_hash() {
        let mut input = build_map(&[2, 1, -3, 3, -2, 0, 4]);
        dbg!(&input);

        move_element_hash(&mut input, (4, 6), 7);

        let expected = build_map(&[2, 1, -3, 3, 4, -2, 0]);

        assert_eq!(
            input.iter().sorted_by_key(|(_, v)| *v).collect_vec(),
            expected.iter().sorted_by_key(|(_, v)| *v).collect_vec()
        );
    }

    #[test]
    fn test_move_position_backwards_hash() {
        let mut input = build_map(&[1, -3, 2, 3, -2, 0, 4]);
        dbg!(&input);

        move_element_hash(&mut input, (-2, 4), 7);

        let expected = build_map(&[1, -3, -2, 2, 3, 0, 4]);

        assert_eq!(
            input.iter().sorted_by_key(|(_, v)| *v).collect_vec(),
            expected.iter().sorted_by_key(|(_, v)| *v).collect_vec()
        );
    }

    #[test]
    fn test_move_position_backwards_wrap_hash() {
        let mut input = build_map(&[1, -3, 2, 3, -2, 0, 4]);
        dbg!(&input);

        move_element_hash(&mut input, (-3, 1), 7);

        let expected = build_map(&[1, 2, 3, -2, -3, 0, 4]);

        assert_eq!(
            input.iter().sorted_by_key(|(_, v)| *v).collect_vec(),
            expected.iter().sorted_by_key(|(_, v)| *v).collect_vec()
        );
    }

    #[test]
    fn test_move_position_to_itself_hash() {
        let mut input = build_map(&[2, 1, 3]);
        dbg!(&input);

        move_element_hash(&mut input, (3, 0), 3);

        let expected = build_map(&[2, 1, 3]);

        assert_eq!(
            input.iter().sorted_by_key(|(_, v)| *v).collect_vec(),
            expected.iter().sorted_by_key(|(_, v)| *v).collect_vec()
        );
    }

    #[test]
    fn test_move_position_forwards_duplicate() {
        let mut input = build_map(&[2, 1, 2, 3]);
        dbg!(&input);

        move_element_hash(&mut input, (2, 0), 4);

        let mut expected = HashMap::new();
        expected.insert((2, 0), 2);
        expected.insert((2, 2), 1);
        expected.insert((1, 1), 0);
        expected.insert((3, 3), 3);

        assert_eq!(
            input.iter().sorted_by_key(|(_, v)| *v).collect_vec(),
            expected.iter().sorted_by_key(|(_, v)| *v).collect_vec()
        );
    }

    #[test]
    fn test_move_position_forwards_duplicate_latter() {
        let mut input = build_map(&[2, 1, 2, 3]);
        dbg!(&input);

        move_element_hash(&mut input, (2, 2), 4);

        let mut expected = HashMap::new();
        expected.insert((2, 0), 0);
        expected.insert((2, 2), 1);
        expected.insert((1, 1), 2);
        expected.insert((3, 3), 3);

        assert_eq!(
            input.iter().sorted_by_key(|(_, v)| *v).collect_vec(),
            expected.iter().sorted_by_key(|(_, v)| *v).collect_vec()
        );
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day20.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 1);
        Ok(())
    }
}
