use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

// Each tile in the valley is a bitpacked u8
type Valley = Vec<Vec<u8>>;

// Wind Direction Flags
const NORTH: u8 = 8;
const SOUTH: u8 = 4;
const EAST: u8 = 2;
const WEST: u8 = 1;
// Other tile flags
const EMPTY: u8 = 0;
const WALL: u8 = 128;

#[derive(Debug, Hash, PartialEq, Eq)]
struct State {
    row: usize,
    col: usize,
    depth: usize,
}

fn parse(lines: &[String]) -> Valley {
    let mut out = Vec::with_capacity(lines[0].len());
    for line in lines {
        let row = line
            .chars()
            .map(|c| -> u8 {
                match c {
                    '^' => NORTH,
                    'v' => SOUTH,
                    '>' => EAST,
                    '<' => WEST,
                    '.' => EMPTY,
                    '#' => WALL,
                    _ => panic!("unsupported character {c}"),
                }
            })
            .collect();
        out.push(row);
    }
    out
}

fn next_step(previous: &Valley) -> Valley {
    // Mutate the valley into it's next incarnation

    let row_len = previous[0].len();
    let mut next = Vec::with_capacity(previous.len());
    next.push(previous[0].clone());

    for ri in 1..previous.len() - 1 {
        let mut row: Vec<u8> = std::iter::repeat(0).take(row_len - 1).collect();
        row[0] = WALL;
        for (ci, cv) in row[1..].iter_mut().enumerate() {
            let ci = ci + 1;
            *cv |= previous[ri - 1][ci] & SOUTH; // south wind coming down
            *cv |= previous[ri + 1][ci] & NORTH; // north wind coming up
            *cv |= previous[ri][ci + 1] & WEST; // west wind blowing left
            *cv |= previous[ri][ci - 1] & EAST; // east wind blowing right
        }

        row.push(WALL);
        next.push(row);
    }

    // Deal with wrapping
    // for the bottom row, wrap south winds around to the top row
    for (ci, v) in previous[previous.len() - 2].iter().enumerate() {
        next[1][ci] |= v & SOUTH;
    }
    // for the top row, wrap north winds around to the bottom
    for (ci, v) in previous[1].iter().enumerate() {
        next[previous.len() - 2][ci] |= v & NORTH;
    }
    // left/right wrapping
    for (p_row, n_row) in zip(previous, &mut next) {
        n_row[row_len - 2] |= p_row[1] & WEST;
        n_row[1] |= p_row[row_len - 2] & EAST;
    }

    // Push the end row and return
    next.push(previous[previous.len() - 1].clone());
    next
}

fn _display(valley: &Valley) {
    for row in valley {
        for v in row {
            print!(
                "{}",
                match *v {
                    WALL => '#',
                    EMPTY => '.',
                    NORTH => '^',
                    SOUTH => 'v',
                    EAST => '>',
                    WEST => '<',
                    x if x.count_ones() > 1 => (x.count_ones() + 48).try_into().unwrap(),
                    _ => panic!(),
                }
            );
        }
        println!();
    }
}

fn solve(initial: Valley, the_start: (usize, usize), the_end: (usize, usize)) -> (usize, Valley) {
    let max_row = std::cmp::max(the_start.0, the_end.0);

    let mut queue: Vec<_> = Vec::new();
    queue.push(State {
        row: the_start.0,
        col: the_start.1,
        depth: 0,
    });
    let mut p_valley = initial;

    // loop
    for _depth in 1..=1000 {
        // Start a collection for the next level down
        let mut next = HashSet::new();

        // Create the next valley
        let n_valley = next_step(&p_valley);

        // For each state at this level - push the possible next states into the next set
        while let Some(prev) = queue.pop() {
            // Can we stand still?
            if n_valley[prev.row][prev.col] == EMPTY {
                next.insert(State {
                    depth: prev.depth + 1,
                    ..prev
                });
            }
            // Can we move up?
            if prev.row > 0 && n_valley[prev.row - 1][prev.col] == EMPTY {
                next.insert(State {
                    row: prev.row - 1,
                    depth: prev.depth + 1,
                    ..prev
                });
            }
            // Can we move down?
            if prev.row < max_row && n_valley[prev.row + 1][prev.col] == EMPTY {
                next.insert(State {
                    row: prev.row + 1,
                    depth: prev.depth + 1,
                    ..prev
                });
            }
            // Can we move left?
            if n_valley[prev.row][prev.col - 1] == EMPTY {
                next.insert(State {
                    col: prev.col - 1,
                    depth: prev.depth + 1,
                    ..prev
                });
            }
            // Can we move right?
            if n_valley[prev.row][prev.col + 1] == EMPTY {
                next.insert(State {
                    col: prev.col + 1,
                    depth: prev.depth + 1,
                    ..prev
                });
            }
        }

        // Check if we're done
        if let Some(s) = next.iter().find(|s| (s.row, s.col) == the_end) {
            return (s.depth, n_valley);
        }

        // Get ready for the next loop
        queue.extend(next);
        p_valley = n_valley;
    }

    panic!("Route not found within 1000 iterations");
}

fn part_a(lines: &[String]) -> usize {
    let initial = parse(lines);
    let the_start = (0, 1);
    let the_end: (usize, usize) = (initial.len() - 1, initial[0].len() - 2);
    solve(initial, the_start, the_end).0
}

fn part_b(lines: &[String]) -> usize {
    let initial = parse(lines);
    let the_start = (0, 1);
    let the_end: (usize, usize) = (initial.len() - 1, initial[0].len() - 2);

    let (l1, initial) = solve(initial, the_start, the_end); // Fwd
    let (l2, initial) = solve(initial, the_end, the_start); // Bkd
    l1 + l2 + solve(initial, the_start, the_end).0 // Fwd
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

    const TEST_INPUT: &str = "#.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#";

    #[test]
    fn test_next() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let v0 = &parse(lines.as_slice());
        let v1 = &next_step(v0);
        let v2 = &next_step(v1);

        _display(v0);
        _display(v1);

        for v in [v0, v1, v2] {
            assert_eq!(v[0][0], WALL);
            assert_eq!(v[0][1], EMPTY);
            assert_eq!(v[0][2], WALL);

            assert_eq!(v.len(), v0.len());
            assert!(v.iter().all(|row| row.len() == v0[0].len()));
        }

        assert_eq!(
            v1[1],
            [
                WALL,
                EMPTY,
                EAST,
                EAST | WEST | SOUTH,
                EMPTY,
                WEST,
                EMPTY,
                WALL
            ]
        );

        assert_eq!(
            v2[3],
            [WALL, EMPTY, EAST, EAST | WEST, EMPTY, NORTH, EAST, WALL]
        );
    }

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 18);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 54);
    }
}
