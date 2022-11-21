use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> AResult<Vec<u16>> {
    Ok(lines
        .iter()
        .map(|l| (*l).parse::<u16>())
        .map(Result::unwrap)
        .collect())
}

fn part_a(lines: &[String]) -> AResult<u16> {
    let mut it = parse(lines)?.into_iter();

    let mut acc = 0u16;
    let mut prev = it.next().unwrap();

    for next in it {
        acc += (next > prev) as u16;
        prev = next;
    }

    Ok(acc)
}

fn part_b(lines: &[String]) -> AResult<u16> {
    let parsed = parse(lines)?;
    let mut it = parsed.windows(3);

    let mut acc = 0u16;
    let mut prev = it.next().unwrap();

    for next in it {
        let psum: u16 = prev.iter().sum();
        let nsum: u16 = next.iter().sum();

        acc += (nsum > psum) as u16;
        prev = next;
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
    let file = File::open(format!("./data/day_{ex}_a.txt"))?;
    let lines: Vec<String> = BufReader::new(file).lines()
        .map(Result::unwrap)
        .collect();

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
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 7);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 5);
        Ok(())
    }
}
