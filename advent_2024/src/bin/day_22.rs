use humantime::format_duration;
use indicatif::ProgressIterator;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn generate_sequence(seed: isize) -> [isize; 2001] {
    let mut output = [0; 2001];
    output[0] = seed;

    let mut secret = seed;

    for val in output.iter_mut().skip(1) {
        let s64 = secret * 64;
        secret = (secret ^ s64) % 16_777_216;

        let d32 = secret / 32;
        secret = (secret ^ d32) % 16_777_216;

        let s2048 = secret * 2048;
        secret = (secret ^ s2048) % 16_777_216;

        *val = secret;
    }

    output
}

fn part_a(lines: &[String]) -> isize {
    let seeds: Vec<isize> = lines.iter().flat_map(|s| s.parse()).collect();
    seeds
        .iter()
        .map(|&seed| generate_sequence(seed).last().copied().unwrap())
        .sum()
}

fn part_b(lines: &[String]) -> isize {
    let seeds: Vec<isize> = lines.iter().flat_map(|s| s.parse()).collect();

    let mut seq_values = BTreeMap::new();
    for seed in seeds.into_iter().progress() {
        // Generate the sequence
        let seq = generate_sequence(seed);

        // Setup a series of lazy maps to compute appropriate sequence values
        let prices = seq.iter().map(|v| v.rem_euclid(10));
        let diffs = prices.clone().tuple_windows().map(|(a, b)| b - a);
        let diff_windows = diffs.tuple_windows();

        // Store the price after the first instance of this 4 element sequence for later
        for (w, price) in diff_windows.zip(prices.skip(4)) {
            let (_, _, _, _) = w;
            seq_values.entry((seed, w)).or_insert(price);
        }
    }

    // Compute the total number of bananas that would be sold after each sequence
    let mut totals = BTreeMap::new();
    for ((_seed, seq), val) in seq_values.into_iter().progress() {
        totals.entry(seq).and_modify(|l| *l += val).or_insert(val);
    }

    // Return the most!
    *totals.values().max().unwrap()
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

    #[test]
    fn test_a() {
        const TEST_INPUT: &str = "1\n10\n100\n2024";
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 37_327_623);
    }

    #[test]
    fn test_b() {
        const TEST_INPUT: &str = "1\n2\n3\n2024";
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 23);
    }
}
