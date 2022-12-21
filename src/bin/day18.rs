use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use itertools::Itertools;

#[derive(Default, Eq, PartialEq, Hash, Debug, Clone, Copy)]
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
    let min_x = maybe_surface_cubes.keys().map(|c| c.x).min().unwrap();
    let min_y = maybe_surface_cubes.keys().map(|c| c.y).min().unwrap();
    let min_z = maybe_surface_cubes.keys().map(|c| c.z).min().unwrap();
    let max_x = maybe_surface_cubes.keys().map(|c| c.x).max().unwrap();
    let max_y = maybe_surface_cubes.keys().map(|c| c.y).max().unwrap();
    let max_z = maybe_surface_cubes.keys().map(|c| c.z).max().unwrap();
    dbg!(max_x, max_y, max_z);
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    // explore from a spot known to be outside the lava
    queue.push_back(Coord3D::default());
    seen.insert(Coord3D::default());
    while let Some(next) = queue.pop_front() {
        get_adjacent_cubes(&next)
            .iter()
            .filter(|c| {
                c.x >= min_x
                    && c.x <= max_x
                    && c.y >= min_y
                    && c.y <= max_y
                    && c.z >= min_z
                    && c.z <= max_z
            })
            .for_each(|c| {
                if !seen.contains(c) && !cubes.contains(c) {
                    queue.push_back(*c);
                    seen.insert(*c);
                }
            });
    }
    maybe_surface_cubes
        .iter()
        .filter(|(c, _)| seen.contains(c))
        .map(|(_, count)| count)
        .sum()
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
