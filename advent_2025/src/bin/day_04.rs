use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> HashSet<(isize, isize)> {
    let mut out = HashSet::new();
    for (rid, row) in lines.iter().enumerate() {
        for (cid, c) in row.chars().enumerate() {
            if c == '@' {
                out.insert((rid as isize, cid as isize));
            }
        }
    }
    out
}

fn find_movable(grid: &HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    let mut movable = HashSet::new();
    for (rid, cid) in grid {
        let mut acc = 0;
        for r_delta in -1..2 {
            for c_delta in -1..2 {
                if !(r_delta == 0 && c_delta == 0) && grid.contains(&(rid + r_delta, cid + c_delta))
                {
                    acc += 1;
                }
            }
        }

        if acc < 4 {
            movable.insert((*rid, *cid));
        }
    }

    movable
}

fn part_a(lines: &[String]) -> usize {
    let grid = parse(lines);
    find_movable(&grid).len()
}

fn part_b(lines: &[String]) -> usize {
    let mut grid = parse(lines);
    let starting_roll_count = grid.len();
    let mut roll_count = grid.len();
    loop {
        let to_remove = find_movable(&grid);
        if to_remove.is_empty() {
            return starting_roll_count - roll_count;
        }
        grid.retain(|v| !to_remove.contains(v));
        roll_count = grid.len();
    }
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

    const TEST_INPUT: &str = "..@@.@@@@.
                              @@@.@.@.@@
                              @@@@@.@.@@
                              @.@@@@..@.
                              @@.@@@@.@@
                              .@@@@@@@.@
                              .@.@.@.@@@
                              @.@@@.@@@@
                              .@@@@@@@@.
                              @.@.@@@.@.";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 13);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 43);
    }
}
