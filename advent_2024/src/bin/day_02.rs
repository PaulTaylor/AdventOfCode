use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<Vec<isize>> {
    lines
        .iter()
        .map(|l| l.split_whitespace().flat_map(str::parse).collect())
        .collect()
}

fn check_report(report: &[isize], can_remove: bool) -> bool {
    let diffs: Vec<_> = report.windows(2).map(|w| w[1] - w[0]).collect();
    let mut safe = diffs.iter().all(|v| (1..=3).contains(v));

    if can_remove && !safe {
        let unsafe_idx = diffs
            .iter()
            .enumerate()
            .find_map(|(i, v)| (!(1..=3).contains(v)).then_some(i));

        if let Some(ui) = unsafe_idx {
            // Need to check both cases where one of ui/ui+1 is removed
            let mut with_removed: Vec<isize> = report.to_vec();
            with_removed.remove(ui);
            safe = check_report(&with_removed, false);

            if !safe {
                with_removed[ui] = report[ui];
                safe = check_report(&with_removed, false);
            }
        }
    }
    safe
}

fn part_a(lines: &[String]) -> usize {
    let reports = parse(lines);

    let mut safe = 0;
    for mut report in reports {
        if check_report(&report, false) {
            safe += 1;
        } else {
            report.reverse();
            safe += usize::from(check_report(&report, false));
        }
    }

    safe
}

fn part_b(lines: &[String]) -> usize {
    let reports = parse(lines);

    let mut safe = 0;
    for mut report in reports {
        if check_report(&report, true) {
            safe += 1;
        } else {
            report.reverse();
            safe += usize::from(check_report(&report, true));
        }
    }

    safe
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

    const TEST_INPUT: &str = "7 6 4 2 1
                              1 2 7 8 9
                              9 7 6 2 1
                              1 3 2 4 5
                              8 6 4 4 1
                              1 3 6 7 9";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 2);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 4);
    }
}
