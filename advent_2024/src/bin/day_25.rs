use humantime::format_duration;
use itertools::Itertools;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn part_a(lines: &[String]) -> usize {
    let blocks = lines.split(String::is_empty);
    let max_depth = 6usize;
    let mut locks = vec![];
    let mut keys = vec![];

    for block in blocks {
        let mut block = block.to_vec();
        let mut acc = (0..block[0].len()).map(|_| 0usize).collect_vec();
        let is_lock = block[0].chars().all(|c| c == '#');

        if !is_lock {
            block.reverse();
        }

        for row in &block[1..] {
            for (idx, c) in row.char_indices() {
                if c == '#' {
                    acc[idx] += 1;
                }
            }
        }

        if is_lock {
            locks.push(acc);
        } else {
            keys.push(acc);
        }
    }

    let mut acc = 0;
    for lock in &locks {
        for key in &keys {
            let does_fit = key
                .iter()
                .zip(lock.iter())
                .map(|(k, l)| k + l)
                .all(|v| v < max_depth);

            acc += usize::from(does_fit);
        }
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
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));
    println!("Merry christmas!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#####
                              .####
                              .####
                              .####
                              .#.#.
                              .#...
                              .....

                              #####
                              ##.##
                              .#.##
                              ...##
                              ...#.
                              ...#.
                              .....

                              .....
                              #....
                              #....
                              #...#
                              #.#.#
                              #.###
                              #####

                              .....
                              .....
                              #.#..
                              ###..
                              ###.#
                              ###.#
                              #####

                              .....
                              .....
                              .....
                              #....
                              #.#..
                              #.#.#
                              #####";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 3);
    }
}
