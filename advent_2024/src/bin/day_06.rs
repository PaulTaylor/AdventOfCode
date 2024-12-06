use humantime::format_duration;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Location = (isize, isize);
type Guard = (Location, char);
type Grid = HashMap<Location, char>;

fn parse(lines: &[String]) -> (Guard, Grid) {
    let mut grid: HashMap<_, _> = lines
        .iter()
        .enumerate()
        .flat_map(|(r, l)| {
            l.char_indices()
                .map(|(c, v)| ((r.try_into().unwrap(), c.try_into().unwrap()), v))
                .collect::<Vec<_>>()
        })
        .collect();

    // Find the guard
    let guard_loc = grid
        .iter()
        .find_map(|(&k, &v)| if v == '^' { Some(k) } else { None })
        .expect("Expected an initial location for the guard");

    grid.insert(guard_loc, '.');

    (((guard_loc.0, guard_loc.1), '^'), grid)
}

/// Process one guard step returning a new guard location if appropriate
/// and a loop flag indicating if we stopped because of a loop or because
/// the Guard exited the board
///
/// `extra_block` gives the location of the new block added to the board in Part 2
fn step(
    (guard_loc, dir): Guard,
    grid: &Grid,
    extra_block: Option<Location>,
    history: &mut HashSet<Guard>,
) -> (Option<Guard>, bool) {
    let (row, col) = guard_loc;
    let candidate = match dir {
        '^' => (row - 1, col),
        '>' => (row, col + 1),
        'v' => (row + 1, col),
        '<' => (row, col - 1),
        _ => unreachable!(),
    };

    let effective_value = match extra_block {
        Some(eb_loc) if candidate == eb_loc => Some(&'#'),
        _ => grid.get(&candidate),
    };

    match effective_value {
        Some(&'#') => {
            // The new location is blocked - rotate and try again
            let new_dir = match dir {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => unreachable!(),
            };
            step((guard_loc, new_dir), grid, extra_block, history)
        }
        Some(_) => {
            if !history.insert((candidate, dir)) {
                return (None, true);
            }
            (Some((candidate, dir)), false)
        }
        _ => (None, false),
    }
}

/// Simulate the guards walking pattern with the given configuration
///
/// `guard` gives the guards initial location
/// `grid` gives the layout of the room
/// `extra_block` gives the location of the new block added to the board in Part 2
fn walk(mut guard: Guard, grid: &Grid, extra_block: Option<Location>) -> (HashSet<Guard>, bool) {
    let mut history = HashSet::new(); // Unused for part_a
    history.insert(guard);

    let is_loop;
    loop {
        let (new_guard, new_loop) = step(guard, grid, extra_block, &mut history);
        if let Some(ng) = new_guard {
            guard = ng;
        } else {
            is_loop = new_loop;
            break;
        }
    }
    (history, is_loop)
}

fn part_a(lines: &[String]) -> usize {
    let (guard_loc, grid) = parse(lines);
    let (history, _) = walk(guard_loc, &grid, None);

    history
        .into_iter()
        .map(|l| l.0)
        .collect::<HashSet<_>>()
        .len()
}

fn part_b(lines: &[String]) -> usize {
    let (guard, grid) = parse(lines);

    let original_walk = walk(guard, &grid, None)
        .0
        .into_iter()
        .map(|l| l.0)
        .collect::<HashSet<_>>();

    // Lets parallelise this with Rayon because why not?
    let ow_len = original_walk.len();
    original_walk
        .into_par_iter()
        .progress_count(ow_len as u64)
        .filter(|o| walk(guard, &grid, Some(*o)).1)
        .count()
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

    const TEST_INPUT: &str = "....#.....
                              .........#
                              ..........
                              ..#.......
                              .......#..
                              ..........
                              .#..^.....
                              ........#.
                              #.........
                              ......#...";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 41);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 6);
    }
}
