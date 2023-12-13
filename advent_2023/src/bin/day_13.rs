use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<Vec<Vec<char>>> {
    lines
        .split(String::is_empty)
        .map(|group| group.iter().map(|s| s.chars().collect()).collect())
        .collect()
}

fn transpose(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let col_count = grid[0].len();
    (0..col_count)
        .map(|i| grid.iter().map(|row| row[i]).collect())
        .collect()
}

fn _grid_string(grid: &[Vec<char>]) -> String {
    let s: String = grid
        .iter()
        .map(|row| -> String {
            let mut s: String = row.iter().collect();
            s.push('\n');
            s
        })
        .collect();

    s.trim_end().to_string()
}

fn find_vertical_lor(grid: &[Vec<char>], is_b: bool) -> Option<usize> {
    find_horizontal_lor(&transpose(grid), is_b)
}

fn find_horizontal_lor(grid: &[Vec<char>], is_b: bool) -> Option<usize> {
    // Scan down the grid and check each location for a reflection
    for row_after_lof in 1..grid.len() {
        let slice_size = std::cmp::min(row_after_lof, grid.len() - row_after_lof);

        let before = &grid[row_after_lof - slice_size..row_after_lof];
        let after = &grid[row_after_lof..row_after_lof + slice_size];

        assert_eq!(before.len(), after.len());

        // The smudged fold will have exactly 1 error and a perfect match 0
        let errors: usize = before
            .iter()
            .zip(after.iter().rev())
            .map(|(row_a, row_b)| row_a.iter().zip(row_b).filter(|(a, b)| a != b).count())
            .sum();

        if (!is_b && errors == 0) || (is_b && errors == 1) {
            return Some(row_after_lof);
        }
    }

    None
}

fn part_a(lines: &[String]) -> usize {
    let mut left_of_vertical = 0usize;
    let mut above_horizontal = 0usize;

    for grid in parse(lines) {
        if let Some(n) = find_vertical_lor(&grid, false) {
            left_of_vertical += n;
        } else if let Some(n) = find_horizontal_lor(&grid, false) {
            above_horizontal += n;
        } else {
            unreachable!();
        }
    }

    left_of_vertical + (100 * above_horizontal)
}

fn part_b(lines: &[String]) -> usize {
    let mut left_of_vertical = 0usize;
    let mut above_horizontal = 0usize;
    for grid in parse(lines) {
        // Determine the part_a answer (so we know what to avoid)
        let a_h = find_horizontal_lor(&grid, false);
        let a_v = find_vertical_lor(&grid, false);

        let b_h = find_horizontal_lor(&grid, true);
        let b_v = find_vertical_lor(&grid, true);

        // Make doubly sure a different line is detected than in part a
        assert_ne!((a_h, a_v), (b_h, b_v));

        if let Some(n) = b_v {
            left_of_vertical += n;
        } else if let Some(n) = b_h {
            above_horizontal += n;
        } else {
            unreachable!();
        }
    }

    left_of_vertical + (100 * above_horizontal)
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

    const TEST_INPUT: &str = "#.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 405);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 400);
    }
}
