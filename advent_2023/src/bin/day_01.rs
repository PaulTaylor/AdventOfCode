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
            let c = p.captures(l);
            if let Some(caps) = c {
                let m1 = caps.get(1);
                let m2 = caps.get(2);

                match (m1, m2) {
                    (Some(d1), Some(d2)) => {
                        (10 * d1.as_str().parse::<usize>().unwrap())
                            + d2.as_str().parse::<usize>().unwrap()
                    }
                    (None, Some(d1)) => {
                        (10 * d1.as_str().parse::<usize>().unwrap())
                            + d1.as_str().parse::<usize>().unwrap()
                    }
                    _ => panic!("Something unexpected happened"),
                }
            } else {
                0
            }
        })
        .sum()
}

fn part_b(lines: &[String]) -> usize {
    let mut total = 0;
    for line in lines {
        // Find the first digit through forward search
        let mut d1 = '!';
        for offset in 0..line.len() {
            let segment = &line[offset..];

            let found = match segment {
                s if s.starts_with(|c: char| c.is_ascii_digit()) => segment.chars().next(),
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
            };

            if let Some(d) = found {
                d1 = d;
                break;
            }
        }

        assert_ne!(d1, '!', "The first digit was not found");

        // Search backwards for the last one
        let mut d2 = '!';
        for offset in (1..=line.len()).rev() {
            let segment = &line[0..offset];

            let found = match segment {
                s if s.ends_with(|c: char| c.is_ascii_digit()) => segment.chars().last(),
                s if s.ends_with("one") => Some('1'),
                s if s.ends_with("two") => Some('2'),
                s if s.ends_with("three") => Some('3'),
                s if s.ends_with("four") => Some('4'),
                s if s.ends_with("five") => Some('5'),
                s if s.ends_with("six") => Some('6'),
                s if s.ends_with("seven") => Some('7'),
                s if s.ends_with("eight") => Some('8'),
                s if s.ends_with("nine") => Some('9'),
                _ => None,
            };

            if let Some(d) = found {
                d2 = d;
                break;
            }
        }

        assert_ne!(d2, '!', "The last digit was not found");

        let num = format!("{d1}{d2}");
        total += num.parse::<usize>().unwrap();
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
