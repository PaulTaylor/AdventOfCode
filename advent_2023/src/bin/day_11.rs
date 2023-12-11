use humantime::format_duration;
use regex::Regex;
use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<Vec<char>> {
    lines.iter().map(|l| l.chars().collect()).collect()
}

fn expand(source: &mut [Vec<char>], factor: usize) -> BTreeSet<(usize, usize)> {
    // Find empty columns
    let mut empty_cols = vec![];
    for cidx in 0..source[0].len() {
        if source.iter().all(|row| row[cidx] == '.') {
            empty_cols.push(cidx);
        }
    }

    // Find empty rows
    let mut empty_rows = vec![];
    for (rid, row) in source.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows.push(rid);
        }
    }

    // Find the raw galaxy coordinates
    let galaxy_locs: BTreeSet<_> = source
        .iter()
        .enumerate()
        .flat_map(|(rid, row)| -> Vec<_> {
            row.iter()
                .enumerate()
                .filter_map(|(cid, &c)| if c == '#' { Some((rid, cid)) } else { None })
                .collect()
        })
        .collect();

    // Shift the raw co-ordinates to their new locations
    galaxy_locs
        .iter()
        .map(|&(row, col)| {
            let row_shift = empty_rows.iter().filter(|&&r| r < row).count();
            let col_shift = empty_cols.iter().filter(|&&c| c < col).count();

            (
                row + (row_shift * factor.saturating_sub(1)),
                col + (col_shift * factor.saturating_sub(1)),
            )
        })
        .collect()
}

fn calculate_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let &(a_row, a_col) = a;
    let &(b_row, b_col) = b;

    a_row.abs_diff(b_row) + a_col.abs_diff(b_col)
}

fn part_a(lines: &[String]) -> usize {
    let mut grid = parse(lines);
    let galaxies = expand(&mut grid, 2);

    let mut acc = 0;
    for (gid, gal) in galaxies.iter().enumerate() {
        for other in galaxies.iter().skip(gid + 1) {
            acc += calculate_distance(gal, other);
        }
    }

    acc
}

fn part_b(lines: &[String], factor: usize) -> usize {
    let mut grid = parse(lines);
    let galaxies = expand(&mut grid, factor);

    let mut acc = 0;
    for (gid, gal) in galaxies.iter().enumerate() {
        for other in galaxies.iter().skip(gid + 1) {
            acc += calculate_distance(gal, other);
        }
    }

    acc
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
    println!("Part B result = {}", part_b(lines.as_slice(), 1_000_000));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 374);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice(), 10), 1030);
        assert_eq!(part_b(lines.as_slice(), 100), 8410);
    }
}
