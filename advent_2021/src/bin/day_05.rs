use humantime::format_duration;
use regex::Regex;
use std::{
    cmp::{max},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    vec::Vec
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> AResult<Vec<(i32, i32, i32, i32)>> {
    let re = Regex::new("([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)")?;

    let mut out = vec![];
    for line in lines {
        let caps = re.captures(line).unwrap();
        let mut it = caps.iter().map(Option::unwrap);
        it.next(); // Skip capture 0 as it's the whole string

        let coords: Vec<_> = it
            .map(|m| -> i32 { m.as_str().parse().unwrap() })
            .collect();

        out.push((coords[0], coords[1], coords[2], coords[3]))
    }
    Ok(out)
}

fn solve(lines: &[String], no_diags: bool) -> AResult<u32> {
    let coords = parse(lines)?;

    let max_x = coords.iter().fold(0, |m, t| max(m, max(t.0, t.2))) as usize;
    let max_y = coords.iter().fold(0, |m, t| max(m, max(t.1, t.3))) as usize;

    let mut grid: Vec<Vec<u16>> = Vec::with_capacity(max_y as usize);
    grid.resize_with(max_y + 1, Vec::new);
    grid.iter_mut().for_each(|l| l.resize(max_x + 1, 0));

    for (x1, y1, x2, y2) in coords {
        if no_diags && (x1 != x2) && (y1 != y2) {
            continue;
        }
        
        let x_delta = (x2 - x1).signum();
        let y_delta = (y2 - y1).signum();
        let steps = max((x1 - x2).abs(), (y1 - y2).abs());

        let mut row = y1;
        let mut col = x1;
        for _ in 0..=steps {
            grid[row as usize][col as usize] += 1;
            row += y_delta;
            col += x_delta;
        }
    }

    // Count overlaps
    Ok(grid
        .into_iter()
        .map(|row| -> u32 { row.into_iter().filter(|x| x > &1).count() as u32 })
        .sum())
}

fn _draw(grid: &Vec<Vec<u16>>) {
    println!("==============================");
    for row in grid {
        println!("{:?}", row)
    }
    println!("==============================");
}

fn part_a(lines: &[String]) -> AResult<u32> {
    solve(lines, true)
}

fn part_b(lines: &[String]) -> AResult<u32> {
    solve(lines, false)
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
    println!("Part A result = {}", part_a(lines.as_slice())?);
    println!("Part B result = {}", part_b(lines.as_slice())?);
    let end = Instant::now();

    println!("Run took {}ms", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 5);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 12);
        Ok(())
    }
}
