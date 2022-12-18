use humantime::format_duration;
use regex::Regex;
use std::{
    cmp::max,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};
use Shape::*;

type AResult<T> = anyhow::Result<T>;

#[non_exhaustive]
#[derive(Debug)]
enum Shape {
    HLine,
    Cross,
    RevL,
    VLine,
    Square,
}

impl Shape {
    // Create a block function for each shape - which will return the
    // coordinates for each of its blocks in real space

    fn blocks(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        match self {
            HLine => (0..4).map(|i| (x + i, y)).collect(),
            Cross => vec![
                (x + 1, y),     // bottom mid
                (x, y + 1),     // mid left
                (x + 1, y + 1), // mid mid
                (x + 2, y + 1), // mid right
                (x + 1, y + 2), // top mid
            ],
            RevL => vec![
                (x, y),         // bottom left
                (x + 1, y),     // bottom mid
                (x + 2, y),     // bottom right
                (x + 2, y + 1), // stick middle
                (x + 2, y + 2), // stick top
            ],
            VLine => (0..4).map(|i| (x, y + i)).collect(),
            Square => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        }
    }
}

#[derive(Debug)]
enum Action {
    Push,
    Fall,
}

fn detect_cycle(heights: &str) -> Option<(usize, String)> {
    // Detect cycles (by abusing Regex :))
    // returns the offset from the start and the cycle string

    // Pattern looks for the start of the candidate being repeated continually until
    // the end of the string - it must repeat twice or more so we can be sure we're
    // actually cycling
    let pattern = fancy_regex::Regex::new(r##"^(.{1,}?)\1{2,}(.*)$"##).unwrap();

    for start in 0..heights.chars().count() / 2 {
        if let Ok(Some(caps)) = pattern.captures(&heights[start..]) {
            let cycle = caps.get(1)?.as_str();
            // Is the part of the string after the repeated cycles < the cycle length
            // and does it match the beginning of the cycle
            let remainder = caps.get(2)?.as_str();
            if remainder.len() < cycle.len() && (remainder == &cycle[0..remainder.len()]) {
                // found!
                return Some((start, cycle.to_string()));
            }
        }
    }

    None // Didn't find a cycle
}

fn solve(lines: &[String], target: usize) -> AResult<usize> {
    let period = 5 * lines[0].chars().count();
    let mut board: HashMap<(usize, usize), char> = HashMap::new();
    let mut feed = [HLine, Cross, RevL, VLine, Square].iter().cycle();
    let mut moves = lines[0].chars().cycle();
    let mut loops = 0;
    let mut heights = String::new();
    let mut highest_y = 0;
    let mut state: Option<(&Shape, usize, usize, &Action)> = None;
    let cycle;

    loop {
        if loops > period * 20 {
            cycle = detect_cycle(&heights);
            break;
        }
        loops += 1;

        match state {
            None => {
                // Need to spawn a new rock
                let new_x = 2usize;
                let new_y: usize = highest_y + 4;
                state = Some((feed.next().unwrap(), new_x, new_y, &Action::Push));
            }
            Some((s, x, y, a)) => match a {
                Action::Fall => {
                    let new_blocks = s.blocks(x, y - 1);

                    let valid = new_blocks
                        .iter()
                        .all(|(x, y)| (*y > 0) && !board.contains_key(&(*x, *y)));

                    if valid {
                        // Block does fall by one block
                        state = Some((s, x, y - 1, &Action::Push));
                    } else {
                        // Block can't fall and so comes to rest at it's current position
                        let orig = highest_y;
                        for (x, y) in s.blocks(x, y) {
                            board.insert((x, y), '#');
                            highest_y = max(highest_y, y);
                        }
                        heights.push_str(&format!("{}", highest_y - orig));
                        state = None
                    }
                }
                Action::Push => {
                    let dir = moves.next().unwrap();
                    let new_x = match dir {
                        '<' if x > 0 => x - 1,
                        '>' => x + 1,
                        _ => {
                            // would move past left hand edge
                            state = Some((s, x, y, &Action::Fall));
                            continue;
                        }
                    };

                    let new_blocks = s.blocks(new_x, y);

                    // All blocks must have x < 7 and must not collide with resting blocks
                    let valid = new_blocks
                        .iter()
                        .all(|(x, y)| (*x < 7) && !board.contains_key(&(*x, *y)));

                    if valid {
                        state = Some((s, new_x, y, &Action::Fall));
                    } else {
                        state = Some((s, x, y, &Action::Fall));
                    }
                }
            },
        }
    }

    // Now we have the cycle - determine the height of the stack after the required number of rocks

    if let Some((offset, cycle)) = cycle {
        let mut result: usize = heights[..offset]
            .chars()
            .map(|c| (c as u8 - 48) as usize)
            .sum();
        let mut remaining = target - offset;

        let cycle_sum: usize = cycle.chars().map(|c| (c as u8 - 48) as usize).sum();
        let repetitions = remaining / cycle.len();
        result += repetitions * cycle_sum;
        remaining -= repetitions * cycle.len();

        let rem_sum: usize = cycle
            .chars()
            .take(remaining)
            .map(|c| (c as u8 - 48) as usize)
            .sum();

        Ok(result + rem_sum)
    } else {
        panic!("No solution found :-(");
    }
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
    println!("Part A result = {}", solve(lines.as_slice(), 2022)?);
    println!(
        "Part B result = {}",
        solve(lines.as_slice(), 1000000000000)?
    );
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_detect_cycle() -> AResult<()> {
        // Some example data generated from sim runs of the test scenario
        let heights = [
            1, 3, 2, 1, 2, 1, 3, 2, 2, 0, 1, 3, 2, 0, 2, 1, 3, 3, 4, 0, 1, 2, 3, 0, 1, 1, 3, 2, 2,
            0, 0, 2, 3, 4, 0, 1, 2, 1, 2, 0, 1, 2, 1, 2, 0, 1, 3, 2, 0, 0, 1, 3, 3, 4, 0, 1, 2, 3,
            0, 1, 1, 3, 2, 2, 0, 0, 2, 3, 4, 0, 1, 2, 1, 2, 0, 1, 2, 1, 2, 0, 1, 3, 2, 0, 0, 1, 3,
            3, 4, 0, 1, 2, 3, 0, 1, 1, 3, 2, 2, 0, 0, 2, 3, 4, 0, 1, 2, 1, 2, 0, 1, 2, 1, 2, 0, 1,
            3, 2, 0, 0, 1, 3, 3, 4, 0, 1, 2, 3, 0, 1, 1, 3, 2, 2, 0, 0, 2, 3, 4, 0, 1, 2, 1, 2, 0,
            1, 2, 1, 2, 0, 1, 3, 2, 0, 0, 1, 3, 3, 4, 0, 1, 2, 3, 0, 1, 1, 3, 2, 2, 0, 0, 2, 3, 4,
            0, 1, 2, 1, 2, 0, 1, 2, 1, 2, 0, 1, 3, 2, 0, 0, 1, 3, 3, 4, 0, 1, 2, 3, 0, 1, 1, 3, 2,
            2, 0, 0, 2, 3, 4, 0, 1, 2, 1, 2, 0, 1, 2, 1, 2, 0, 1, 3, 2, 0, 0, 1, 3, 3, 4, 0, 1, 2,
            3, 0, 1, 1, 3, 2, 2, 0, 0, 2, 3, 4, 0, 1, 2, 1, 2, 0, 1, 2, 1, 2, 0, 1, 3, 2, 0, 0, 1,
            3, 3, 4, 0, 1, 2, 3, 0, 1, 1, 3, 2, 2, 0, 0, 2, 3, 4, 0, 1, 2, 1, 2, 0, 1, 2, 1, 2, 0,
            1, 3, 2, 0, 0, 1, 3, 3, 4, 0, 1, 2, 3, 0, 1, 1, 3, 2, 2, 0, 0, 2, 3, 4, 0, 1, 2, 1, 2,
            0, 1, 2, 1,
        ];

        let heights: String = heights.iter().map(|x| format!("{}", x)).collect();
        if let Some((offset, cycle)) = detect_cycle(&heights) {
            assert_eq!(offset, 15);
            assert_eq!(cycle, "13340123011322002340121201212013200".to_string());
        } else {
            panic!("failed to compute cycles")
        }
        Ok(())
    }

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(solve(lines.as_slice(), 2022)?, 3068);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(solve(lines.as_slice(), 1000000000000)?, 1514285714288);
        Ok(())
    }
}
