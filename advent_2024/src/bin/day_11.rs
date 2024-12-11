use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn step(num: &String) -> Vec<String> {
    if num == "0" {
        return vec!["1".to_string()];
    }

    if num.len().rem_euclid(2) == 0 {
        let trim_pattern: Regex = Regex::new(r"^(0+)([0-9]+)$").unwrap();
        let split = num.len() / 2;
        let left = trim_pattern.replace(&num[..split], "$2").to_string();
        let right = trim_pattern.replace(&num[split..], "$2").to_string();
        return vec![left, right];
    }

    let value: usize = num.parse().unwrap();
    vec![format!("{}", value * 2024)]
}

fn part_ab(lines: &[String], blinks: usize) -> usize {
    let mut stones: HashMap<String, usize> = HashMap::new();
    for number in lines[0].split_whitespace() {
        stones
            .entry(number.to_string())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    for _ in 0..blinks {
        let mut next = HashMap::new();
        for (value, count) in stones {
            for num in step(&value) {
                next.entry(num).and_modify(|v| *v += count).or_insert(count);
            }
        }
        stones = next;
    }

    stones.values().sum()
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

    // Run the part_abs
    let start = Instant::now();
    println!("Part A result = {}", part_ab(lines.as_slice(), 25));
    println!("Part B result = {}", part_ab(lines.as_slice(), 75));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_ab(lines.as_slice(), 6), 22);
        assert_eq!(part_ab(lines.as_slice(), 25), 55312);
    }
}
