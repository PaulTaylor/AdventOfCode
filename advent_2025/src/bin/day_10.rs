use good_lp::{Expression, ProblemVariables, Solution, Variable, constraint, variable};
use good_lp::{SolverModel, microlp};
use humantime::format_duration;
use regex::Regex;
use std::collections::HashMap;
use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

#[derive(Debug, Clone)]
struct Machine(Vec<bool>, Vec<Vec<usize>>, Vec<usize>, Vec<bool>);

fn extract_digits(digit_str: &str) -> Vec<usize> {
    let digit_pattern = Regex::new(r"\d+").unwrap();
    digit_pattern
        .find_iter(digit_str)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

fn parse(lines: &[String]) -> Vec<Machine> {
    let mut machines = Vec::with_capacity(lines.len());

    let pattern =
        Regex::new(r"(?<lights>\[([\.#]+)\]) (?<buttons>(\([0-9,]+\)\s+)+)(?<joltage>\{[0-9,]+\})")
            .expect("invalid pattern");

    for line in lines {
        let caps = pattern.captures(line).unwrap();

        let light_str = caps.name("lights").unwrap().as_str();
        let lights = (2..light_str.len()).map(|_| false).collect();
        let target = light_str
            .chars()
            .filter_map(|c| match c {
                '#' => Some(true),
                '.' => Some(false),
                _ => None,
            })
            .collect();

        let button_str = caps.name("buttons").unwrap().as_str();
        let buttons = button_str
            .split(' ')
            .map(extract_digits)
            .filter(|b| !b.is_empty())
            .collect();

        let joltage_str = caps.name("joltage").unwrap().as_str();
        let joltage = joltage_str.split(" ").map(extract_digits).next().unwrap();

        machines.push(Machine(lights, buttons, joltage, target));
    }

    machines
}

fn part_a(lines: &[String]) -> usize {
    let machines = parse(lines);
    let mut acc = 0;
    for machine in machines {
        let Machine(start_state, buttons, _, target) = machine;

        let mut queue = BTreeSet::from_iter([(0, start_state)]);
        while let Some((depth, state)) = queue.pop_first() {
            if state == target {
                acc += depth;
                break;
            }

            queue.extend(buttons.iter().map(|btn| {
                (
                    depth + 1,
                    state
                        .iter()
                        .enumerate()
                        .map(|(idx, elem)| if btn.contains(&idx) { !elem } else { *elem })
                        .collect(),
                )
            }));
        }
    }
    acc
}

fn part_b(lines: &[String]) -> f64 {
    let machines = parse(lines);

    let mut acc = 0f64;
    for machine in machines {
        let mut variables = ProblemVariables::new();

        // Create button set count variables
        let mut bs_counts = vec![];
        for idx in 0..machine.1.len() {
            let def = variable().name(format!("bs_{idx}_pushes")).min(0).integer();
            bs_counts.push(variables.add(def));
        }

        // Determine which bs_counts contribute to which positions
        let mut position_counts = HashMap::new();
        for (idx, set) in machine.1.iter().enumerate() {
            for button in set {
                let count = &bs_counts[idx];
                position_counts
                    .entry(*button)
                    .and_modify(|v: &mut Vec<&Variable>| v.push(count))
                    .or_insert(vec![count]);
            }
        }

        // Define the goal function - to minimise sum(bs_counts)
        let total_pushes = bs_counts
            .iter()
            .skip(1)
            .fold(Expression::from(bs_counts[0]), |acc, o| acc + o);

        // Create the problem and define the constraint that the sum of each position count
        // must be equal to the target value
        let mut prob = variables.minimise(&total_pushes).using(microlp);
        for (idx, var) in position_counts {
            let pos_total = var
                .iter()
                .skip(1)
                .fold(Expression::from(*var[0]), |a, b| a + *b);
            prob = prob.with(constraint!(pos_total == machine.2[idx] as u32))
        }

        let sln = prob.solve().expect("No solution found");

        acc += sln.eval(total_pushes);
    }

    acc
}

#[cfg(not(tarpaulin_include))]
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

    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 7);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 33f64);
    }
}
