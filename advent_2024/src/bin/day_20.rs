use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Coord = (isize, isize);

#[allow(clippy::cast_possible_wrap)]
fn parse(lines: &[String]) -> (Coord, Coord, HashSet<Coord>) {
    let mut start = None;
    let mut end = None;

    let mut blockers = HashSet::new();

    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.char_indices() {
            match c {
                '#' => {
                    blockers.insert((x as isize, y as isize));
                }
                'S' => start = Some((x as isize, y as isize)),
                'E' => end = Some((x as isize, y as isize)),
                '.' => (),
                _ => unreachable!(),
            }
        }
    }

    (start.unwrap(), end.unwrap(), blockers)
}

fn generate_neighbours(u: Coord, blockers: &HashSet<Coord>, max_dim: isize) -> Vec<Coord> {
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

fn dijkstra(
    blockers: &HashSet<Coord>,
    start: Coord,
    end: Coord,
    max_dim: isize,
) -> Option<(usize, VecDeque<Coord>)> {
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    let mut queue = BTreeSet::new();

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

#[allow(clippy::cast_possible_wrap)]
fn solution(lines: &[String], max_cheat: isize, count_threshold: isize) -> usize {
    // Start by running a normal dijkstra to get the shortest "normal" path
    let (start, end, blockers) = parse(lines);
    let max_dim = lines.len() as isize;
    let (_, path) = dijkstra(&blockers, start, end, max_dim).unwrap();

    // Identify cheats by walking along the path and checking all forward shortcuts
    // - cheat distance is the manhattan distance between the two points
    let mut cheats = HashMap::new();
    for (ai, (ax, ay)) in path.iter().enumerate() {
        for (bi, (bx, by)) in path.iter().enumerate().skip(ai + 2) {
            let cheat_dist = (ax - bx).abs() + (ay - by).abs();
            let path_distance = (bi - ai) as isize;
            let save = path_distance - cheat_dist;

            if cheat_dist <= max_cheat && save >= count_threshold {
                cheats.entry(save).and_modify(|i| *i += 1).or_insert(1);
            }
        }
    }

    cheats.values().sum()
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
    println!("Part A result = {}", solution(&lines, 2, 100));
    println!("Part B result = {}", solution(&lines, 20, 100));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "###############
                              #...#...#.....#
                              #.#.#.#.#.###.#
                              #S#...#.#.#...#
                              #######.#.#.###
                              #######.#.#...#
                              #######.#.###.#
                              ###..E#...#...#
                              ###.#######.###
                              #...###...#...#
                              #.#####.#.###.#
                              #.#...#.#.#...#
                              #.#.#.#.#.#.###
                              #...#...#...###
                              ###############";

    #[test]
    fn test_solution() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(solution(&lines, 2, 1), 44);
        assert_eq!(solution(&lines, 2, 20), 5);
        assert_eq!(solution(&lines, 2, 64), 1);

        assert_eq!(solution(&lines, 20, 50), 285);
        assert_eq!(solution(&lines, 20, 70), 7 + 22 + 12);
        assert_eq!(solution(&lines, 20, 72), 7 + 22);
        assert_eq!(solution(&lines, 20, 74), 7);
        assert_eq!(solution(&lines, 20, 76), 3);
    }
}
