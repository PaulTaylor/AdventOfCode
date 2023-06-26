use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

#[derive(Debug)]
struct Valve {
    id: String,
    rate: usize,
    tunnels: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Path(usize, usize, Vec<String>);

#[derive(Debug)]
struct Distances<'a> {
    dist: HashMap<(&'a String, &'a String), usize>,
    next: HashMap<(&'a String, &'a String), &'a String>,
}

impl<'a> Distances<'a> {
    fn new(valves: &'a HashMap<String, Valve>) -> Self {
        let mut dist = HashMap::new();
        let mut next = HashMap::new();

        for u in valves.values() {
            for v in &u.tunnels {
                dist.insert((&u.id, v), 1);
                next.insert((&u.id, v), v);
            }

            dist.insert((&u.id, &u.id), 0);
            next.insert((&u.id, &u.id), &u.id);
        }

        for k in valves.keys() {
            for i in valves.keys() {
                for j in valves.keys() {
                    let o_dik = dist.get(&(i, k));
                    let o_dkj = dist.get(&(k, j));
                    let dij = dist.get(&(i, j)).unwrap_or(&usize::MAX);

                    if let (Some(dik), Some(dkj)) = (o_dik, o_dkj) {
                        let alt = dik + dkj;
                        if dij > &alt {
                            dist.insert((i, j), alt);
                            next.insert((i, j), next[&(i, k)]);
                        }
                    }
                }
            }
        }

        Distances { dist, next }
    }

    fn route(&self, src: String, dest: &String) -> Vec<String> {
        let mut r = vec![src];
        while r.last().unwrap() != dest {
            let k = &(&r.last().unwrap()[..2].to_string(), dest);
            let next = self.next[k].clone();
            r.push(next);
        }
        r.remove(0); // remove the start otherwise it'll be duplicated
        r.push(format!("{dest}_O")); // Open the valve when we get to it
        r
    }
}

fn parse(lines: &[String]) -> AResult<HashMap<String, Valve>> {
    let pattern = Regex::new(
        "^Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnel(?:s)? lead(?:s)? to valve(?:s)? (.*)$",
    )?;
    Ok(lines
        .iter()
        .map(|l| {
            let c = pattern
                .captures(l)
                .unwrap_or_else(|| panic!("regex does not match - {l}"));
            let id = c.get(1).unwrap().as_str().to_string();
            let v = Valve {
                id: id.clone(),
                rate: c.get(2).unwrap().as_str().parse().unwrap(),
                tunnels: c
                    .get(3)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(Into::into)
                    .collect(),
            };
            (id, v)
        })
        .collect())
}

fn bounds(path: &[String], valves: &HashMap<String, Valve>) -> (usize, usize) {
    let current = {
        let mut acc = 0;
        for (t, a) in path.iter().enumerate() {
            if a.ends_with("_O") {
                let rem_time = 30 - t;
                acc += rem_time * valves[&a[0..2]].rate;
            }
        }
        acc
    };

    if path.len() == 31 {
        // Quick exit if the path is 30 minutes long as we can't possibly do more
        return (current, current);
    }

    // Upper bound is if all valves were instantly activtated - not possible in
    // the actual scenario but it is easy to calculate
    let remaining = {
        let remaining_mins: usize = 31 - path.len();
        let all: HashSet<_> = valves.keys().filter(|&v| valves[v].rate > 0).collect();
        let open_valves: Vec<_> = path[1..]
            .iter()
            .filter_map(|x| x.strip_suffix("_O"))
            .map(ToOwned::to_owned)
            .collect();

        let path_set = open_valves.iter().collect();
        let mut unopened_valves: Vec<_> = all.difference(&path_set).collect();
        unopened_valves.sort_by_key(|&&v| valves[v].rate);

        let tot: usize = unopened_valves
            .iter()
            .map(|&&v| valves[v].rate)
            .take(remaining_mins)
            .sum();

        tot * remaining_mins
    };

    (current, current + remaining)
}

#[allow(clippy::similar_names)]
fn part_a(lines: &[String]) -> AResult<usize> {
    let valves = parse(lines)?;
    let all_valves: HashSet<String> = valves
        .iter()
        .filter_map(|(k, v)| if v.rate > 0 { Some(k) } else { None })
        .cloned()
        .collect();

    // Create a distance matrix for all valve pairs
    let distances = Distances::new(&valves);

    let mut queue: BTreeSet<Path> = BTreeSet::new();
    let start = vec!["AA".to_string()];
    let (lb, ub) = bounds(&start, &valves);

    queue.insert(Path(
        ub, // upper bound - what happens if we activate all valves now
        lb, // lower bound - the current pressure
        start,
    ));

    // path is the option with the highest possible upper bound
    while let Some(Path(ub, lb, path)) = queue.pop_last() {
        // Exit when we have a fully realised path that is the maximal path
        if ub == lb {
            return Ok(ub);
        }

        let open_valves: HashSet<String> = path
            .iter()
            .filter_map(|x| x.strip_suffix("_O"))
            .map(str::to_string)
            .collect();

        let unopened_valves: Vec<_> = all_valves.difference(&open_valves).collect();

        for n in unopened_valves {
            let n = n.clone();
            let mut new_path: Vec<_> = path.clone();
            new_path.extend(distances.route(path.last().unwrap().clone(), &n));

            // calculate new bounds for new path
            let (new_lb, new_ub) = bounds(&new_path, &valves);
            // Enqueue if it's not too long
            if new_path.len() <= 31 {
                let new_path = Path(new_ub, new_lb, new_path);
                queue.insert(new_path);
            }
        }

        // Remove elements from the queue if the upper bound is < the the original
        // lb for this potential solution
        queue.retain(|x| x.0 >= lb);
    }

    Err(anyhow::format_err!("Solution is not found"))
}

#[derive(PartialEq, Eq, Clone)]
struct BPath {
    path: Vec<String>,
    score: usize,
    opened: HashSet<String>,
}

impl PartialOrd for BPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.score, &self.path).cmp(&(other.score, &other.path))
    }
}

fn path_score(p: &[String], valves: &HashMap<String, Valve>) -> usize {
    let mut acc: usize = 0;
    for (t, a) in p.iter().enumerate() {
        if a.ends_with("_O") {
            let rem_time = 25 - t;
            acc += rem_time * valves[&a[0..2]].rate;
        }
    }
    acc
}

impl BPath {
    fn new(path: Vec<String>, valves: &HashMap<String, Valve>, opened: &HashSet<String>) -> BPath {
        BPath {
            score: path_score(&path, valves),
            path,
            opened: opened.clone(),
        }
    }
}

fn part_b(lines: &[String]) -> AResult<usize> {
    let valves = parse(lines)?;
    let all_valves: HashSet<String> = valves
        .iter()
        .filter_map(|(k, v)| if v.rate > 0 { Some(k) } else { None })
        .cloned()
        .collect();

    // Create a distance matrix for all valve pairs
    let distances = &Distances::new(&valves);

    // Generate the paths possible in the time limit (26 minutes)
    // Search for the two largest (non-overlapping) paths in that set

    let mut paths: BTreeSet<BPath> = BTreeSet::new();
    let mut queue: Vec<Vec<String>> = vec![];
    let mut first = true;

    while !queue.is_empty() || first {
        first = false;

        let path = queue.pop().unwrap_or_default();
        let loc = path.last().unwrap_or(&"AA".to_string()).clone()[..2].to_string();

        let visited = &path
            .iter()
            .filter_map(|x| x.strip_suffix("_O").map(str::to_string))
            .collect();

        let unopened = all_valves.difference(visited);

        for next in unopened {
            let dist = distances.dist[&(&loc, next)];
            if path.len() + dist < 26 {
                let route = distances.route(loc.clone(), next);
                queue.push(path.iter().cloned().chain(route.iter().cloned()).collect());
            }
        }

        if !path.is_empty() {
            paths.insert(BPath::new(path, &valves, visited));
        }
    }

    let best_segment_score = paths.last().unwrap().score;
    let mut best_sofar = best_segment_score;

    for left in paths.iter().rev() {
        if left.score + best_segment_score < best_sofar {
            break;
        }

        // Iterate over the other paths (in descending score order) where the right
        // path is completely disjoint with the left path
        for right in paths
            .iter()
            .rev()
            .filter(|o| left.opened.is_disjoint(&o.opened))
        {
            if left.score + right.score <= best_sofar {
                break;
            }

            let combined_score = left.score + right.score;
            if combined_score > best_sofar {
                best_sofar = combined_score;
            }
        }
    }

    Ok(best_sofar)
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
    println!("Part A result = {}", part_a(lines.as_slice())?);
    println!("Part B result = {}", part_b(lines.as_slice())?);
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_bounds() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let valves = parse(lines.as_slice())?;
        let path = [
            "AA", "DD", "DD_O", "CC", "BB", "BB_O", "AA", "II", "JJ", "JJ_O", "II", "AA", "DD",
            "EE", "FF", "GG", "HH", "HH_O", "GG", "FF", "EE", "EE_O", "DD", "CC", "CC_O",
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<String>>();

        assert_eq!(path.len(), 25);

        // Cost of this path should be exactly 1651
        assert_eq!((1651, 1651), bounds(path.as_slice(), &valves));

        for i in 1..path.len() {
            let (lb, ub) = bounds(&path[..i], &valves);
            assert!(lb < 1651);
            assert!(ub > 1651);
        }
        Ok(())
    }

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 1651);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        let r = part_b(lines.as_slice())?;
        assert_eq!(r, 1707);
        Ok(())
    }
}
