use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn part_a(lines: &[String]) -> usize {
    let pattern = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut acc = 0usize;
    for line in lines {
        for caps in pattern.captures_iter(line) {
            let (_, [a, b]): (&str, [&str; 2]) = caps.extract();
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();
            acc += a * b;
        }
    }
    acc
}

fn part_b(lines: &[String]) -> usize {
    let pattern = Regex::new(r"(?:do(?:n't)?)\(\)|(?:mul\(([0-9]{1,3}),([0-9]{1,3})\))").unwrap();

    let mut acc = 0usize;
    let mut active = true;
    for line in lines {
        for caps in pattern.captures_iter(line) {
            let strs: Vec<_> = caps.iter().map(|m| m.map(|m| m.as_str())).collect();

            match strs.as_slice() {
                [Some("do()"), ..] => active = true,
                [Some("don't()"), ..] => active = false,
                [_, Some(a), Some(b)] if active => {
                    let a: usize = a.parse().unwrap();
                    let b: usize = b.parse().unwrap();
                    acc += a * b;
                }
                _ => {}
            }
        }
    }
    acc
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

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT_B: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 161);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT_B.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 48);
    }
}
