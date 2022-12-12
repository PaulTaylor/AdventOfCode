use humantime::format_duration;
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Coord = (usize, usize);

#[derive(Debug, Eq, Clone, Copy)]
struct Adj(u64, Coord);

impl PartialEq for Adj {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Adj {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Adj {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Equal => self.1.cmp(&other.1),
            x => x,
        }
    }
}

fn parse(lines: &[String]) -> AResult<Vec<Vec<char>>> {
    Ok(lines.iter().map(|l| l.chars().collect()).collect())
}

fn find_neighbours<F: Fn(char, char) -> bool>(
    grid: &[Vec<char>],
    coords: Coord,
    can_move: F,
) -> Vec<Coord> {
    let mut neighbours = Vec::new();
    let (ri, ci) = coords;
    let c = grid[ri][ci];

    if ri > 0 && can_move(c, grid[ri - 1][ci]) {
        neighbours.push((ri - 1, ci))
    }
    if ri < grid.len() - 1 && can_move(c, grid[ri + 1][ci]) {
        neighbours.push((ri + 1, ci))
    }
    if ci > 0 && can_move(c, grid[ri][ci - 1]) {
        neighbours.push((ri, ci - 1))
    }
    if ci < grid[0].len() - 1 && can_move(c, grid[ri][ci + 1]) {
        neighbours.push((ri, ci + 1));
    }

    neighbours
}

#[allow(non_snake_case)]
fn part_a(lines: &[String]) -> AResult<u64> {
    let grid = parse(lines)?;

    // it's dijkstra time!
    let mut dist: HashMap<Coord, u64> = HashMap::new();
    let mut Q: BTreeSet<Adj> = BTreeSet::new();

    'outer: for (ri, row) in grid.iter().enumerate() {
        for (ci, _) in row.iter().enumerate() {
            let coords = (ri, ci);
            if grid[ri][ci] == 'S' {
                dist.insert(coords, 0);
                Q.insert(Adj(0, coords));
                break 'outer;
            }
        }
    }

    // Define the function for valid moves
    let can_move = |old: char, new: char| -> bool {
        match (old, new) {
            ('z', _) => true,   // z is the highest - can go anywhere from here (incl E)
            ('y', 'E') => true, // E is considered to have height z so is a vaild move from y
            ('S', 'a') => true, // S has height a
            ('S', 'b') => true, // S has height a
            ('E', _) => false,  // Cannot leave E once we arrive
            (o, n) => (n <= ((o as u8) + 1) as char) && n >= 'a',
        }
    };

    while let Some(&u_adj) = Q.iter().next() {
        Q.remove(&u_adj);

        let Adj(u_dist, u) = u_adj;
        if grid[u.0][u.1] == 'E' {
            return Ok(u_dist); // reached the target
        }

        let neighbours = find_neighbours(&grid, u, can_move);
        for v in neighbours {
            let alt = u_dist + 1;
            let v_dist = *dist.get(&v).unwrap_or(&u64::MAX);
            if alt < v_dist {
                dist.insert(v, alt);
                Q.remove(&Adj(v_dist, v));
                Q.insert(Adj(alt, v));
            }
        }
    }

    Err(anyhow::format_err!("Not found"))
}

#[allow(non_snake_case)]
fn part_b(lines: &[String]) -> AResult<u64> {
    let grid = parse(lines)?;

    // it's dijkstra time again only this time in reverse
    let mut dist: HashMap<Coord, u64> = HashMap::new();
    let mut Q: BTreeSet<Adj> = BTreeSet::new();

    'outer: for (ri, row) in grid.iter().enumerate() {
        for (ci, _) in row.iter().enumerate() {
            let coords = (ri, ci);
            if grid[ri][ci] == 'E' {
                dist.insert(coords, 0);
                Q.insert(Adj(0, coords));
                break 'outer;
            }
        }
    }

    // Define the function for valid moves
    let can_move = |old: char, new: char| -> bool {
        // Clone of can_move(...) with part_b's criteria
        match (old, new) {
            ('E', 'z') => true,
            ('E', 'y') => true,
            ('E', _) => false,
            ('b', 'S') => true,
            (n, o) => (n <= ((o as u8) + 1) as char) && n >= 'a',
        }
    };

    while let Some(&u_adj) = Q.iter().next() {
        Q.remove(&u_adj);

        let Adj(u_dist, u) = u_adj;
        if grid[u.0][u.1] == 'a' {
            return Ok(u_dist); // reached the target
        }

        let neighbours = find_neighbours(&grid, u, can_move);
        for v in neighbours {
            let alt = u_dist + 1;
            let v_dist = *dist.get(&v).unwrap_or(&u64::MAX);
            if alt < v_dist {
                dist.insert(v, alt);
                Q.remove(&Adj(v_dist, v));
                Q.insert(Adj(alt, v));
            }
        }
    }

    Err(anyhow::format_err!("Not found"))
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

    const TEST_INPUT: &str = "Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 31);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 29);
        Ok(())
    }
}
