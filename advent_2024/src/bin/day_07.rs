use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<(isize, Vec<isize>)> {
    let pattern = Regex::new(r"(\d+)").unwrap();
    lines
        .iter()
        .filter_map(|l| -> Option<_> {
            let mut matches = pattern.find_iter(l);
            Some((
                matches.next()?.as_str().parse().ok()?,
                matches
                    .filter_map(|m| -> Option<isize> { m.as_str().parse().ok() })
                    .collect::<Vec<isize>>(),
            ))
        })
        .collect()
}

fn extend(mut acc: isize, op: char, remaining: &[isize], target: isize, ops: &[char]) -> bool {
    if remaining.is_empty() {
        return acc == target;
    }

    // Since there's no way to reduce the accumulator we can bail out early if `acc > target`
    if acc > target {
        return false;
    }

    acc = match op {
        '+' => acc + remaining[0],
        '*' => acc * remaining[0],
        '|' => format!("{acc}{}", remaining[0]).parse().unwrap(),
        _ => unreachable!(),
    };

    for op in ops {
        if extend(acc, *op, &remaining[1..], target, ops) {
            return true;
        }
    }

    false
}

fn part_a(lines: &[String]) -> isize {
    let ops: [char; 2] = ['+', '*'];
    let mut acc = 0;
    for (target, numbers) in parse(lines) {
        for op in ops {
            if extend(numbers[0], op, &numbers[1..], target, &ops) {
                acc += target;
                break;
            }
        }
    }

    acc
}

fn part_b(lines: &[String]) -> isize {
    let ops: [char; 3] = ['+', '*', '|'];
    let mut acc = 0;
    for (target, numbers) in parse(lines) {
        for op in ops {
            if extend(numbers[0], op, &numbers[1..], target, &ops) {
                acc += target;
                break;
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

    const TEST_INPUT: &str = "190: 10 19
                              3267: 81 40 27
                              83: 17 5
                              156: 15 6
                              7290: 6 8 6 15
                              161011: 16 10 13
                              192: 17 8 14
                              21037: 9 7 18 13
                              292: 11 6 16 20";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 3749);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 11387);
    }
}
