use humantime::format_duration;
use rayon::prelude::*;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Coord = (usize, usize);

fn parse(lines: &[String]) -> Vec<Vec<char>> {
    lines.iter().map(|l| l.chars().collect()).collect()
}

#[allow(clippy::match_on_vec_items, clippy::similar_names)]
fn find_neighbours(grid: &[Vec<char>], (rid, cid): Coord) -> HashSet<Coord> {
    match grid[rid][cid] {
        // If this position is a slope - there is only 1 neighbour which is in the
        // direction of the slope
        '^' if grid[rid - 1][cid] != '#' => vec![(rid - 1, cid)],
        '>' if grid[rid][cid + 1] != '#' => vec![(rid, cid + 1)],
        'v' if grid[rid + 1][cid] != '#' => vec![(rid + 1, cid)],
        '<' if grid[rid][cid + 1] != '#' => vec![(rid, cid - 1)],
        // We're on a path tile
        '.' => {
            let mut n = vec![];
            if rid > 0 && !"#v".contains(grid[rid - 1][cid]) {
                n.push((rid - 1, cid)); // Up
            }
            if cid < grid.len() - 1 && !"#<".contains(grid[rid][cid + 1]) {
                n.push((rid, cid + 1)); // Right
            }
            if rid < grid.len() - 1 && !"#^".contains(grid[rid + 1][cid]) {
                n.push((rid + 1, cid)); // Down
            }
            if cid > 0 && !"#>".contains(grid[rid][cid - 1]) {
                n.push((rid, cid - 1)); // Left
            }
            n
        }
        _ => vec![],
    }
    .into_iter()
    .collect()
}

fn create_edge_map(grid: &[Vec<char>]) -> HashMap<Coord, Vec<(Coord, usize)>> {
    let mut all_edges = HashMap::new();

    // Generate all possible edges
    for (rid, row) in grid.iter().enumerate() {
        for (cid, _) in row.iter().enumerate() {
            let neighbours = find_neighbours(grid, (rid, cid));
            for n in neighbours {
                all_edges
                    .entry((rid, cid))
                    .and_modify(|l: &mut Vec<_>| l.push(n))
                    .or_insert(vec![n]);
            }
        }
    }

    // Compress the set of edges down to the junction points only
    let mut junction_edges = HashMap::new();
    let mut checked = HashSet::new();
    let mut to_check = VecDeque::new();
    to_check.push_back((0, 1));

    // We'll loop over a queue of junction points - which is added too as we discover them
    while let Some(start) = to_check.pop_front() {
        if checked.contains(&start) {
            continue;
        }
        checked.insert(start);

        let possible_nexts = all_edges.get(&start).unwrap().clone();
        for next in possible_nexts {
            let mut history: HashSet<_> = [start, next].into_iter().collect();
            let mut end = next;
            let mut out_edges: Vec<_> = all_edges
                .get(&next)
                .unwrap()
                .iter()
                .filter(|e| !history.contains(e))
                .copied()
                .collect();

            // Keep fast-forwarding until we get to the next junction point
            while out_edges.len() == 1 {
                history.insert(end);
                end = out_edges[0];

                out_edges = all_edges
                    .get(&end)
                    .unwrap()
                    .iter()
                    .filter(|e| !history.contains(e))
                    .copied()
                    .collect();
            }

            // We've now reached the next junction point - set the edge & cost
            junction_edges
                .entry(start)
                .and_modify(|l: &mut Vec<_>| l.push((end, history.len())))
                .or_insert(vec![(end, history.len())]);

            // Add this junction point to the queue for us to check in the future
            to_check.push_back(end);
        }
    }

    junction_edges
}

fn solve_with_edges(
    edges: &HashMap<Coord, Vec<(Coord, usize)>>,
    pos: Coord,
    target: Coord,
    history: &HashSet<Coord>,
    cost: usize,
) -> usize {
    if pos == target {
        return cost;
    }

    edges
        .get(&pos)
        .unwrap()
        .par_iter() // Exploit Rayon to parallelise the recursive checks
        .map(|(next, w)| {
            if history.contains(next) {
                return 0;
            }

            let mut new_hist = history.clone();
            new_hist.insert(*next);
            solve_with_edges(edges, *next, target, &new_hist, cost + w)
        })
        .max()
        .unwrap()
}

fn solve(lines: &[String]) -> usize {
    let grid = parse(lines);
    let edges = create_edge_map(&grid);
    let start = (0, 1);
    let target = (grid.len() - 1, grid[0].len() - 2);
    let history: HashSet<_> = [start].into_iter().collect();
    solve_with_edges(&edges, start, target, &history, 0)
}

fn part_a(lines: &[String]) -> usize {
    solve(lines)
}

fn part_b(lines: &[String]) -> usize {
    let altered_lines: Vec<_> = lines.iter().map(|l| l.replace(['v', '>'], ".")).collect();
    solve(&altered_lines)
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

    const TEST_INPUT: &str = "#.#####################
    #.......#########...###
    #######.#########.#.###
    ###.....#.>.>.###.#.###
    ###v#####.#v#.###.#.###
    ###.>...#.#.#.....#...#
    ###v###.#.#.#########.#
    ###...#.#.#.......#...#
    #####.#.#.#######.#.###
    #.....#.#.#.......#...#
    #.#####.#.#.#########v#
    #.#...#...#...###...>.#
    #.#.#v#######v###.###v#
    #...#.>.#...>.>.#.###.#
    #####v#.#.###v#.#.###.#
    #.....#...#...#.#.#...#
    #.#########.###.#.#.###
    #...###...#...#...#.###
    ###.###.#.###v#####v###
    #...#...#.#.>.>.#.>.###
    #.###.###.#.###.#.#v###
    #.....###...###...#...#
    #####################.#";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 94);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 154);
    }
}
