use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
    time::Instant
};

fn parse(lines: &[String]) -> Vec<usize> {
    return lines
        .iter()
        .map(|l| (*l).parse::<usize>().unwrap())
        .collect();
}

fn part_a(lines: &[String]) -> usize {
    let parsed = parse(lines);
    return zip(parsed.iter(), parsed[1..].iter())
        .filter(|(prev, next)| next > prev)
        .count();
}

fn part_b(lines: &[String]) -> usize {
    let parsed = parse(lines);
    zip(parsed.windows(3), parsed[1..].windows(3))
        .map(|(p, n)| -> (usize, usize) { (p.iter().sum(), n.iter().sum()) })
        .filter(|(p, n)| p < n)
        .count()
}

fn main() -> anyhow::Result<()> {
    // Figure out which exercise we are so we can load the correct input file
    let pattern = Regex::new("[0-9]+$")?;
    let name = &std::env::args().next().unwrap();
    let ex = pattern
        .find(name)
        .expect("binary name should contain a number")
        .as_str();
    println!("Running code for Day {}.", ex);

    // Load the appropriate input text
    let file = File::open(format!("./data/day_{ex}_a.txt"))?;
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    // Run the solutions
    let start = Instant::now();
    println!("Part A result = {}", part_a(&lines));
    println!("Part B result = {}", part_b(&lines));
    let end = Instant::now();
    
    println!("Run took {}ms", (end - start).as_secs_f32() * 1000.0);

    // Exit cleanly
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

    #[test]
    fn test_a() {
        let lines: Vec<String> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(&lines), 7);
    }

    #[test]
    fn test_b() {
        let lines: Vec<String> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 5);
    }
}
