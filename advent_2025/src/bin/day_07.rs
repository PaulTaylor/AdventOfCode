use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> (HashSet<(usize, usize)>, (usize, usize)) {
    let mut splitters = HashSet::new();
    let mut start = None;

    for (row, l) in lines.iter().enumerate() {
        for (col, c) in l.char_indices() {
            match c {
                '^' => {
                    splitters.insert((row, col));
                }
                'S' => start = Some((row, col)),
                _ => {}
            }
        }
    }

    (splitters, start.expect("A start position was not detected"))
}

fn part_a(lines: &[String]) -> usize {
    let (splitters, start) = parse(lines);

    let mut splits = 0;
    let mut beams = HashSet::from_iter([start]);
    while !beams.is_empty() {
        let mut new_beams = HashSet::new();
        for (row, col) in beams {
            if row == lines.len() {
                continue;
            }

            if splitters.contains(&(row + 1, col)) {
                splits += 1;
                new_beams.insert((row + 1, col - 1));
                new_beams.insert((row + 1, col + 1));
            } else {
                new_beams.insert((row + 1, col));
            }
        }
        beams = new_beams;
    }

    splits
}

fn part_b(lines: &[String]) -> usize {
    let (splitters, start) = parse(lines);

    let mut beams = BTreeMap::from_iter([(start, 1)]);
    loop {
        let n_beams = beams.values().sum();

        let mut new_beams = BTreeMap::new();
        for ((row, col), freq) in beams {
            if row == lines.len() {
                return n_beams;
            }

            if splitters.contains(&(row + 1, col)) {
                new_beams
                    .entry((row + 1, col - 1))
                    .and_modify(|v| *v += freq)
                    .or_insert(freq);
                new_beams
                    .entry((row + 1, col + 1))
                    .and_modify(|v| *v += freq)
                    .or_insert(freq);
            } else {
                new_beams
                    .entry((row + 1, col))
                    .and_modify(|v| *v += freq)
                    .or_insert(freq);
            }
        }
        beams = new_beams;
    }
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

    const TEST_INPUT: &str = ".......S.......
                              ...............
                              .......^.......
                              ...............
                              ......^.^......
                              ...............
                              .....^.^.^.....
                              ...............
                              ....^.^...^....
                              ...............
                              ...^.^...^.^...
                              ...............
                              ..^...^.....^..
                              ...............
                              .^.^.^.^.^...^.
                              ...............";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 21);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 40);
    }
}
