use advent_of_code2022::Coordinate;
use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    let heights = include_str!("../../input/day8.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let mut count = 0;
    for row in 0..heights.len() {
        for col in 0..heights[0].len() {
            let coordinate = Coordinate { row, col };
            if is_visible(&heights, coordinate) {
                // dbg!(coordinate);
                count += 1;
            }
        }
    }

    let mut results = vec![];
    for row in 0..heights.len() {
        for col in 0..heights[0].len() {
            let coordinate = Coordinate { row, col };
            results.push(get_scenic_score(&heights, coordinate));
        }
    }
    let best_score = results.iter().max().unwrap();

    dbg!(count);
    dbg!(best_score);
    Ok(())
}

fn is_visible(heights: &Vec<Vec<u32>>, coord: Coordinate) -> bool {
    // search row and column for higher things
    let tree_height = heights[coord.row][coord.col];
    let mut visible = false;
    // check column
    visible |= (0..coord.row).all(|row| heights[row][coord.col] < tree_height);
    visible |= ((coord.row + 1)..heights.len()).all(|row| heights[row][coord.col] < tree_height);
    visible |= (0..coord.col).all(|col| heights[coord.row][col] < tree_height);
    visible |= ((coord.col + 1)..heights[0].len()).all(|col| heights[coord.row][col] < tree_height);
    visible
}

fn get_scenic_score(heights: &Vec<Vec<u32>>, coord: Coordinate) -> u64 {
    let tree_height = heights[coord.row][coord.col];
    let mut score: u64 = 1;
    let binding = (0..coord.row).rev().collect_vec();
    score *= binding
        .split_inclusive(|&row| heights[row][coord.col] >= tree_height)
        .next()
        .map_or(0, |slice| slice.len()) as u64;
    let binding = ((coord.row + 1)..heights.len()).collect_vec();
    score *= binding
        .split_inclusive(|&row| heights[row][coord.col] >= tree_height)
        .next()
        .map_or(0, |slice| slice.len()) as u64;
    let binding = (0..coord.col).rev().collect_vec();
    score *= binding
        .split_inclusive(|&col| heights[coord.row][col] >= tree_height)
        .next()
        .map_or(0, |slice| slice.len()) as u64;
    let binding = ((coord.col + 1)..heights[0].len()).collect_vec();
    score *= binding
        .split_inclusive(|&col| heights[coord.row][col] >= tree_height)
        .next()
        .map_or(0, |slice| slice.len()) as u64;
    score
}
