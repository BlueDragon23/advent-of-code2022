use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Add, AddAssign, Mul, Sub},
    time::Instant,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct Template {
    id: u32,
    ore_cost: Resources,
    clay_cost: Resources,
    obsidian_cost: Resources,
    geode_cost: Resources,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Robots {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Add for Robots {
    type Output = Robots;

    fn add(self, rhs: Self) -> Self::Output {
        Robots {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Add for Resources {
    type Output = Resources;

    fn add(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl Sub for Resources {
    type Output = Resources;

    fn sub(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl Mul<u32> for Resources {
    type Output = Resources;

    fn mul(self, rhs: u32) -> Self::Output {
        Resources {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}

type State = (Resources, Robots, u32);

fn main() -> color_eyre::Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day19.txt"))?;
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

    use crate::Resources;

    use super::Template;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        combinator::map,
        sequence::{delimited, terminated, tuple},
        Finish, IResult,
    };

    fn parse_id(input: &str) -> IResult<&str, u32> {
        delimited(tag("Blueprint "), nom::character::complete::u32, tag(": "))(input)
    }

    fn parse_cost(input: &str) -> IResult<&str, Resources> {
        // Each ore robot costs 4 ore.
        // Each obsidian robot costs 3 ore and 14 clay.
        // Each geode robot costs 2 ore and 7 obsidian.
        alt((
            map(
                delimited(
                    delimited(
                        tag("Each "),
                        alt((tag("ore"), tag("clay"))),
                        tag(" robot costs "),
                    ),
                    nom::character::complete::u32,
                    tag(" ore. "),
                ),
                |ore| Resources {
                    ore,
                    ..Default::default()
                },
            ),
            map(
                tuple((
                    delimited(
                        tag("Each obsidian robot costs "),
                        nom::character::complete::u32,
                        tag(" ore and "),
                    ),
                    terminated(nom::character::complete::u32, tag(" clay. ")),
                )),
                |(ore, clay)| Resources {
                    ore,
                    clay,
                    ..Default::default()
                },
            ),
            map(
                tuple((
                    delimited(
                        tag("Each geode robot costs "),
                        nom::character::complete::u32,
                        tag(" ore and "),
                    ),
                    terminated(nom::character::complete::u32, tag(" obsidian.")),
                )),
                |(ore, obsidian)| Resources {
                    ore,
                    obsidian,
                    ..Default::default()
                },
            ),
        ))(input)
    }

    fn parse_line(input: &str) -> IResult<&str, Template> {
        map(
            tuple((parse_id, parse_cost, parse_cost, parse_cost, parse_cost)),
            |(id, ore_cost, clay_cost, obsidian_cost, geode_cost)| Template {
                id,
                ore_cost,
                clay_cost,
                obsidian_cost,
                geode_cost,
            },
        )(input)
    }

    pub fn parse_input(input: &str) -> color_eyre::Result<Vec<Template>> {
        Ok(input
            .lines()
            .map(|line| parse_line(line).finish().unwrap().1)
            .collect())
    }
}

fn solve_part1(input: &[Template]) -> u32 {
    let max_geodes = input
        .iter()
        .map(|template| {
            let robots = Robots {
                ore: 1,
                ..Robots::default()
            };
            let resources = Resources::default();
            let max_geodes = find_max_geodes(
                &resources,
                template,
                &robots,
                24,
                &RefCell::new(HashMap::new()),
            );
            dbg!(max_geodes);
            (max_geodes, template.id)
        })
        .collect_vec();
    max_geodes
        .iter()
        .map(|(geode, template_id)| geode * template_id)
        .sum()
}

fn find_max_geodes(
    resources: &Resources,
    template: &Template,
    robots: &Robots,
    time_remaining: u32,
    memoising: &RefCell<HashMap<State, u32>>,
) -> u32 {
    if time_remaining == 0 {
        return resources.geode;
    }
    if let Some((_, value)) =
        memoising
            .borrow()
            .get_key_value(&(*resources, *robots, time_remaining))
    {
        return *value;
    }
    // Gather before adding robots
    let gathered_resources = Resources {
        ore: robots.ore,
        clay: robots.clay,
        obsidian: robots.obsidian,
        geode: robots.geode,
    };
    // After building new robots
    let new_resources = *resources + gathered_resources;
    let result = build_robots(resources, template, robots)
        .iter()
        .map(|(cost, added_robots)| {
            let after_cost = new_resources - *cost;
            find_max_geodes(
                &after_cost,
                template,
                &(*robots + *added_robots),
                time_remaining - 1,
                memoising,
            )
        })
        .max()
        .unwrap();
    memoising
        .borrow_mut()
        .insert((*resources, *robots, time_remaining), result);
    result
}

// generate all possible combinations of resources spent and robots built
fn build_robots(
    resources: &Resources,
    template: &Template,
    current_robots: &Robots,
) -> Vec<(Resources, Robots)> {
    let max_geodes = get_max_robots(resources, &template.geode_cost);
    let mut results = vec![];
    // always build the maximum geode robots
    let geode_robots = max_geodes;
    let remaining = *resources - (template.geode_cost * geode_robots);
    let max_obsidian = get_max_robots(&remaining, &template.obsidian_cost);
    for obsidian_robots in (0..=max_obsidian).rev() {
        let remaining = remaining - (template.obsidian_cost * obsidian_robots);
        let max_clay = if current_robots.clay > 5 {
            0
        } else {
            get_max_robots(&remaining, &template.clay_cost)
        };
        for clay_robots in (0..=max_clay).rev() {
            let remaining = remaining - (template.clay_cost * clay_robots);
            let max_ore = if current_robots.ore > 5 {
                0
            } else {
                get_max_robots(&remaining, &template.ore_cost)
            };
            for ore_robots in (0..=max_ore).rev() {
                let remaining = remaining - (template.ore_cost * ore_robots);
                results.push((
                    // spent resources
                    *resources - remaining,
                    Robots {
                        geode: geode_robots,
                        obsidian: obsidian_robots,
                        clay: clay_robots,
                        ore: ore_robots,
                    },
                ));
            }
        }
    }
    // build
    results
}

fn get_max_robots(resources: &Resources, cost: &Resources) -> u32 {
    vec![
        resources.ore.checked_div_euclid(cost.ore),
        resources.clay.checked_div_euclid(cost.clay),
        resources.obsidian.checked_div_euclid(cost.obsidian),
    ]
    .into_iter()
    .flatten()
    .min()
    .unwrap()
}

fn solve_part2(input: &[Template]) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day19.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 33);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day19.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn test_build_robots() {
        let template = Template {
            id: 1,
            ore_cost: Resources {
                ore: 4,
                ..Default::default()
            },
            clay_cost: Resources {
                ore: 2,
                ..Default::default()
            },
            obsidian_cost: Resources {
                ore: 3,
                clay: 14,
                ..Default::default()
            },
            geode_cost: Resources {
                ore: 2,
                obsidian: 7,
                ..Default::default()
            },
        };
        let resources = Resources {
            // enough for some other bots
            ore: 11,
            // 0-1 obsidian bots
            clay: 17,
            // 0-1 geode bots
            obsidian: 8,
            geode: 0,
        };
        let actual = build_robots(&resources, &template, &Robots::default());
        let expected = vec![(
            resources,
            Robots {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
        )];
        dbg!(&actual);
        assert_eq!(actual.len(), 36);
        assert!(expected.iter().all(|t| actual.contains(t)));
    }

    #[test]
    fn test_build_no_robots() {
        let template = Template {
            id: 1,
            ore_cost: Resources {
                ore: 4,
                ..Default::default()
            },
            clay_cost: Resources {
                ore: 2,
                ..Default::default()
            },
            obsidian_cost: Resources {
                ore: 3,
                clay: 14,
                ..Default::default()
            },
            geode_cost: Resources {
                ore: 2,
                obsidian: 7,
                ..Default::default()
            },
        };
        let resources = Resources {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        let actual = build_robots(&resources, &template, &Robots::default());
        let expected = vec![(
            Resources::default(),
            Robots {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
        )];
        dbg!(&actual);
        assert!(expected.iter().all(|t| actual.contains(t)));
    }
}
