use humantime::format_duration;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

lazy_static! {
    static ref PATTERN: Regex =
        Regex::new(r##"^Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.$"##)
            .unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Blueprint {
    id: usize,
    ore_ore_cost: usize,
    clay_ore_cost: usize,
    obs_ore_cost: usize,
    obs_clay_cost: usize,
    geo_ore_cost: usize,
    geo_obs_cost: usize,
}

impl From<&String> for Blueprint {
    fn from(value: &String) -> Self {
        let matches = PATTERN.captures(value).unwrap();
        Blueprint {
            id: matches.get(1).unwrap().as_str().parse().unwrap(),
            ore_ore_cost: matches.get(2).unwrap().as_str().parse().unwrap(),
            clay_ore_cost: matches.get(3).unwrap().as_str().parse().unwrap(),
            obs_ore_cost: matches.get(4).unwrap().as_str().parse().unwrap(),
            obs_clay_cost: matches.get(5).unwrap().as_str().parse().unwrap(),
            geo_ore_cost: matches.get(6).unwrap().as_str().parse().unwrap(),
            geo_obs_cost: matches.get(7).unwrap().as_str().parse().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    time: usize,
    // resources available
    ore: usize,
    clay: usize,
    obs: usize,
    // robots available
    r_ore: usize,
    r_clay: usize,
    r_obs: usize,
    r_geode: usize,
    // output
    geos: usize,
}

fn parse(lines: &[String]) -> Vec<Blueprint> {
    lines.iter().map(Blueprint::from).collect()
}

fn evaluate_bp(bp: Blueprint, time_limit: usize) -> usize {
    let mut to_check: Vec<State> = vec![State {
        time: 0, // to be read as "this is the state at the end of {time} minutes"
        ore: 0,
        clay: 0,
        obs: 0,
        r_ore: 1,
        r_clay: 0,
        r_obs: 0,
        r_geode: 0,
        geos: 0,
    }];

    for _t in 1..=time_limit {
        let mut next_ts = Vec::with_capacity(to_check.len() * 4);
        next_ts.par_extend(to_check.into_par_iter().flat_map(|state| {
            let noop = State {
                time: state.time + 1,
                ore: state.ore + state.r_ore,
                clay: state.clay + state.r_clay,
                obs: state.obs + state.r_obs,
                geos: state.geos + state.r_geode,
                ..state
            };

            let mut successors = vec![noop];

            if state.ore >= bp.ore_ore_cost {
                successors.push(State {
                    ore: noop.ore - bp.ore_ore_cost,
                    r_ore: noop.r_ore + 1,
                    ..noop
                });
            }
            if state.ore >= bp.clay_ore_cost {
                successors.push(State {
                    ore: noop.ore - bp.clay_ore_cost,
                    r_clay: noop.r_clay + 1,
                    ..noop
                });
            }
            if state.ore >= bp.obs_ore_cost && state.clay >= bp.obs_clay_cost {
                successors.push(State {
                    ore: noop.ore - bp.obs_ore_cost,
                    clay: noop.clay - bp.obs_clay_cost,
                    r_obs: noop.r_obs + 1,
                    ..noop
                });
            }
            if state.ore >= bp.geo_ore_cost && state.obs >= bp.geo_obs_cost {
                successors.push(State {
                    ore: noop.ore - bp.geo_ore_cost,
                    obs: noop.obs - bp.geo_obs_cost,
                    r_geode: noop.r_geode + 1,
                    ..noop
                });
            }

            successors
        }));

        // Beam search - sort on heuristic and limit to best N items
        next_ts.sort_by(|s1, s2| {
            // Heursitic prefers numbers of advanced robots, then the
            // quantity of the basic common resources.  There might be a better
            // one that lets you reduce the beam size
            (s1.geos, s1.r_geode, s1.r_obs, s1.ore, s1.clay)
                .cmp(&(s2.geos, s2.r_geode, s2.r_obs, s2.ore, s2.clay))
                .reverse()
        });

        // Beam size is arrived at my trial-and-error, if using a different input
        // you might need to increase it from this value
        to_check = next_ts.into_iter().take(50000).collect();
    }

    to_check.iter().map(|s| s.geos).max().unwrap()
}

fn part_a(lines: &[String]) -> usize {
    parse(lines)
        .into_iter()
        .map(|bp| bp.id * evaluate_bp(bp, 24))
        .sum()
}

fn part_b(lines: &[String]) -> usize {
    parse(&lines[..3])
        .into_iter()
        .map(|bp| evaluate_bp(bp, 32))
        .product()
}

fn main() -> AResult<()> {
    // Figure out which exercise we are so we can load the correct input file
    let pattern = Regex::new("[0-9]+$")?;
    let name = &std::env::args().next().expect("binary name not found.");
    let ex = pattern
        .find(name)
        .expect("binary name should contain a number")
        .as_str();
    println!("Running code for Day {ex}.");

    // Load the appropriate input text
    let file = File::open(format!("./data/day_{ex}.txt"))?;
    let lines: Vec<String> = BufReader::new(file).lines().map(Result::unwrap).collect();

    // Run the solutions
    let start = Instant::now();
    println!("Part A result = {}", part_a(lines.as_slice()));
    println!("Part B result = {}", part_b(lines.as_slice()));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
  Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_eval_bp() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let plan = parse(&lines[..1])[0];
        assert_eq!(
            plan,
            Blueprint {
                id: 1,
                ore_ore_cost: 4,
                clay_ore_cost: 2,
                obs_ore_cost: 3,
                obs_clay_cost: 14,
                geo_ore_cost: 2,
                geo_obs_cost: 7,
            }
        );

        assert_eq!(evaluate_bp(plan, 24), 9);
    }

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 33);
    }

    #[test]
    fn test_b() {
        let mut lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        lines.push(lines[0].clone());
        assert_eq!(part_b(lines.as_slice()), 56 * 62 * 56);
    }
}
