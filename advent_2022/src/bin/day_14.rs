use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

const SPAWN: (usize, usize) = (500, 0);

fn parse(lines: &[String]) -> (HashMap<(usize, usize), char>, usize) {
    let rocks: Vec<_> = lines
        .iter()
        .map(|l| -> Vec<(usize, usize)> {
            l.split(" -> ")
                .map(|c| -> (usize, usize) {
                    let n: Vec<_> = c.split(',').map(|v| v.parse().unwrap()).collect();
                    (n[0], n[1])
                })
                .collect()
        })
        .collect();

    let mut max_y = 0;
    let mut grid = HashMap::new();
    for segments in rocks {
        for segment in segments.windows(2) {
            let x1 = segment.iter().map(|s| s.0).min().unwrap();
            let x2 = segment.iter().map(|s| s.0).max().unwrap();
            let y1 = segment.iter().map(|s| s.1).min().unwrap();
            let y2 = segment.iter().map(|s| s.1).max().unwrap();

            for x in x1..=x2 {
                for y in y1..=y2 {
                    grid.insert((x, y), '#');
                    max_y = std::cmp::max(y, max_y);
                }
            }
        }
    }

    (grid, max_y)
}

fn _display(grid: &HashMap<(usize, usize), char>) {
    let min_x = grid.keys().map(|k| k.0).min().unwrap();
    let min_y = 0; // Always 0 because of the sand start position
    let max_x = grid.keys().map(|k| k.0).max().unwrap();
    let max_y = grid.keys().map(|k| k.1).max().unwrap();

    let max_x = std::cmp::max(max_x, 500);

    println!("=========================");
    for y in min_y..=max_y {
        print!("{:02}  ", y);
        for x in min_x..=max_x {
            match grid.get(&(x, y)) {
                Some(c) => print!("{}", c),
                None if (x, y) == SPAWN => print!("+"),
                None => print!(" "),
            }
        }
        println!()
    }
    println!("=========================");
}

fn part_a(lines: &[String]) -> AResult<usize> {
    let (mut grid, floor) = parse(lines);
    let starting_len = grid.len();

    'outer: loop {
        let (mut sx, mut sy) = SPAWN;
        loop {
            let mut moved = false;
            if grid.get(&(sx, sy + 1)).is_none() {
                // Can move directly down
                sy += 1;
                moved = true;
            } else if !moved && grid.get(&(sx - 1, sy + 1)).is_none() {
                // Down is blocked, try down-left
                sx -= 1;
                sy += 1;
                moved = true;
            } else if !moved && grid.get(&(sx + 1, sy + 1)).is_none() {
                sx += 1;
                sy += 1;
                moved = true;
            }

            if !moved {
                // if we've not moved - then this is the final position of this sand
                // and we can try an put another block of sand in
                grid.insert((sx, sy), 'o');
                break;
            }

            if sy > floor {
                // Stop the sim - some sand is in the abyss
                break 'outer;
            }
        }
    }

    Ok(grid.len() - starting_len)
}

fn part_b(lines: &[String]) -> AResult<usize> {
    let (mut grid, max_y) = parse(lines);
    let floor = max_y + 1;
    let starting_len = grid.len();

    'outer: loop {
        let (mut sx, mut sy) = SPAWN;
        loop {
            let mut moved = false;
            if grid.get(&(sx, sy + 1)).is_none() && sy < floor {
                // Can move directly down
                sy += 1;
                moved = true;
            } else if !moved && grid.get(&(sx - 1, sy + 1)).is_none() && sy < floor {
                // Down is blocked, try down-left
                sx -= 1;
                sy += 1;
                moved = true;
            } else if !moved && grid.get(&(sx + 1, sy + 1)).is_none() && sy < floor {
                sx += 1;
                sy += 1;
                moved = true;
            }

            if !moved {
                grid.insert((sx, sy), 'o');
                if SPAWN == (sx, sy) {
                    break 'outer;
                } else {
                    break;
                }
            }
        }
    }

    Ok(grid.len() - starting_len)
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
    println!("Part A result = {}", part_a(lines.as_slice())?);
    println!("Part B result = {}", part_b(lines.as_slice())?);
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 24);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 93);
        Ok(())
    }
}
