use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn part_a(lines: &[String]) -> AResult<u64> {
    let mut acc = 0u64;
    for line in lines {
        let mid = line.len() / 2;
        let (a, b): (HashSet<char>, HashSet<char>) = (
            (line[0..mid]).chars().collect(),
            (line[mid..]).chars().collect(),
        );

        let common: Vec<&char> = a.intersection(&b).collect();
        let val = match common[0] {
            'A'..='Z' => 27 + (*common[0] as u64) - ('A' as u64),
            'a'..='z' => 1 + (*common[0] as u64) - ('a' as u64),
            _ => {
                panic!();
            }
        };
        acc += val;
    }

    Ok(acc)
}

fn part_b(lines: &[String]) -> AResult<u64> {
    let mut acc = 0u64;
    for group in lines.chunks_exact(3) {
        let mut items: HashMap<char, u32> = HashMap::new();

        for line in group {
            let unique: HashSet<_> = HashSet::from_iter(line.chars());
            for c in unique {
                items.entry(c).and_modify(|v| *v += 1).or_insert(1);
            }
        }

        let (badge, _) = items.into_iter().find(|(_, v)| *v == 3).unwrap();
        acc += match badge {
            'A'..='Z' => 27 + (badge as u64) - ('A' as u64),
            'a'..='z' => 1 + (badge as u64) - ('a' as u64),
            _ => {
                panic!();
            }
        };
    }

    Ok(acc)
}

fn main() -> AResult<()> {
    // Figure out which exercise we are so we can load the correct input file
    let pattern = Regex::new("[0-9]+$")?;
    let name = &std::env::args().next().expect("binary name not found.");
    let ex = pattern
        .find(name)
        .expect("binary name should contain a number")
        .as_str();
    println!("Running code for Day {}.", ex);

    // Load the appropriate input text
    let file = File::open(format!("./data/day_{ex}.txt"))?;
    let lines: Vec<String> = BufReader::new(file).lines().map(Result::unwrap).collect();

    // Run the solutions
    let start = Instant::now();
    println!("Part A result = {}", part_a(lines.as_slice())?);
    println!("Part B result = {}", part_b(lines.as_slice())?);
    let end = Instant::now();

    println!("Run took {}ms", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 157);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 70);
        Ok(())
    }
}
