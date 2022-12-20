use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<(u32, u32, u32, u32)> {
    let mut out: Vec<(u32, u32, u32, u32)> = Vec::with_capacity(lines.len());
    for line in lines {
        let nums: Vec<_> = line
            .split(',')
            .map(|l| -> Vec<_> { l.split('-').collect() })
            .map(|ll| -> Vec<_> { ll.iter().map(|v| v.parse::<u32>().unwrap()).collect() })
            .collect();

        out.push((nums[0][0], nums[0][1], nums[1][0], nums[1][1]));
    }

    out
}

fn part_a(lines: &[String]) -> u32 {
    let parts = parse(lines);
    let mut acc = 0u32;
    for (a, b, c, d) in parts {
        if (a <= c && b >= d) | (c <= a && d >= b) {
            acc += 1;
        }
    }
    acc
}

fn part_b(lines: &[String]) -> u32 {
    let parts = parse(lines);
    let mut acc = 0u32;
    for (a, b, c, d) in parts {
        if (a <= c && c <= b && b <= d)  // a c b d
         | (c <= a && a <= d && d <= b)  // c a d b
         | (a <= c && c <= d && d <= b)  // a c d b
         | (c <= a && a <= b && b <= d)  // c a b d
         | false
        // the false just stops the auto-formatter moving my comments
        {
            acc += 1;
        }
    }

    acc
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
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 2);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 4);
    }
}
