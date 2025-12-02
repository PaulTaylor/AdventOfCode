use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<(char, isize)> {
    lines
        .iter()
        .map(|l| {
            let (dir, num) = l.split_at(1);
            (dir.chars().next().unwrap(), isize::from_str(num).unwrap())
        })
        .collect()
}

fn part_a(lines: &[String]) -> usize {
    let mut acc = 0;
    let mut pos = 50isize;
    for (dir, freq) in parse(lines) {
        pos = match dir {
            'L' => pos - freq,
            'R' => pos + freq,
            _ => panic!("Unknown direction"),
        }
        .rem_euclid(100);

        acc += (pos == 0) as usize
    }

    acc
}

fn part_b(lines: &[String]) -> usize {
    let mut acc = 0;
    let mut pos = 50isize;
    for (dir, freq) in parse(lines) {
        let wraps = freq.div_euclid(100).unsigned_abs();
        let rem = freq.rem_euclid(100);

        let new_pos = match dir {
            'L' => pos - rem,
            'R' => pos + rem,
            _ => panic!("Unknown direction"),
        }
        .rem_euclid(100);

        acc += wraps
            + match dir {
                _ if new_pos == 0 => 1,
                'L' if pos > 0 && pos < new_pos => 1,
                'R' if pos > new_pos => 1,
                _ => 0,
            };
        pos = new_pos;
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

    const TEST_INPUT: &str = "L68
                              L30
                              R48
                              L5
                              R60
                              L55
                              L1
                              L99
                              R14
                              L82";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 3);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&vec![String::from("L50"), String::from("L1")]), 1);
        assert_eq!(part_b(&vec![String::from("L50"), String::from("R1")]), 1);
        assert_eq!(part_b(&vec![String::from("R200")]), 2);
        assert_eq!(part_b(&vec![String::from("L200")]), 2);
        assert_eq!(part_b(&lines), 6);
        assert_eq!(part_b(&vec![String::from("R1000")]), 10);
    }
}
