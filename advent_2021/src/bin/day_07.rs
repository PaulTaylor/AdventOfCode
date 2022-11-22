use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> AResult<Vec<u32>> {
    Ok(lines[0].split(',').map(|s| s.parse().unwrap()).collect())
}

fn part_a(lines: &[String]) -> AResult<u32> {
    let init_pos = parse(lines)?;
    let max_pos = *(init_pos.iter().max().unwrap());

    let mut costs: HashMap<u32, u32> = HashMap::new();

    let min_pos = (0..max_pos)
        .min_by_key(|pos| -> u32 {
            let cost: u32 = init_pos.iter().map(|p| pos.abs_diff(*p)).sum();
            costs.insert(*pos, cost);
            cost
        })
        .unwrap();

    Ok(*(costs.get(&min_pos).unwrap()))
}

fn part_b(lines: &[String]) -> AResult<u32> {
    let init_pos = parse(lines)?;
    let max_pos = *(init_pos.iter().max().unwrap());

    let mut costs: HashMap<u32, u32> = HashMap::new();
    let min_pos = (0..max_pos)
        .min_by_key(|pos| -> u32 {
            let cost: u32 = init_pos
                .iter()
                .map(|p| pos.abs_diff(*p))
                .map(|dist| -> u32 { (1..=dist).sum() })
                .sum();
            costs.insert(*pos, cost);
            cost
        })
        .unwrap();

    Ok(*(costs.get(&min_pos).unwrap()))
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

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 37);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 168);
        Ok(())
    }
}
