use humantime::format_duration;
use regex::Regex;
use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> AResult<[[u8; 10]; 10]> {
    let mut arr = [[0u8; 10]; 10];

    for (ir, line) in lines.iter().enumerate() {
        for (ic, char) in line.chars().enumerate() {
            arr[ir][ic] = (char as u8) - 48;
        }
    }

    Ok(arr)
}

fn calculate_bounds(row: usize, col: usize, rcount: usize, ccount: usize) -> (usize, usize, usize, usize) {
    (
        if row == 0 { 0 } else { row - 1 },
        min(rcount - 1, row + 1), 
        if col == 0 { 0 } else { col - 1 }, 
        min(ccount - 1, col + 1)
    )
}

fn do_round(arr: &mut [[u8; 10]; 10]) -> u64 {
    let mut flashers: Vec<(usize, usize)> = Vec::new();

    // Increment all by 1 - and catch the initial flashers
    for row in 0..arr.len() {
        for col in 0..arr[0].len() {
            arr[row][col] += 1;
            if arr[row][col] == 10 {
                flashers.push((row, col));
            }
        }
    }
    
    // Deal with flashers
    let mut start = 0;
    while start < flashers.len() {
        let original_len = flashers.len();
        for i in start..original_len {
            // Check the 8 adjacent spaces and incr
            let (row, col) = flashers[i];
            let (top, bottom, left, right) = 
            calculate_bounds(row, col, arr.len(), arr[0].len()); 
            
            #[allow(clippy::needless_range_loop)] // reason="a loop is more readable"
            for r in top..=bottom {
                for c in left..=right {
                    arr[r][c] += 1;
                    if arr[r][c] == 10 { // only check for 10 to avoid double counting
                        flashers.push((r,c));
                    }
                }
            }
        }
        start = original_len
    }

    // Set all flashers to be 0 and count at the same time
    let mut flashes = 0;
    for (row, col) in flashers {
        arr[row][col] = 0;
        flashes += 1;
    }

    flashes
}

fn part_a(lines: &[String]) -> AResult<u64> {
    let mut arr = parse(lines)?;
    let mut flashes = 0;

    for _round in 0..100 {
        flashes += do_round(&mut arr);
    }

    Ok(flashes)
}

fn part_b(lines: &[String]) -> AResult<u64> {
    let mut arr = parse(lines)?;
    let target = (arr.len() * arr[0].len()) as u64;
    let mut round = 0;
    let mut flashes = 0;
    while flashes < target {
        round += 1;
        flashes = do_round(&mut arr);
    }

    Ok(round)
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

    const TEST_INPUT: &str = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 1656);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 195);
        Ok(())
    }
}
