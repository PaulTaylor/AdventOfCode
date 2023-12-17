use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<(String, Vec<usize>)> {
    lines
        .iter()
        .map(|l| {
            let mut split = l.split_whitespace();
            let pattern = split.next().unwrap();
            let groups = split
                .next()
                .unwrap()
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<usize>>();

            (pattern.to_string(), groups)
        })
        .collect()
}

#[allow(clippy::match_same_arms)]
fn solve<'a>(
    pattern: &'a str,
    groups: &'a [usize],
    left_in_group: Option<usize>,
    cache: &mut HashMap<(&'a str, &'a [usize], Option<usize>), usize>,
) -> usize {
    // params:
    // pattern - the remaining pattern characters
    // groups - the future groups that we have not yet started
    // left_in_group - the number of #'s remaining in the current group (or None if no group)
    // cache - the memoization cache

    // Memo' check
    let key = (pattern, groups, left_in_group);
    if let Some(&v) = cache.get(&key) {
        return v;
    }

    let valid = match (pattern.chars().next(), left_in_group) {
        // Terminating conditions first
        (None, None | Some(0)) if groups.is_empty() => {
            // We've reached the end of a valid solution as there are no more chars
            // there are no more groups, and we're either just ended, or are not in a group
            1
        }
        (None, _) => {
            // No more characters, but we've stuff left over - invalid
            0
        }
        // En-route conditions
        (Some('.'), None) => {
            // On a operational spring and not in a group - noop
            solve(&pattern[1..], groups, None, cache)
        }
        (Some('#'), None) => {
            // On a broken spring, and not in a group - so should start one
            if groups.is_empty() {
                // No groups to start though - invalid
                0
            } else {
                solve(&pattern[1..], &groups[1..], Some(groups[0] - 1), cache)
            }
        }
        (Some('?'), None) => {
            // On an unknown but not in a group - two options
            // Either continue as if we're a .
            let a = solve(&pattern[1..], groups, None, cache);
            // or start a group as if we're a #
            let b = if groups.is_empty() {
                // No groups to start - so # would be invalid here
                0
            } else {
                solve(&pattern[1..], &groups[1..], Some(groups[0] - 1), cache)
            };

            a + b
        }
        (Some('.'), Some(0)) => {
            // We hit a operational spring at the end of a group - good!
            solve(&pattern[1..], groups, None, cache)
        }
        (Some('#'), Some(0)) => {
            // We expected to be at the end of the group, but we're not so there is
            // nothing valid past here
            0
        }
        (Some('?'), Some(0)) => {
            // We expect to be at the end of the group - so the only possible correct answer is .
            solve(&pattern[1..], groups, None, cache)
        }
        (Some('.'), Some(_)) => {
            // Hit an operational spring when we expected to be inside a group - invalid
            0
        }
        (Some('#'), Some(x)) => {
            // We're happily continuing along our group of #s
            solve(&pattern[1..], groups, Some(x - 1), cache)
        }
        (Some('?'), Some(x)) => {
            // Unknown, but we expect to be inside a group here so act as a #
            solve(&pattern[1..], groups, Some(x - 1), cache)
        }
        _ => unreachable!(),
    };

    // Memo' insert
    cache.insert(key, valid);
    valid
}

fn part_a(lines: &[String]) -> usize {
    parse(lines)
        .iter()
        .map(|(pattern, groups)| solve(pattern, groups, None, &mut HashMap::new()))
        .sum()
}

fn part_b(lines: &[String]) -> usize {
    parse(lines)
        .into_iter()
        .map(|(pattern, groups)| ([pattern.as_str()].repeat(5).join("?"), groups.repeat(5)))
        .map(|(pattern, groups)| {
            let mut cache = HashMap::new();
            solve(&pattern, &groups, None, &mut cache)
        })
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

    const TEST_INPUT: &str = "???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 21);
    }

    #[test]
    fn test_b_small() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines[0..1]), 1);
        assert_eq!(part_b(&lines[5..]), 506_250);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 525_152);
    }
}
