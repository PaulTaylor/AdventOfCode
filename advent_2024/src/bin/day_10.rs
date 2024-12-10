use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Coord = (isize, isize);
type Grid = HashMap<Coord, usize>;

#[allow(clippy::cast_possible_wrap)]
fn parse(lines: &[String]) -> (Grid, HashSet<Coord>) {
    let (grid_entries, zero_entries): (Vec<_>, Vec<_>) = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices()
                .map(|(x, c)| {
                    let ge = ((x as isize, y as isize), (c as usize) - 48);
                    if c == '0' {
                        (ge, Some((x as isize, y as isize)))
                    } else {
                        (ge, None)
                    }
                })
                .collect::<Vec<_>>()
        })
        .unzip();

    (
        grid_entries.into_iter().collect(),
        zero_entries.into_iter().flatten().collect(),
    )
}

fn recurse(x: isize, y: isize, mut path: Vec<Coord>, grid: &Grid) -> HashSet<Vec<Coord>> {
    path.push((x, y));

    let mut out = HashSet::new();
    match grid.get(&(x, y)) {
        Some(9) => {
            out.insert(path);
            return out;
        }
        Some(&v) => {
            for (xd, yd) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                match grid.get(&(x + xd, y + yd)) {
                    Some(&v2) if v2 == v + 1 => {
                        out.extend(recurse(x + xd, y + yd, path.clone(), grid));
                    }
                    _ => {}
                }
            }
        }
        _ => unreachable!(),
    }
    out
}

fn part_ab(lines: &[String]) -> (usize, usize) {
    let (grid, zeroes) = parse(lines);

    // Walk the paths recursively
    let mut paths: HashMap<Coord, HashSet<Vec<Coord>>> = HashMap::new();
    for (zx, zy) in zeroes {
        let end_points = recurse(zx, zy, vec![(zx, zy)], &grid);
        paths
            .entry((zx, zy))
            .and_modify(|s| s.extend(end_points.clone()))
            .or_insert(end_points);
    }

    // Generate the results from the sets of paths
    let unique_end_sum = paths
        .values()
        .map(|s| {
            s.iter()
                .map(|p| *p.last().unwrap())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();

    let unique_path_sum = paths.values().map(HashSet::len).sum();

    (unique_end_sum, unique_path_sum)
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
    let (a, b) = part_ab(lines.as_slice());
    println!("Part A result = {a}");
    println!("Part B result = {b}");
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "89010123
                              78121874
                              87430965
                              96549874
                              45678903
                              32019012
                              01329801
                              10456732";

    #[test]
    fn test_ab() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_ab(lines.as_slice()), (36, 81));
    }
}
