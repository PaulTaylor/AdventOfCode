use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn part_a(line: &str) -> AResult<usize> {
    solve(line, 4)
}

fn part_b(line: &str) -> AResult<usize> {
    solve(line, 14)
}

fn solve(line: &str, window_size: usize) -> AResult<usize> {
    let chars: Vec<_> = line.chars().collect();
    for (i, grp) in chars.windows(window_size).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(grp);
        if set.len() == window_size {
            return Ok(i + window_size);
        }
    }
    Err(anyhow::format_err!("Not found"))
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
    println!("Part A result = {}", part_a(lines.first().unwrap())?);
    println!("Part B result = {}", part_b(lines.first().unwrap())?);
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [(&str, usize); 4] = [
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    const TEST_INPUT_B: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];

    #[test]
    fn test_a() -> AResult<()> {
        for (line, exp) in TEST_INPUT {
            assert_eq!(part_a(line)?, exp);
        }
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        for (line, exp) in TEST_INPUT_B {
            assert_eq!(part_b(line)?, exp);
        }
        Ok(())
    }
}
