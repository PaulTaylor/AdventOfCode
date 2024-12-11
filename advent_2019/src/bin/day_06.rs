use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> HashMap<String, String> {
    let out: HashMap<_, _> = lines
        .iter()
        .map(|l| {
            let mut split = l.split(')');
            let centre = split.next().unwrap();
            let orbiter = split.next().unwrap();
            (orbiter.to_string(), centre.to_string())
        })
        .collect();

    assert!(out.len() == lines.len());

    out
}

fn generate_path_from(start: &str, parent_lookup: &HashMap<String, String>) -> Vec<String> {
    let mut path = vec![start.to_string()];
    let mut p = start;
    while let Some(q) = parent_lookup.get(p) {
        path.insert(0, q.to_string());
        p = q;
    }
    path
}

fn part_a(lines: &[String]) -> usize {
    let parent_lookup = parse(lines);
    let all_objects: HashSet<&String> =
        parent_lookup.keys().chain(parent_lookup.values()).collect();

    let mut acc = 0;
    for start in all_objects {
        let path = generate_path_from(start, &parent_lookup);
        acc += path.len() - 1;
    }
    acc
}

fn part_b(lines: &[String]) -> usize {
    let parent_lookup = parse(lines);

    // Collect both paths to the centre
    let my_root_path = generate_path_from("YOU", &parent_lookup);
    let santas_path = generate_path_from("SAN", &parent_lookup);

    // Calculate the common root of these paths
    let common_root: Vec<_> = my_root_path
        .iter()
        .zip(santas_path.iter())
        .filter_map(|(me, santa)| if me == santa { Some(me) } else { None })
        .collect();

    // Total length of the non-common paths
    let result = (my_root_path.len() - common_root.len()) + (santas_path.len() - common_root.len());

    // Subtract 2 because I'm counting me-local and santa-local in the result
    result - 2
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

    const TEST_INPUT: &str = "COM)B
                                B)C
                                C)D
                                D)E
                                E)F
                                B)G
                                G)H
                                D)I
                                E)J
                                J)K
                                K)L";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 42);
    }

    #[test]
    fn test_b() {
        let mut input = TEST_INPUT.to_string();
        input.push_str("\nK)YOU\nI)SAN");

        let lines: Vec<_> = input.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 4);
    }
}
