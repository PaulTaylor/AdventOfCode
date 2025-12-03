use humantime::format_duration;
use itertools::Itertools;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn solve(bank: &[char], digits: usize, base: String) -> Option<usize> {
    // There are not enough digits left in the bank to provide a valid solution
    if digits > bank.len() {
        return None;
    }

    // Found a valid solution
    if digits == 0 {
        return base.parse().ok();
    }

    // Try each possible next digit in descending order
    let candidates = bank.iter().unique().sorted().rev();
    for d in candidates {
        let new_base = format!("{base}{d}");
        let d_idx = bank.iter().position(|v| v == d).unwrap();
        let (_, remaining) = bank.split_at(d_idx + 1);

        // Recurse to the next digit and return if solved
        if let Some(n) = solve(remaining, digits - 1, new_base) {
            return Some(n);
        }
    }

    panic!("No solution found :(")
}

fn part_a(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .flat_map(|bank| solve(&bank, 2, String::new()))
        .sum()
}

fn part_b(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .flat_map(|bank| solve(&bank, 12, String::new()))
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

    const TEST_INPUT: &str = "987654321111111
                              811111111111119
                              234234234234278
                              818181911112111";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 357);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 3121910778619);
    }
}
