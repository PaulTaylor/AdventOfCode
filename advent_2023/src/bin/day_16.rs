use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
    usize,
};

type AResult<T> = anyhow::Result<T>;
type Grid = Vec<Vec<char>>;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Beam(char, (usize, usize));

fn parse(lines: &[String]) -> Grid {
    lines.iter().map(|l| l.chars().collect()).collect()
}

fn next_beams(dir: char, tile: char, n_coords: (usize, usize)) -> Vec<Beam> {
    match (dir, tile) {
        // Empty Space (and splitters that do nothing)
        (_, '.') | ('L' | 'R', '-') | ('U' | 'D', '|') => vec![Beam(dir, n_coords)],
        // Mirrors
        ('R', '/') | ('L', '\\') => vec![Beam('U', n_coords)],
        ('R', '\\') | ('L', '/') => vec![Beam('D', n_coords)],
        ('U', '/') | ('D', '\\') => vec![Beam('R', n_coords)],
        ('U', '\\') | ('D', '/') => vec![Beam('L', n_coords)],
        // Splitters that do something
        ('L' | 'R', '|') => vec![Beam('U', n_coords), Beam('D', n_coords)],
        ('U' | 'D', '-') => vec![Beam('L', n_coords), Beam('R', n_coords)],
        // Catcher
        _ => panic!("Don't know how to handle moving {dir} into a {tile}"),
    }
}

fn count_engergised(grid: &Grid, start_dir: char, start_pos: (usize, usize)) -> usize {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut history = HashSet::new();
    let mut beam_ends = next_beams(start_dir, grid[start_pos.0][start_pos.1], start_pos);

    while !beam_ends.is_empty() {
        // Move each beam by one step and return the appropriate output beams
        beam_ends = beam_ends
            .into_iter()
            .flat_map(|beam: Beam| {
                // If we've processed this beam in this position before - bail out
                if history.contains(&beam) {
                    return vec![];
                }

                let Beam(dir, (c_row, c_col)) = beam;
                history.insert(beam);

                // Move 1 step in the current direction
                let possible_new_pos = match dir {
                    'R' if c_col < num_cols - 1 => Some((c_row, c_col + 1)),
                    'L' if c_col > 0 => Some((c_row, c_col - 1)),
                    'U' if c_row > 0 => Some((c_row - 1, c_col)),
                    'D' if c_row < num_rows - 1 => Some((c_row + 1, c_col)),
                    _ => None,
                };

                if let Some((n_row, n_col)) = possible_new_pos {
                    // Change the direction/split appropriately

                    let tile = grid[n_row][n_col];
                    let n_coords = (n_row, n_col);
                    next_beams(dir, tile, n_coords)
                } else {
                    // Hit a wall - remove the beam
                    vec![]
                }
            })
            .collect();
    }

    history.iter().map(|b| b.1).collect::<HashSet<_>>().len()
}

fn part_a(lines: &[String]) -> usize {
    let grid = parse(lines);
    count_engergised(&grid, 'R', (0, 0))
}

fn part_b(lines: &[String]) -> usize {
    let grid = parse(lines);
    let last_col = grid[0].len() - 1;
    let last_row = grid.len() - 1;

    // Corners first
    let lengths = [
        // Top Left
        count_engergised(&grid, 'R', (0, 0)),
        count_engergised(&grid, 'D', (0, 0)),
        // Top Right
        count_engergised(&grid, 'L', (0, last_col)),
        count_engergised(&grid, 'D', (0, last_col)),
        // Bottom Left
        count_engergised(&grid, 'R', (last_row, 0)),
        count_engergised(&grid, 'U', (last_row, 0)),
        // Bottom Right
        count_engergised(&grid, 'L', (last_row, last_col)),
        count_engergised(&grid, 'U', (last_row, last_col)),
    ];

    // Now do rest of the sides - starting on the top row
    lengths
        .into_iter()
        .chain((1..last_col).map(|col| count_engergised(&grid, 'D', (0, col))))
        .chain(
            // Left hand side
            (1..last_row).map(|row| count_engergised(&grid, 'R', (row, 0))),
        )
        .chain(
            // Right hand side
            (1..last_row).map(|row| count_engergised(&grid, 'L', (row, last_col))),
        )
        .chain(
            // Bottom Row
            (1..last_col).map(|col| count_engergised(&grid, 'U', (last_row, col))),
        )
        .max()
        .unwrap()
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

    const TEST_INPUT: &str = r".|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 46);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 51);
    }
}
