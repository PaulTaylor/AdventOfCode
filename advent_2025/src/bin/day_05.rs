use humantime::format_duration;
use itertools::Itertools;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut ranges = vec![];
    let mut items = vec![];

    for line in lines {
        match line {
            l if l.contains('-') => {
                let mut bits = l.split("-");
                let a: usize = bits.next().unwrap().parse().unwrap();
                let b: usize = bits.next().unwrap().parse().unwrap();
                ranges.push((a, b));
            }
            l if !l.is_empty() => items.push(l.parse().unwrap()),
            _ => {}
        }
    }

    (ranges, items)
}

fn part_a(lines: &[String]) -> usize {
    let (ranges, items) = parse(lines);

    items
        .into_iter()
        .map(|i| ranges.iter().any(|&(a, b)| (a..=b).contains(&i)) as usize)
        .sum()
}

fn part_b(lines: &[String]) -> usize {
    parse(lines)
        .0
        .into_iter()
        .sorted()
        .fold(vec![], |mut acc, (a, b)| {
            match acc.last() {
                Some(&(x, y)) if (x..=y).contains(&a) => {
                    // overlap - push out y to be max(y, b)
                    let last_idx = acc.len() - 1;
                    acc[last_idx] = (x, y.max(b));
                    acc
                }
                Some(_) | None => {
                    // no overlap - start a new range
                    acc.push((a, b));
                    acc
                }
            }
        })
        .into_iter()
        .map(|(a, b)| (b - a) + 1)
        .sum()
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

    const TEST_INPUT: &str = "3-5
                              10-14
                              16-20
                              12-18

                              1
                              5
                              8
                              11
                              17
                              32";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 3);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 14);
    }
}
