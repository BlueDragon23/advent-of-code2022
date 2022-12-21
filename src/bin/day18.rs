use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use itertools::Itertools;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct Coord3D {
    x: i32,
    y: i32,
    z: i32,
}

fn main() -> color_eyre::Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day18.txt"))?;
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input),
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
    use super::Coord3D;
    use nom::{bytes::complete::tag, combinator::map, multi::separated_list1, Finish, IResult};

    fn parse_line(input: &str) -> IResult<&str, Coord3D> {
        map(
            separated_list1(tag(","), nom::character::complete::i32),
            |xs| Coord3D {
                x: xs[0],
                y: xs[1],
                z: xs[2],
            },
        )(input)
    }

    pub fn parse_input(input: &str) -> color_eyre::Result<Vec<Coord3D>> {
        Ok(input
            .lines()
            .map(|line| parse_line(line).finish().unwrap().1)
            .collect())
    }
}

fn get_adjacent_cubes(cube: &Coord3D) -> Vec<Coord3D> {
    let moves = vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];
    moves
        .iter()
        .map(|(dx, dy, dz)| Coord3D {
            x: cube.x + dx,
            y: cube.y + dy,
            z: cube.z + dz,
        })
        .collect_vec()
}

fn solve_part1(input: &[Coord3D]) -> usize {
    let cubes: HashSet<Coord3D> = input.iter().copied().collect();
    let result = cubes.iter().fold(0, |surface_area, cube| {
        surface_area
            + get_adjacent_cubes(cube)
                .iter()
                .filter(|adj| !cubes.contains(adj))
                .count()
    });
    result
}

fn solve_part2(input: &[Coord3D]) -> u32 {
    let cubes: HashSet<Coord3D> = input.iter().copied().collect();
    let maybe_surface_cubes: HashMap<Coord3D, u32> =
        cubes
            .iter()
            .fold(HashMap::new(), |mut surface_cubes, cube| {
                get_adjacent_cubes(cube)
                    .iter()
                    .filter(|adj| !cubes.contains(adj))
                    .for_each(|adj| *surface_cubes.entry(*adj).or_default() += 1);
                surface_cubes
            });
    // connected components
    let mut connected_components: Vec<Vec<Coord3D>> = vec![];
    let mut seen_cubes: HashSet<Coord3D> = HashSet::new();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day18.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 64);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day18.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 58);
        Ok(())
    }
}
