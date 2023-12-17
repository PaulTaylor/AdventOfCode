use humantime::format_duration;
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type Grid = Vec<Vec<usize>>;
type Coord = (usize, usize);
type AResult<T> = anyhow::Result<T>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    row: usize,
    col: usize,
    dir: char,
    path: Vec<Coord>,
    heat_loss: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Ordering is for the binary heap only
        // Favor lower heat_loss states
        self.heat_loss.cmp(&other.heat_loss).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(lines: &[String]) -> Grid {
    lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

//
// Functions to generate successor states in the specified direction
//

fn left(original: &State, min: usize, max: usize, grid: &Grid) -> Vec<State> {
    let mut out = Vec::with_capacity(max - min);
    for candidate_offset in min..=max {
        if let Some(n_col) = original.col.checked_sub(candidate_offset) {
            if !original.path.contains(&(original.row, n_col)) {
                let mut np = original.path.clone();
                np.extend((n_col..original.col).rev().map(|nc| (original.row, nc)));

                let np_loss: usize = (n_col..original.col).map(|i| grid[original.row][i]).sum();

                out.push(State {
                    col: n_col,
                    dir: 'L',
                    path: np,
                    heat_loss: original.heat_loss + np_loss,
                    ..*original
                });
            }
        }
    }
    out
}

fn right(original: &State, min: usize, max: usize, grid: &Grid) -> Vec<State> {
    let mut out = Vec::with_capacity(max - min);
    for candidate_offset in min..=max {
        if (original.col + candidate_offset) < grid[0].len() {
            let n_col = original.col + candidate_offset;

            if !original.path.contains(&(original.row, n_col)) {
                let mut np = original.path.clone();
                np.extend((original.col..n_col).map(|nc| (original.row, nc + 1)));

                let np_loss: usize = (original.col..n_col)
                    .map(|i| grid[original.row][i + 1])
                    .sum();

                out.push(State {
                    col: n_col,
                    dir: 'R',
                    path: np,
                    heat_loss: original.heat_loss + np_loss,
                    ..*original
                });
            }
        }
    }
    out
}

fn up(original: &State, min: usize, max: usize, grid: &Grid) -> Vec<State> {
    let mut out = Vec::with_capacity(max - min);
    for candidate_offset in min..=max {
        if let Some(n_row) = original.row.checked_sub(candidate_offset) {
            if !original.path.contains(&(n_row, original.col)) {
                let mut np = original.path.clone();
                np.extend((n_row..original.row).rev().map(|nr| (nr, original.col)));

                let np_loss: usize = (n_row..original.row).map(|i| grid[i][original.col]).sum();

                out.push(State {
                    row: n_row,
                    dir: 'U',
                    path: np,
                    heat_loss: original.heat_loss + np_loss,
                    ..*original
                });
            }
        }
    }
    out
}

fn down(original: &State, min: usize, max: usize, grid: &Grid) -> Vec<State> {
    let mut out = Vec::with_capacity(max - min);
    for candidate_offset in min..=max {
        if (original.row + candidate_offset) < grid.len() {
            let n_row = original.row + candidate_offset;
            if !original.path.contains(&(n_row, original.col)) {
                let mut np = original.path.clone();
                np.extend((original.row..n_row).map(|nr| (nr + 1, original.col)));

                let np_loss: usize = (original.row..n_row)
                    .map(|i| grid[i + 1][original.col])
                    .sum();

                out.push(State {
                    row: n_row,
                    dir: 'D',
                    path: np,
                    heat_loss: original.heat_loss + np_loss,
                    ..*original
                });
            }
        }
    }
    out
}

//
// End direction functions
//

fn solve(lines: &[String], min_move: usize, max_move: usize) -> usize {
    let grid = parse(lines);
    let mut best = usize::MAX;
    let l_row = grid.len() - 1;
    let l_col = grid[0].len() - 1;

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        row: 0,
        col: 0,
        dir: 'U',
        path: vec![],
        heat_loss: 0,
    });

    let mut best_losses = BTreeMap::new();

    while !heap.is_empty() {
        let original = heap.pop().unwrap();
        if (original.row == l_row) && (original.col == l_col) {
            // Reached the end
            if best > original.heat_loss {
                best = original.heat_loss;
            }
            continue;
        }

        // If we've been at this position before with a better heat loss score - bail out
        let best_entry = best_losses.get(&(original.row, original.col, original.dir));
        if let Some(&best_to_here) = best_entry {
            if best_to_here <= original.heat_loss {
                continue;
            }
            best_losses.insert(
                (original.row, original.col, original.dir),
                original.heat_loss,
            );
        } else {
            best_losses.insert(
                (original.row, original.col, original.dir),
                original.heat_loss,
            );
        }

        match original.dir {
            'U' => {
                heap.extend(left(&original, min_move, max_move, &grid));
                heap.extend(right(&original, min_move, max_move, &grid));
                heap.extend(down(&original, min_move, max_move, &grid));
            }
            'R' => {
                heap.extend(left(&original, min_move, max_move, &grid));
                heap.extend(up(&original, min_move, max_move, &grid));
                heap.extend(down(&original, min_move, max_move, &grid));
            }
            'D' => {
                heap.extend(left(&original, min_move, max_move, &grid));
                heap.extend(right(&original, min_move, max_move, &grid));
                heap.extend(up(&original, min_move, max_move, &grid));
            }
            'L' => {
                heap.extend(up(&original, min_move, max_move, &grid));
                heap.extend(right(&original, min_move, max_move, &grid));
                heap.extend(down(&original, min_move, max_move, &grid));
            }
            _ => unreachable!(),
        }

        // Only keep those states that can be better than the current best
        heap.retain(|i| (i.heat_loss + ((l_row - i.row) + (l_col - i.col))) <= best);
    }

    best
}

fn part_a(lines: &[String]) -> usize {
    solve(lines, 1, 3)
}

fn part_b(lines: &[String]) -> usize {
    solve(lines, 4, 10)
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

    const TEST_INPUT: &str = "2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533";

    #[test]
    fn test_dir_left() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let grid = parse(&lines);

        let orig = State {
            col: 3,
            row: 0,
            dir: 'X',
            path: vec![],
            heat_loss: 0,
        };
        let t1 = State {
            col: 2,
            dir: 'L',
            path: vec![(0, 2)],
            heat_loss: 1,
            ..orig.clone()
        };
        assert_eq!(left(&orig, 1, 1, &grid), &[t1.clone()]);

        let t2 = State {
            col: 1,
            dir: 'L',
            path: vec![(0, 2), (0, 1)],
            heat_loss: 5,
            ..orig.clone()
        };
        assert_eq!(left(&orig, 1, 2, &grid), &[t1.clone(), t2.clone()]);

        let t3 = State {
            row: 0,
            col: 0,
            dir: 'L',
            path: vec![(0, 2), (0, 1), (0, 0)],
            heat_loss: 7,
        };
        assert_eq!(
            left(&orig, 1, 10, &grid),
            &[t1.clone(), t2.clone(), t3.clone()]
        );

        assert_eq!(left(&orig, 2, 10, &grid), &[t2.clone(), t3.clone()]);

        assert_eq!(left(&orig, 5, 5, &grid), &[]);
    }

    #[test]
    fn test_dir_right() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let grid = parse(&lines);

        let orig = State {
            col: 10,
            row: 0,
            dir: 'X',
            path: vec![],
            heat_loss: 0,
        };
        let t1 = State {
            col: 11,
            dir: 'R',
            path: vec![(0, 11)],
            heat_loss: 2,
            ..orig.clone()
        };
        assert_eq!(right(&orig, 1, 1, &grid), &[t1.clone()]);

        let t2 = State {
            col: 12,
            dir: 'R',
            path: vec![(0, 11), (0, 12)],
            heat_loss: 5,
            ..orig.clone()
        };
        assert_eq!(
            right(&orig, 1, 2, &grid),
            &[t1.clone(), t2.clone()],
            "simple two test"
        );

        assert_eq!(
            right(&orig, 1, 10, &grid),
            &[t1.clone(), t2.clone()],
            "overflow max test"
        );

        assert_eq!(right(&orig, 2, 2, &grid), &[t2.clone()], "min length test");

        assert_eq!(right(&orig, 5, 5, &grid), &[], "too many test");
    }

    #[test]
    fn test_dir_up() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let grid = parse(&lines);

        let orig = State {
            col: 0,
            row: 2,
            dir: 'X',
            path: vec![],
            heat_loss: 0,
        };
        let t1 = State {
            row: 1,
            dir: 'U',
            path: vec![(1, 0)],
            heat_loss: 3,
            ..orig.clone()
        };
        assert_eq!(up(&orig, 1, 1, &grid), &[t1.clone()]);

        let t2 = State {
            row: 0,
            dir: 'U',
            path: vec![(1, 0), (0, 0)],
            heat_loss: 5,
            ..orig.clone()
        };
        assert_eq!(up(&orig, 1, 2, &grid), &[t1.clone(), t2.clone()]);

        assert_eq!(up(&orig, 1, 10, &grid), &[t1.clone(), t2.clone()]);

        assert_eq!(up(&orig, 2, 10, &grid), &[t2.clone()]);

        assert_eq!(up(&orig, 5, 5, &grid), &[]);
    }

    #[test]
    fn test_dir_down() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let grid = parse(&lines);

        let orig = State {
            col: 0,
            row: 10,
            dir: 'X',
            path: vec![],
            heat_loss: 0,
        };
        let t1 = State {
            row: 11,
            dir: 'D',
            path: vec![(11, 0)],
            heat_loss: 2,
            ..orig.clone()
        };
        assert_eq!(down(&orig, 1, 1, &grid), &[t1.clone()]);

        let t2 = State {
            row: 12,
            dir: 'D',
            path: vec![(11, 0), (12, 0)],
            heat_loss: 6,
            ..orig.clone()
        };
        assert_eq!(down(&orig, 1, 3, &grid), &[t1.clone(), t2.clone()]);

        assert_eq!(down(&orig, 1, 10, &grid), &[t1.clone(), t2.clone()]);

        assert_eq!(down(&orig, 2, 10, &grid), &[t2.clone()]);

        assert_eq!(down(&orig, 5, 5, &grid), &[]);
    }

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 102);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 94);
    }
}
