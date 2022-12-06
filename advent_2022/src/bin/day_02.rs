use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> AResult<Vec<(i32, i32)>> {
    let f = |x: &str| match x.chars().next().unwrap() {
        'A' | 'X' => 0,
        'B' | 'Y' => 1,
        'C' | 'Z' => 2,
        _ => panic!(),
    };

    Ok(lines
        .iter()
        .map(|l| -> Vec<&str> { l.split_whitespace().collect() })
        .map(|v| (f(v[0]), f(v[1])))
        .collect())
}

fn part_a(lines: &[String]) -> AResult<i64> {
    let rounds = parse(lines)?;

    let mut acc = 0i32;
    for (a, b) in rounds {
        let mut wld = ((b - a).rem_euclid(3) + 1) * 3;
        if wld > 6 {
            wld = 0; // deal with the wrap around case
        }
        let me = b + 1;
        acc += wld + me;
    }

    Ok(acc.into())
}

fn part_b(lines: &[String]) -> AResult<i64> {
    let rounds = parse(lines)?;
    let mut acc = 0;

    for (a, b) in rounds {
        let res = match b {
            0 => (a - 1).rem_euclid(3) + 1,     // lose - i choose a - 1
            1 => 3 + a + 1,                     // draw - i choose the same as a
            2 => 6 + (a + 1).rem_euclid(3) + 1, // win  - i choose a + 1
            _ => panic!(),
        };
        acc += res;
    }

    Ok(acc.into())
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

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
    B X
    C Z";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 15);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 12);
        Ok(())
    }
}
