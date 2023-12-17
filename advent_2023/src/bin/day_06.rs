use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> [Vec<usize>; 2] {
    [
        lines[0]
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect(),
        lines[1]
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect(),
    ]
}

fn part_a(lines: &[String]) -> usize {
    // Lets just do a simple iterator on this part :)

    let [times, distances] = parse(lines);

    let mut acc = 1;
    for (total_time, max_distance) in times.into_iter().zip(distances) {
        let mut wins = 0;
        for hold_time in 0..=total_time {
            let d = hold_time * (total_time - hold_time);
            if d > max_distance {
                wins += 1;
            }
        }
        acc *= std::cmp::max(1, wins);
    }
    acc
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn part_b(lines: &[String]) -> usize {
    // Solution is the roots of the equation
    // h(t - h) - d = 0
    // h = hold time, t = total_time, d = distance to beat

    let total_time = lines[0]
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse::<f64>()
        .unwrap();

    let max_distance = lines[1]
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse::<f64>()
        .unwrap();

    // Rolling back the years to GCSE Maths 🙂
    let r1 = (-total_time + (total_time.powf(2.0) - (4.0 * max_distance)).sqrt()) / -2.0;
    let r2 = (-total_time - (total_time.powf(2.0) - (4.0 * max_distance)).sqrt()) / -2.0;

    (r2.floor() - r1.ceil()) as usize + 1
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

    const TEST_INPUT: &str = "Time:      7  15   30
    Distance:  9  40  200";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 288);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 71503);
    }
}
