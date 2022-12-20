use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<Vec<usize>> {
    lines
        .iter()
        .map(|l| l.chars().map(|c| (c as usize) - ('0' as usize)).collect())
        .collect()
}

fn part_a(lines: &[String]) -> usize {
    let grid = parse(lines);

    // Perimiter trees are always visible
    let mut visible = (lines.len() * 2) + ((lines[0].len() - 2) * 2);

    for row in 1..grid.len() - 1 {
        for col in 1..grid[0].len() - 1 {
            let this = grid[row][col];
            // Left
            let left_height = grid[row][0..col].iter().max().unwrap();
            if left_height < &this {
                visible += 1;
                continue;
            }

            // Right
            let right_height = grid[row][col + 1..].iter().max().unwrap();
            if right_height < &this {
                visible += 1;
                continue;
            }

            // Up
            let up_height = grid[0..row].iter().map(|r| r[col]).max().unwrap();
            if up_height < this {
                visible += 1;
                continue;
            }

            // Down
            let down_height = grid[row + 1..].iter().map(|r| r[col]).max().unwrap();
            if down_height < this {
                visible += 1;
            }
        }
    }

    visible
}

fn scenic_score(grid: &[Vec<usize>], ri: usize, ci: usize) -> usize {
    let this = &grid[ri][ci];

    let lefts = &grid[ri][0..ci];
    let blocker = lefts.iter().rev().enumerate().find(|(_, x)| *x >= this);
    let left_score = match blocker {
        Some((idx, _)) => idx + 1,
        None if lefts.is_empty() => 0, // on the edge so 0
        None => lefts.len(),           // all trees in this direction are smaller than this
    };

    let rights = &grid[ri][ci + 1..];
    let blocker = rights.iter().enumerate().find(|(_, x)| *x >= this);
    let right_score = match blocker {
        Some((idx, _)) => idx + 1,
        None if rights.is_empty() => 0,
        None => rights.len(),
    };

    let ups = &grid[0..ri];
    let blocker = ups
        .iter()
        .map(|r| r[ci])
        .rev()
        .enumerate()
        .find(|(_, x)| x >= this);
    let up_score = match &blocker {
        Some((idx, _)) => idx + 1,
        None if ups.is_empty() => 0,
        None => ups.len(),
    };

    let downs = &grid[ri + 1..];
    let blocker = downs
        .iter()
        .map(|r| r[ci])
        .enumerate()
        .find(|(_, x)| x >= this);
    let down_score = match &blocker {
        Some((idx, _)) => idx + 1,
        None if downs.is_empty() => 0,
        None => downs.len(),
    };

    left_score * right_score * up_score * down_score
}

fn part_b(lines: &[String]) -> usize {
    let grid = parse(lines);
    let mut scores = Vec::with_capacity(grid.len() * grid[0].len());

    for (ri, row) in grid.iter().enumerate() {
        for (ci, _) in row.iter().enumerate() {
            scores.push(scenic_score(&grid, ri, ci));
        }
    }

    *scores.iter().max().unwrap()
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
    println!("Part B result = {}", part_b(lines.as_slice()));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "30373
    25512
    65332
    33549
    35390";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 21);
    }

    #[test]
    fn test_scenic_score() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let grid = parse(&lines);
        assert_eq!(scenic_score(&grid, 1, 2), 4);
        assert_eq!(scenic_score(&grid, 3, 2), 8);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 8);
    }
}
