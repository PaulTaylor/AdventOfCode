use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> AResult<Vec<(&str, u32)>> {
    let mut res = vec!();
    for l in lines {
        let mut it = l.split_whitespace();
        let dir = it.next().expect("no direction");
        let dst = it.next().expect("no distance").parse()?;

        res.push((dir, dst));
    }
    Ok(res)
}

fn part_a(lines: &[String]) -> AResult<u32> {
    let instructions = parse(lines)?;

    let mut h_pos = 0u32;
    let mut depth = 0u32;
    
    for (dir, dst) in instructions {
        match dir {
            "forward" => h_pos += dst,
            "down" => depth += dst,
            "up" => depth -= dst,
            _ => panic!("Unknown instruction ({} {})", dir, dst)
        }
    }

    Ok(h_pos * depth)
}

fn part_b(lines: &[String]) -> AResult<u32> {
    let instructions = parse(lines)?;

    let mut h_pos = 0u32;
    let mut depth = 0u32;
    let mut aim = 0u32;

    for (dir, dst) in instructions {
        match dir {
            "down" => aim += dst,
            "up" => aim -= dst,
            "forward" => { h_pos += dst; depth += aim * dst },
            _ => panic!("Unknown instruction ({} {})", dir, dst)
        }
    }
    
    Ok(h_pos * depth)
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

    const TEST_INPUT: &str = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 150);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 900);
        Ok(())
    }
}
