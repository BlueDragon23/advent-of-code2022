use advent_of_code2022::Coordinate;
use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    let heights = include_str!("../../input/day8.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let count = (0..heights.len())
        .cartesian_product(0..heights[0].len())
        .fold(0, |count, (row, col)| {
            let coordinate = Coordinate { row, col };
            if is_visible(&heights, coordinate) {
                return count + 1;
            }
            count
        });

    let best_score = (0..heights.len())
        .cartesian_product(0..heights[0].len())
        .map(|(row, col)| {
            let coordinate = Coordinate { row, col };
            get_scenic_score(&heights, coordinate)
        })
        .max()
        .unwrap();

    dbg!(count);
    dbg!(best_score);
    Ok(())
}

fn is_visible(heights: &Vec<Vec<u32>>, coord: Coordinate<usize>) -> bool {
    // search row and column for higher things
    let tree_height = heights[coord.row][coord.col];
    vec![
        (0..coord.row).all(|row| heights[row][coord.col] < tree_height),
        ((coord.row + 1)..heights.len()).all(|row| heights[row][coord.col] < tree_height),
        (0..coord.col).all(|col| heights[coord.row][col] < tree_height),
        ((coord.col + 1)..heights[0].len()).all(|col| heights[coord.row][col] < tree_height),
    ]
    .iter()
    .any(|&b| b)
}

fn get_scenic_score(heights: &Vec<Vec<u32>>, coord: Coordinate<usize>) -> u64 {
    let tree_height = heights[coord.row][coord.col];
    let mut score: u64 = 1;
    score *= get_direction_scenic_score(&(0..coord.row).rev().collect_vec(), |&row| {
        heights[row][coord.col] >= tree_height
    });
    score *= get_direction_scenic_score(&((coord.row + 1)..heights.len()).collect_vec(), |&row| {
        heights[row][coord.col] >= tree_height
    });
    score *= get_direction_scenic_score(&(0..coord.col).rev().collect_vec(), |&col| {
        heights[coord.row][col] >= tree_height
    });
    score *=
        get_direction_scenic_score(&((coord.col + 1)..heights[0].len()).collect_vec(), |&col| {
            heights[coord.row][col] >= tree_height
        });
    score
}

fn get_direction_scenic_score<F>(range: &[usize], predicate: F) -> u64
where
    F: FnMut(&usize) -> bool,
{
    range
        .split_inclusive(predicate)
        .next()
        .map_or(0, |slice| slice.len()) as u64
}
