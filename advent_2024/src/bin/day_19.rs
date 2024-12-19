use humantime::format_duration;
use indicatif::ProgressIterator;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> (Vec<String>, &[String]) {
    let towels = lines[0].split(", ").map(ToString::to_string).collect();
    let patterns = &lines[2..];
    (towels, patterns)
}

fn check(pattern: &str, towels: &[String], cache: &mut HashMap<String, usize>) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(r) = cache.get(pattern) {
        return *r;
    }

    let mut acc = 0;
    for c in towels {
        if let Some(new_p) = pattern.strip_prefix(c) {
            let r = check(new_p, towels, cache);
            cache.insert(new_p.to_string(), r);
            acc += r;
        }
    }

    acc
}

#[allow(clippy::cast_possible_wrap)]
fn solution(lines: &[String]) -> (usize, usize) {
    let (towels, patterns) = parse(lines);
    let mut a_acc = 0;
    let mut b_acc = 0;
    let mut cache = HashMap::new();
    for pattern in patterns.iter().progress() {
        let count = check(pattern, &towels, &mut cache);
        if count > 0 {
            a_acc += 1usize;
            b_acc += count;
        };
    }

    (a_acc, b_acc)
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
    let (a, b) = solution(&lines);
    let start = Instant::now();
    println!("Part A result = {a}");
    println!("Part B result = {b}");
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

                              brwrr
                              bggr
                              gbbr
                              rrbgbr
                              ubwu
                              bwurrg
                              brgr
                              bbrgwb";

    #[test]
    fn test_solution() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(solution(&lines), (6, 16));
    }
}
