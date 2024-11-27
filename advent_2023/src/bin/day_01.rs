use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn part_a(lines: &[String]) -> usize {
    let p = Regex::new(r"^\D*(\d)?.*(\d)\D*$").unwrap();
    lines
        .iter()
        .map(|l| {
            let c = p.captures(l).unwrap();
            let d2: usize = c.get(2).and_then(|s| s.as_str().parse().ok()).unwrap();
            let d1 = c.get(1).and_then(|s| s.as_str().parse().ok()).unwrap_or(d2);
            (d1 * 10) + d2
        })
        .sum()
}

fn part_b(lines: &[String]) -> usize {
    let mut total = 0;
    for line in lines {
        let numbers: Vec<_> = (0..line.len())
            .filter_map(|e| match &line[e..] {
                s if s.starts_with(|c: char| c.is_ascii_digit()) => s.chars().next(),
                s if s.starts_with("one") => Some('1'),
                s if s.starts_with("two") => Some('2'),
                s if s.starts_with("three") => Some('3'),
                s if s.starts_with("four") => Some('4'),
                s if s.starts_with("five") => Some('5'),
                s if s.starts_with("six") => Some('6'),
                s if s.starts_with("seven") => Some('7'),
                s if s.starts_with("eight") => Some('8'),
                s if s.starts_with("nine") => Some('9'),
                _ => None,
            })
            .map(|c| (c as usize) - 48)
            .collect();

        let d1 = numbers[0];
        let d2 = numbers[numbers.len() - 1];
        total += (d1 * 10) + d2;
    }

    total
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
        const TEST_INPUT: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 142);
    }

    #[test]
    fn test_b() {
        const TEST_INPUT: &str = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 281);
    }
}
