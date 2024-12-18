use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Coord = (isize, isize);

fn parse(lines: &[String]) -> Vec<Coord> {
    lines
        .iter()
        .filter_map(|l| -> Option<Coord> {
            let mut s = l.split(',');
            let x: isize = s.next()?.parse().ok()?;
            let y: isize = s.next()?.parse().ok()?;
            Some((x, y))
        })
        .collect()
}

fn generate_neighbours(u: Coord, blockers: &[Coord], max_dim: isize) -> Vec<Coord> {
    let (ux, uy) = u;

    [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .into_iter()
        .filter_map(|(dx, dy)| {
            let next = (ux + dx, uy + dy);
            let (nx, ny) = next;

            if blockers.contains(&next)
                || (!(0..=max_dim).contains(&nx) || !(0..=max_dim).contains(&ny))
            {
                None
            } else {
                Some(next)
            }
        })
        .collect()
}

fn dijkstra(blockers: &[Coord], max_dim: isize) -> Option<(usize, VecDeque<Coord>)> {
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    let mut queue = BTreeSet::new();

    let start = (0, 0);
    let end: Coord = (max_dim, max_dim);

    dist.insert(start, 0usize);
    queue.insert((0, start));

    while let Some((_, u)) = queue.pop_first() {
        for v in generate_neighbours(u, blockers, max_dim) {
            let alt = dist.get(&u).unwrap() + 1;
            dist.entry(v)
                .and_modify(|dv| {
                    if alt < *dv {
                        *dv = alt;
                        queue.retain(|(_, x)| *x != v);
                        queue.insert((alt, v));
                        prev.insert(v, u);
                    }
                })
                .or_insert_with(|| {
                    queue.insert((alt, v));
                    prev.insert(v, u);
                    alt
                });
        }
    }

    let mut path = VecDeque::new();
    let mut prior = Some(&end);
    while let Some(b) = prior {
        path.push_front(*b);
        prior = prev.get(b);
    }

    dist.get(&end).copied().map(|v| (v, path))
}

fn part_a(lines: &[String], max_dim: isize, limit: usize) -> usize {
    let blockers = parse(lines);
    dijkstra(&blockers[..limit], max_dim)
        .expect("No path found :(")
        .0
}

fn part_b(lines: &[String], max_dim: isize, limit: usize) -> String {
    let blockers = parse(lines);
    let mut last_path = dijkstra(&blockers[..limit], max_dim);

    // Step through each blocker in turn, generating a new path only when the
    // old path is blocked by blockers[l].  If the insertion of the new blocker
    // leads to no path being generated - then we have found our result.
    for l in limit..blockers.len() {
        if let Some((_, path)) = &last_path {
            if path.contains(&blockers[l]) {
                // Regenerate the path with this blocker in place
                last_path = dijkstra(&blockers[..=l], max_dim);
            }
        } else {
            let (bx, by) = blockers[l - 1];
            return format!("{bx},{by}");
        }
    }

    unreachable!()
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
    println!("Part A result = {}", part_a(lines.as_slice(), 70, 1024));
    println!("Part B result = {}", part_b(lines.as_slice(), 70, 1024));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "5,4/4,2/4,5/3,0/2,1/6,3/2,4/1,5/0,6/3,3/2,6/5,1/1,2/5,5/2,5/6,5/1,4/0,4/6,4/1,1/6,1/1,0/0,5/1,6/2,0";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT
            .split('/')
            .map(|l| l.trim().to_string())
            .collect();
        assert_eq!(part_a(lines.as_slice(), 6, 12), 22);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT
            .split('/')
            .map(|l| l.trim().to_string())
            .collect();
        assert_eq!(part_b(&lines, 6, 12), String::from("6,1"));
    }
}
