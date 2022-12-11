use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use reformation::Reformation;

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    test: u128,
    target_true: usize,
    target_false: usize,
}

#[derive(Reformation, Clone, Copy, Debug)]
#[reformation("  Operation: new = {left} {operator} {right}")]
struct Operation {
    left: Operand,
    operator: Operator,
    right: Operand,
}

#[derive(Reformation, Clone, Copy, Debug)]
enum Operand {
    #[reformation("old")]
    Old,
    #[reformation(r"{}")]
    Constant(u128),
}

#[derive(Reformation, Clone, Copy, Debug)]
enum Operator {
    #[reformation(r"\+")]
    Add,
    #[reformation(r"\*")]
    Multiply,
}

fn main() -> color_eyre::Result<()> {
    let input = include_str!("../../input/day11.txt");
    let input_processed = parse_input(input);
    println!("Part 1: {}", solve_part1(&input_processed));
    println!("Part 2: {}", solve_part2(&input_processed));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|group| {
            // skip the number
            let mut lines = group.lines().skip(1);
            let items = parse_items(lines.next().unwrap());
            let operation = Operation::parse(lines.next().unwrap()).unwrap();
            let test = lines
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<u128>()
                .unwrap();
            let target_true = lines
                .next()
                .unwrap()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as usize;
            let target_false = lines
                .next()
                .unwrap()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as usize;
            Monkey {
                items,
                operation,
                test,
                target_true,
                target_false,
            }
        })
        .collect_vec()
}

fn parse_items(line: &str) -> VecDeque<u128> {
    line.split(":")
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse::<u128>().unwrap())
        .collect()
}

fn solve_part1(input: &Vec<Monkey>) -> u128 {
    let mut monkeys = input.clone();
    let gcd = gcd(&monkeys);
    let num_rounds = 20;
    let mut inspection_count: Vec<u128> = vec![0; monkeys.len()];
    for _ in 0..num_rounds {
        let (new_monkeys, new_inspection) = process_round(&monkeys, 1, gcd);
        monkeys = new_monkeys;
        inspection_count = new_inspection
            .iter()
            .zip(inspection_count)
            .map(|(a, b)| a + b)
            .collect_vec();
    }
    inspection_count.iter().sorted().rev().take(2).product()
}

fn process_round(monkeys: &Vec<Monkey>, part: u32, gcd: u128) -> (Vec<Monkey>, Vec<u128>) {
    let mut monkeys = monkeys.clone();
    let mut inspection_count = vec![0; monkeys.len()];
    for index in 0..monkeys.len() {
        let (updates, inspections) = process_monkey(&monkeys[index], part, gcd);
        inspection_count[index] += inspections;
        monkeys = monkeys
            .into_iter()
            .enumerate()
            .map(|(i, mut m)| {
                if i == index {
                    m.items.clear();
                } else {
                    for item in updates.get(&i).unwrap_or(&VecDeque::new()) {
                        m.items.push_back(item.clone());
                    }
                }
                m
            })
            .collect_vec();
    }

    (monkeys, inspection_count)
}

fn process_monkey(monkey: &Monkey, part: u32, gcd: u128) -> (HashMap<usize, VecDeque<u128>>, u128) {
    let mut next_items = HashMap::new();
    let mut inspections = 0;
    let mut items = monkey.items.clone();
    while let Some(worry) = items.pop_front() {
        let new_worry = reduce_below_gcd(
            if part == 1 {
                reduce_worry(increase_worry(worry, monkey.operation))
            } else {
                increase_worry(worry, monkey.operation)
            },
            gcd,
        );
        inspections += 1;
        let next_monkey = if test_worry(&new_worry, monkey.test) {
            monkey.target_true
        } else {
            monkey.target_false
        };
        next_items
            .entry(next_monkey)
            .or_insert(VecDeque::new())
            .push_back(new_worry.clone());
    }
    (next_items, inspections)
}

fn test_worry(worry: &u128, test_value: u128) -> bool {
    worry % test_value == 0
}

fn reduce_below_gcd(worry: u128, gcd: u128) -> u128 {
    if worry > gcd {
        worry % gcd
    } else {
        worry
    }
}

fn increase_worry(old: u128, operation: Operation) -> u128 {
    let left = match operation.left {
        Operand::Old => old.clone(),
        Operand::Constant(x) => x,
    };
    let right = match operation.right {
        Operand::Old => old.clone(),
        Operand::Constant(x) => x,
    };
    match operation.operator {
        Operator::Add => left + right,
        Operator::Multiply => left * right,
    }
}

fn reduce_worry(worry: u128) -> u128 {
    worry.div_euclid(3)
}

fn solve_part2(input: &Vec<Monkey>) -> u128 {
    let mut monkeys = input.clone();
    let gcd = gcd(&monkeys);
    dbg!(gcd);
    let num_rounds = 10000;
    let mut inspection_count = vec![0; monkeys.len()];
    for _ in 0..num_rounds {
        let (new_monkeys, new_inspection) = process_round(&monkeys, 2, gcd);
        monkeys = new_monkeys;
        inspection_count = new_inspection
            .iter()
            .zip(inspection_count)
            .map(|(a, b)| a + b)
            .collect_vec();
    }
    inspection_count.iter().sorted().rev().take(2).product()
}

fn gcd(monkeys: &Vec<Monkey>) -> u128 {
    // 17, 13 -> 221
    // x % 17 = (x - 221) % 17
    // x % 13 = (x - 221) % 13
    monkeys.iter().map(|m| m.test).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../input/day11.test.txt");
        let input_processed = parse_input(input);
        let result = solve_part1(&input_processed);
        dbg!(&result);
        assert!(result == 10605);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../input/day11.test.txt");
        let input_processed = parse_input(input);
        let result = solve_part2(&input_processed);
        dbg!(&result);
        assert!(result == 2713310158);
    }
}
