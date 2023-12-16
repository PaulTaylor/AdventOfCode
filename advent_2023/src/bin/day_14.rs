use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<Vec<char>> {
    lines.iter().map(|l| l.chars().collect()).collect()
}

fn roll_north(grid: &mut Vec<Vec<char>>) {
    let grid_len = grid.len();
    let row_len = grid[0].len();

    let mut changes = 1;
    while changes > 0 {
        changes = 0;
        for rid in 0..grid_len - 1 {
            for cid in 0..row_len {
                if let ('.', 'O') = (grid[rid][cid], grid[rid + 1][cid]) {
                    {
                        grid[rid][cid] = 'O';
                        grid[rid + 1][cid] = '.';
                    }
                    changes += 1;
                }
            }
        }
    }
}

fn roll_south(grid: &mut Vec<Vec<char>>) {
    let grid_len = grid.len();
    let row_len = grid[0].len();

    let mut changes = 1;
    while changes > 0 {
        changes = 0;
        for rid in (0..grid_len - 1).rev() {
            for cid in 0..row_len {
                if let ('O', '.') = (grid[rid][cid], grid[rid + 1][cid]) {
                    {
                        grid[rid][cid] = '.';
                        grid[rid + 1][cid] = 'O';
                    }
                    changes += 1;
                }
            }
        }
    }
}

fn roll_west(grid: &mut Vec<Vec<char>>) {
    let row_len = grid[0].len();

    let mut changes = 1;
    while changes > 0 {
        changes = 0;

        for w_cid in 0..row_len - 1 {
            let e_cid = w_cid + 1;
            for row in &mut *grid {
                if let ('.', 'O') = (row[w_cid], row[e_cid]) {
                    {
                        row[w_cid] = 'O';
                        row[e_cid] = '.';
                    }
                    changes += 1;
                }
            }
        }
    }
}

fn roll_east(grid: &mut Vec<Vec<char>>) {
    let row_len = grid[0].len();

    let mut changes = 1;
    while changes > 0 {
        changes = 0;
        for e_cid in 1..row_len {
            let w_cid = e_cid - 1;
            for row in &mut *grid {
                if let ('O', '.') = (row[w_cid], row[e_cid]) {
                    {
                        row[w_cid] = '.';
                        row[e_cid] = 'O';
                    }
                    changes += 1;
                }
            }
        }
    }
}

fn grid_score(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(rid, row)| row.iter().filter(|&&c| c == 'O').count() * (rid + 1))
        .sum()
}

fn brent(scores: &[usize]) -> Option<(usize, usize)> {
    const WINDOW_SIZE: usize = 5;

    let mut power = 1usize;
    let mut lam = 1usize;

    let mut p_tortoise = 0;
    let mut p_hare = 1;

    // Change to check the local neighbourhood rather than just the
    // individual values (as there are duplicate numbers in the sequence)
    while scores[p_tortoise..p_tortoise + WINDOW_SIZE] != scores[p_hare..p_hare + WINDOW_SIZE] {
        if power == lam {
            p_tortoise = p_hare;
            power *= 2;
            lam = 0;
        }
        p_hare += 1;
        lam += 1;

        if p_hare + WINDOW_SIZE >= scores.len() || p_tortoise + WINDOW_SIZE >= scores.len() {
            return None; // No cycle found
        }
    }

    p_tortoise = 0;
    p_hare = lam;
    let mut mu = 0;
    while scores[p_tortoise] != scores[p_hare] {
        p_tortoise += 1;
        p_hare += 1;
        mu += 1;
    }

    Some((lam, mu)) // (loop_len, offset)
}

fn part_a(lines: &[String]) -> usize {
    let mut grid = parse(lines);
    roll_north(&mut grid);
    grid_score(&grid)
}

fn part_b(lines: &[String]) -> usize {
    const SEED_ITERS: usize = 250;
    const TARGET_ITERS: usize = 1_000_000_000;

    let mut previous_grid = parse(lines);

    // Run forward for some number of items to seed the sequence
    let mut scores = vec![];
    for _cycle in 0..SEED_ITERS {
        let mut grid = previous_grid.clone();

        roll_north(&mut grid);
        roll_west(&mut grid);
        roll_south(&mut grid);
        roll_east(&mut grid);

        // Prep for next iteration
        scores.push(grid_score(&grid));
        previous_grid = grid;
    }

    // Detect the cycle in the scores and determine what the target value would be
    if let Some((len, offset)) = brent(&scores) {
        let repeated_section: Vec<_> = scores.iter().skip(offset).take(len).collect();
        let idx = TARGET_ITERS - offset - 1;
        return *repeated_section[idx % len];
    }

    panic!("No cycle detected in scores, perhaps you need to increase SEED_ITERS")
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

    const TEST_INPUT: &str = "O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....";

    #[test]
    fn test_brent() {
        assert_eq!(
            brent(&[1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3]),
            Some((3, 0))
        );
        assert_eq!(
            brent(&[0, 1, 1, 2, 3, 1, 2, 3, 1, 1, 2, 3, 1, 2, 3, 1, 1, 2, 3, 1, 2, 3]),
            Some((7, 1))
        );
        assert_eq!(brent((0..100).collect::<Vec<_>>().as_slice()), None);
    }

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 136);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 64);
    }
}
