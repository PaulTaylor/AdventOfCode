use humantime::format_duration;
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::{HashMap, BTreeSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

#[derive(Debug, Clone, Copy)]
struct Cell(usize, usize, u64);

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.2.cmp(&other.2) {
            Ordering::Equal => { (self.0, self.1).cmp(&(other.0, other.1)) },
            x => x
        }
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Cell {}

fn parse(lines: &[String]) -> AResult<Vec<Vec<u64>>> {
    Ok(lines
        .iter()
        .map(|l| l.chars().map(|c| (c as u64) - 48u64).collect())
        .collect())
}

fn expand(grid: &mut Vec<Vec<u64>>) {
    for row in grid.iter_mut() {
        let orig_row = row.clone();
        for c_delta in 1..5 {
            let new_v: Vec<_> = orig_row
                .iter()
                .map(|v| {
                    let mut n = v + c_delta as u64;
                    while n > 9 {
                        n -= 9
                    }
                    n
                })
                .collect();
            row.extend(new_v);
        }
    }

    let orig_rows = grid.len();
    for r_delta in 1..5 {
        for row in 0..orig_rows {
            let seed_row = &grid[row];
            let new_row: Vec<_> = seed_row
                .iter()
                .map(|v| {
                    let mut n = v + r_delta as u64;
                    while n > 9 {
                        n -= 9
                    }
                    n
                })
                .collect();
            grid.push(new_row);
        }
    }
}

fn solve(map: &[Vec<u64>]) -> AResult<u64> {
    let dist: &mut HashMap<(usize, usize), u64> =
        &mut HashMap::with_capacity(map.len() * map.len());
    dist.insert((0, 0), 0);

    let mut q = BTreeSet::new();
    for (ri, row) in map.iter().enumerate() {
        for (ci, _) in row.iter().enumerate() {
            if (ri, ci) != (0, 0) {
                dist.insert((ri, ci), u64::MAX);
                q.insert(Cell(ri, ci, u64::MAX));
            } else {
                q.insert(Cell(0, 0, 0));
            }
        }
    }

    while let Some(&u) = q.iter().next() {
        // seperate remove as pop_first(..) is a nightly feature
        q.remove(&u);
        
        let Cell(row, col, distance) = u;

        for (d_row, d_col) in [ (-1isize, 0isize), (1, 0), (0, -1), (0, 1) ] {
            let n_row: usize =  (row as isize + d_row) as usize;
            let n_col: usize = (col as isize + d_col) as usize;
            if let Some(next_cost) = map.get(n_row).and_then(|r| r.get(n_col)) {
                let alt = if distance < u64::MAX { distance + next_cost } else { u64::MAX };
                if alt < dist[&(n_row, n_col)] {
                    dist.insert((n_row, n_col), alt);
                    q.replace(Cell(n_row, n_col, alt));
                }
            }
        }
    }

    Ok(*dist.get(&(map.len()-1, map[0].len()-1)).unwrap())
}

fn part_a(lines: &[String]) -> AResult<u64> {
    let map = parse(lines)?;
    solve(&map)
}

fn part_b(lines: &[String]) -> AResult<u64> {
    let mut map = parse(lines)?;
    expand(&mut map);
    solve(&map)
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

    const TEST_INPUT: &str = "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 40);
        Ok(())
    }

    #[test]
    fn test_expand() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let mut map = parse(lines.as_slice())?;
        expand(&mut map);

        assert_eq!(map[0].len(), 50);
        let s: String = map[0].iter().map(|n| format!("{}", n)).collect();
        assert_eq!(s, "11637517422274862853338597396444961841755517295286");

        assert_eq!(map.len(), 50);
        let s: String = map[49].iter().map(|n| format!("{}", n)).collect();
        assert_eq!(s, "67554889357866599146897761125791887223681299833479");

        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 315);
        Ok(())
    }
}
