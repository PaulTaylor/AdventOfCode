use core::panic;
use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn from_snafu(s: &str) -> isize {
    let mut digits = vec![];
    for c in s.chars() {
        let d = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        };
        digits.push(d);
    }

    let mut acc = 0;
    for d in digits {
        acc *= 5;
        acc += d;
    }
    acc
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn to_snafu(mut n: isize) -> String {
    let mut digits = vec![];

    // Regular b5 digits
    while n > 0 {
        digits.insert(0, n % 5);
        n /= 5;
    }

    // Add some extras in case of overflow later
    (0..5).for_each(|_| digits.insert(0, 0));

    // Apply the "smearing"
    let nd = digits.len();
    for i in (0..nd).rev() {
        if digits[i] > 2 {
            digits[i] -= 5;
            digits[i - 1] += 1;
        }
    }

    // Render digits into the string
    digits
        .iter()
        .map(|c| match c {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("unknown c {c}"),
        })
        .collect::<String>()
        .trim_start_matches('0')
        .to_string()
}

fn part_a(lines: &[String]) -> String {
    let tot = lines.iter().map(|s| from_snafu(s)).sum();
    to_snafu(tot)
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
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1=-0-2
        12111
        2=0=
        21
        2=01
        111
        20012
        112
        1=-1=
        1-12
        12
        1=
        122";

    const examples: [(isize, &'static str); 15] = [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314_159_265, "1121-1110-1=0"),
    ];

    #[test]
    fn test_from_snafu() {
        for (dec, snaf) in examples {
            assert_eq!(dec, from_snafu(snaf), "{snaf}");
        }
    }

    #[test]
    fn test_to_snafu() {
        for (dec, snaf) in examples {
            assert_eq!(to_snafu(dec), snaf, "{dec}");
        }
    }

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), "2=-1=0");
    }
}
