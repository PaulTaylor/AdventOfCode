use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant, collections::HashSet,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> AResult<Vec<Vec<u8>>> {
    Ok(lines.iter().map(
        |line| line.chars().map(|c| (c as u8) - 48).collect()
    ).collect())
}

fn part_a(lines: &[String]) -> AResult<(u32, Vec<(usize,usize)>)> {
    let grid = parse(lines)?;
    let mut low_points: Vec<_> = Vec::new();
    let mut acc = 0u32;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {

            let this = grid[row][col];
            let mut adj: Vec<_> = Vec::with_capacity(4);

            if row > 0 {
                adj.push(grid[row-1][col]);
            }
            if row + 1 < grid.len() {
                adj.push(grid[row+1][col]);
            }
            if col > 0 {
                adj.push(grid[row][col-1]);
            }
            if col + 1 < grid[row].len() {
                adj.push(grid[row][col+1]);
            }

            if this < *adj.iter().min().unwrap() {
                acc += (1 + this) as u32;
                low_points.push((row, col));
            }
        }
    }

    Ok((acc, low_points))
}

fn part_b(lines: &[String], low_points: Vec<(usize,usize)>) -> AResult<u32> {
    // nb. low_points are (row, col)
    let grid = parse(lines)?;

    let mut basins = Vec::new();
    for lp in low_points {
        let mut basin: HashSet<(usize, usize)> = HashSet::new();
        basin.insert(lp);
        // A visited set would make this more efficient...

        let mut start_size = 0usize;
        while basin.len() > start_size {
            start_size = basin.len();
            let mut new_points = HashSet::new();
            
            for (row, col) in basin.iter() {
                if row > &0 && grid[row-1][*col] < 9 {
                    new_points.insert((row-1, *col));
                }
                if row + 1 < grid.len() && grid[row+1][*col] < 9 {
                    new_points.insert((row+1, *col));
                }
                if col > &0 && grid[*row][col-1] < 9 {
                    new_points.insert((*row, col-1));
                }
                if col + 1 < grid[*row].len() && grid[*row][col+1] < 9 {
                    new_points.insert((*row, col+1));
                }
            }
            
            basin.extend(new_points);
        }
        basins.push(basin);
    }

    basins.sort_unstable_by_key(|s| s.len());
    let result: u32 = basins.iter().rev().take(3).map(|s| s.len() as u32).product();
    Ok(result)
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
    let file = File::open(format!("./data/day_{ex}_a.txt"))?;
    let lines: Vec<String> = BufReader::new(file).lines().map(Result::unwrap).collect();

    // Run the solutions
    let start = Instant::now();
    let (a_res, lp) = part_a(lines.as_slice())?;
    println!("Part A result = {}", a_res);
    println!("Part B result = {}", part_b(lines.as_slice(), lp)?);
    let end = Instant::now();

    println!("Run took {}ms", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2199943210
    3987894921
    9856789892
    8767896789
    9899965678";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let (res, _) = part_a(lines.as_slice())?;
        assert_eq!(res, 15);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let (_, low_points) = part_a(&lines)?;
        assert_eq!(part_b(&lines, low_points)?, 1134);
        Ok(())
    }
}
