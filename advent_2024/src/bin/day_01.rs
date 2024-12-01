use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> (Vec<usize>, Vec<usize>) {
    let mut a = Vec::with_capacity(lines.len());
    let mut b = Vec::with_capacity(lines.len());

    for line in lines {
        let split: Vec<_> = line.split_whitespace().flat_map(str::parse).collect();
        a.push(split[0]);
        b.push(split[1]);
    }

    (a, b)
}

fn part_a(lines: &[String]) -> usize {
    let (mut a, mut b) = parse(lines);
    a.sort_unstable();
    b.sort_unstable();
    a.into_iter().zip(b).map(|(l, r)| l.abs_diff(r)).sum()
}

fn part_b(lines: &[String]) -> usize {
    let (a, b) = parse(lines);
    let mut b_freq = HashMap::new();
    for v in b {
        b_freq.entry(v).and_modify(|i| *i += 1).or_insert(1);
    }
    a.into_iter()
        .map(|a| a * b_freq.get(&a).unwrap_or(&0))
        .sum()
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

    const TEST_INPUT: &str = "3   4
                              4   3
                              2   5
                              1   3
                              3   9
                              3   3";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 11);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 31);
    }
}
