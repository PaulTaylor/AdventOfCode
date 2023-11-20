use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<f32> {
    lines
        .iter()
        .map(|m| m.parse::<f32>().expect("a whole number"))
        .collect()
}

fn part_a(lines: &[String]) -> f32 {
    parse(lines).iter().map(|m| (m / 3.0).floor() - 2.0).sum()
}

fn fuel_with_fuel(m: f32) -> f32 {
    let mut fuel = 0f32;
    let mut next_fuel = m;
    while next_fuel > 0f32 {
        fuel += next_fuel;
        next_fuel = (next_fuel / 3.0).floor() - 2.0;
    }
    fuel
}

fn part_b(lines: &[String]) -> f32 {
    parse(lines)
        .iter()
        .map(|m| (m / 3.0).floor() - 2.0)
        .map(fuel_with_fuel)
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
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "12
    14
    1969
    100756";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()) as usize, 2 + 2 + 654 + 33583);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(&["14".to_string()]) as usize, 2);
        assert_eq!(part_b(&["1969".to_string()]) as usize, 966);
        assert_eq!(part_b(&["100756".to_string()]) as usize, 50346,);
    }
}
