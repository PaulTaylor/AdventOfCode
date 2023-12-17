use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> (&str, HashMap<&str, (&str, &str)>) {
    let pattern = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)$").unwrap();
    let instr = lines[0].as_str();
    let edges: HashMap<_, _> = lines[2..]
        .iter()
        .map(|l| {
            let m = pattern.captures(l).unwrap();
            (
                m.get(1).unwrap().as_str(),
                (m.get(2).unwrap().as_str(), m.get(3).unwrap().as_str()),
            )
        })
        .collect();

    (instr, edges)
}

fn part_a(lines: &[String]) -> usize {
    // Straightforward path traversal

    let (instr, edges) = parse(lines);

    let mut pos = "AAA";
    let mut steps = 0;
    let mut directions = instr.chars().cycle();

    while pos != "ZZZ" {
        let dir = directions.next().unwrap();
        pos = match dir {
            'L' => edges[&pos].0,
            'R' => edges[&pos].1,
            x => panic!("Unknown direction {x}"),
        };
        steps += 1;
    }

    steps
}

fn part_b(lines: &[String]) -> usize {
    // Find the paths followed from multiple-start to multiple-ends
    //
    // Assumption: Each path is actually cycle of computable length
    //
    // Therefore we can determine the length of each cycle and then find the lowest
    // common multiple of all lengths to find the first point where all paths sync up.

    let (instr, edges) = parse(lines);
    let starts: Vec<&str> = edges.keys().copied().filter(|n| n.ends_with('A')).collect();
    let cycle_lengths: Vec<usize> = starts
        .iter()
        .copied()
        .map(|start| {
            let mut pos = start;
            let mut steps = 0;
            let mut directions = instr.chars().cycle();
            while !pos.ends_with('Z') {
                let dir = directions.next().unwrap();
                pos = match dir {
                    'L' => edges[&pos].0,
                    'R' => edges[&pos].1,
                    x => panic!("Unknown direction {x}"),
                };
                steps += 1;
            }
            steps
        })
        .collect();

    // Since LCM is commutative can simply reduce over the lcm method
    cycle_lengths.into_iter().reduce(num::integer::lcm).unwrap()
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

    const TEST_INPUT_1: &str = "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_2: &str = "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT_1.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 2);
        let lines: Vec<_> = TEST_INPUT_2.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 6);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)"
            .lines()
            .map(|l| l.trim().to_string())
            .collect();
        assert_eq!(part_b(lines.as_slice()), 6);
    }
}
