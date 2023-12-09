use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<Vec<isize>> {
    lines
        .iter()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect()
}

fn next_value(seq: &[isize]) -> isize {
    // If the input sequence is all zeros the next value is also zero
    if seq.iter().all(|&v| v == 0) {
        return 0;
    }

    let diffs: Vec<isize> = seq
        .iter()
        .zip(seq.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();

    // Next value is last input value + next value in the diffs sequence
    let delta = next_value(&diffs);
    seq.last().unwrap() + delta
}

fn part_a(lines: &[String]) -> isize {
    parse(lines).iter().map(|s| next_value(s)).sum()
}

fn part_b(lines: &[String]) -> isize {
    parse(lines)
        .iter()
        .map(|s| {
            let reversed: Vec<_> = s.iter().copied().rev().collect();
            next_value(reversed.as_slice())
        })
        .sum()
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

    const TEST_INPUT: &str = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 114);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines[2..]), 5);
        assert_eq!(part_b(lines.as_slice()), 2);
    }
}
