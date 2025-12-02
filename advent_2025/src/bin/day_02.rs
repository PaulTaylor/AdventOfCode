use fancy_regex::Regex as FancyRegex;
use humantime::format_duration;
use itertools::Itertools;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn solve(lines: &[String], modifier: &str) -> usize {
    let ranges: Vec<(usize, usize)> = lines[0]
        .split(',')
        .filter_map(|s| s.split('-').map(|s| s.parse().unwrap()).collect_tuple())
        .collect();

    let pattern = FancyRegex::new(&format!("^(\\d+)\\1{modifier}$")).unwrap();

    let mut acc = 0;
    for (min, max) in ranges {
        for val in min..=max {
            let val_str = format!("{val}");
            if pattern.is_match(&val_str).unwrap() {
                acc += val;
            }
        }
    }

    acc
}

fn part_a(lines: &[String]) -> usize {
    solve(lines, "")
}

fn part_b(lines: &[String]) -> usize {
    solve(lines, "+")
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

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 1_227_775_554);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 4_174_379_265);
    }
}
