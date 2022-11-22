use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> AResult<Vec<u64>> {
    Ok(lines[0].split(',').map(|s| s.parse().unwrap()).collect())
}

fn solve(lines: &[String], days: u32) -> AResult<u64> {
    let initial_ages = parse(lines)?;

    let mut ages = [0u64; 9];
    for a in &initial_ages {
        ages[*a as usize] += 1;
    }

    for _day in 0..days {
        let zero_fish = ages[0];
        for a in 1..ages.len() {
            ages[(a-1) as usize] = ages[a] as u64;
        }
        ages[6] += zero_fish;
        ages[8] = zero_fish;
    }


    Ok(ages.iter().sum::<u64>())
}

fn part_a(lines: &[String]) -> AResult<u64> {
    solve(lines, 80)
}

fn part_b(lines: &[String]) -> AResult<u64> {
    solve(lines, 256)
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
    let file = File::open(format!("./data/day_{ex}_a.txt"))?;
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

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 5934);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 26984457539u64);
        Ok(())
    }
}
