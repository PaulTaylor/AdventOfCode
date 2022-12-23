use humantime::format_duration;
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> HashSet<(isize, isize)> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(ri, line)| {
            line.char_indices().filter_map(move |(ci, v)| {
                if v == '#' {
                    Some((ci.try_into().unwrap(), ri.try_into().unwrap()))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[allow(clippy::too_many_lines)]
fn solve(lines: &[String], rounds: usize) -> (isize, Option<usize>) {
    let mut elves = parse(lines);
    let n_elves = elves.len();
    let mut consideration: VecDeque<_> = "NSWE".chars().collect();

    let mut n_rounds = None;
    for round in 0..rounds {
        // First Half
        let mut stationary = HashSet::new();
        let mut movers = HashSet::new();
        for (ex, ey) in &elves {
            let mut neighbours = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx != 0 || dy != 0 {
                        neighbours += &elves.contains(&(ex + dx, ey + dy)).into();
                    }
                }
            }

            if neighbours == 0 {
                stationary.insert((ex, ey));
            } else {
                movers.insert((ex, ey));
            }
        }

        if stationary.len() == n_elves {
            n_rounds = Some(round + 1);
            break;
        }

        let mut proposals: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
        for (&ex, &ey) in movers {
            for d in &consideration {
                match d {
                    'N' if !(elves.contains(&(ex, ey - 1))
                        || elves.contains(&(ex - 1, ey - 1))
                        || elves.contains(&(ex + 1, ey - 1))) =>
                    {
                        let target = (ex, ey - 1);
                        proposals.insert((ex, ey), target);
                        break;
                    }
                    'S' if !(elves.contains(&(ex, ey + 1))
                        || elves.contains(&(ex - 1, ey + 1))
                        || elves.contains(&(ex + 1, ey + 1))) =>
                    {
                        let target = (ex, ey + 1);
                        proposals.insert((ex, ey), target);
                        break;
                    }
                    'W' if !(elves.contains(&(ex - 1, ey))
                        || elves.contains(&(ex - 1, ey - 1))
                        || elves.contains(&(ex - 1, ey + 1))) =>
                    {
                        let target = (ex - 1, ey);
                        proposals.insert((ex, ey), target);
                        break;
                    }
                    'E' if !(elves.contains(&(ex + 1, ey))
                        || elves.contains(&(ex + 1, ey - 1))
                        || elves.contains(&(ex + 1, ey + 1))) =>
                    {
                        let target = (ex + 1, ey);
                        proposals.insert((ex, ey), target);
                        break;
                    }
                    _ => {}
                };
            }
        }

        let clashes: HashSet<_> = proposals
            .values()
            .fold(HashMap::new(), |mut m, item| {
                m.entry(item).and_modify(|v| *v += 1).or_insert(1);
                m
            })
            .into_iter()
            .filter_map(|(target, count)| if count > 1 { Some(target) } else { None })
            .collect();

        // Second Half
        for (source, target) in &proposals {
            if !clashes.contains(target) {
                elves.remove(source);
                elves.insert(*target);
            }
        }

        // Rotate consideration
        let c = consideration.pop_front().unwrap();
        consideration.push_back(c);

        assert_eq!(elves.len(), n_elves, "some elves were lost");
    }

    // find bounding box
    let (min_x, min_y, max_x, max_y) = elves.iter().fold(
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
        |acc, &(ex, ey)| {
            let (min_x, min_y, max_x, max_y) = acc;
            (
                min(min_x, ex),
                min(min_y, ey),
                max(max_x, ex),
                max(max_y, ey),
            )
        },
    );

    let mut acc = ((max_x + 1) - min_x) * ((max_y + 1) - min_y);
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if elves.contains(&(x, y)) {
                acc -= 1;
            }
        }
    }

    (acc, n_rounds)
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
    println!("Part A result = {}", solve(lines.as_slice(), 10).0);
    println!(
        "Part B result = {}",
        solve(lines.as_slice(), usize::MAX).1.unwrap()
    );
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT: &str = ".....
    ..##.
    ..#..
    .....
    ..##.
    .....";

    const TEST_INPUT: &str = "....#..
    ..###.#
    #...#.#
    .#...##
    #.###..
    ##.#.##
    .#..#..";

    #[test]
    fn test_a() {
        let lines: Vec<_> = SMALL_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(solve(lines.as_slice(), 10).0, 25);
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(solve(lines.as_slice(), 10).0, 110);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(solve(lines.as_slice(), usize::MAX).1.unwrap(), 20);
    }
}
