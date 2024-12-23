use humantime::format_duration;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> HashMap<String, BTreeSet<String>> {
    let mut edges = HashMap::new();
    for (a, b) in lines.iter().flat_map(|l| {
        let (a, b) = l.split('-').collect_tuple().unwrap();
        [
            (a.to_string(), b.to_string()),
            (b.to_string(), a.to_string()),
        ]
    }) {
        edges
            .entry(a)
            .and_modify(|s: &mut BTreeSet<_>| {
                s.insert(b.to_string());
            })
            .or_insert_with(|| {
                let mut s = BTreeSet::new();
                s.insert(b.to_string());
                s
            });
    }
    edges
}

fn part_a(lines: &[String]) -> usize {
    let edges = parse(lines);
    let vertices: BTreeSet<_> = edges.keys().cloned().collect();

    // Find cycles of length 3 in the graph with brute-force path generation
    let mut candidates: VecDeque<_> = vertices.iter().map(|v| vec![v.to_string()]).collect();
    let mut cycles = BTreeSet::new();

    while let Some(path) = candidates.pop_front() {
        if path.len() == 4 && path[0] == path[3] {
            cycles.insert(path[0..3].iter().cloned().collect::<BTreeSet<_>>());
        } else if path.len() < 4 {
            let mut new = vec![];
            if let Some(to_set) = edges.get(&path[path.len() - 1]) {
                for next in to_set {
                    let mut np = path.clone();
                    np.push(next.to_string());
                    new.push(np);
                }
            }
            candidates.extend(new);
        }
    }
    cycles
        .into_iter()
        .filter(|l| l.iter().any(|v| v.starts_with('t')))
        .count()
}

fn bron_kerbosch(
    r: &BTreeSet<String>,
    mut p: BTreeSet<String>,
    mut x: BTreeSet<String>,
    edges: &HashMap<String, BTreeSet<String>>,
    cliques: &mut Vec<BTreeSet<String>>,
) -> Option<()> {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.iter().map(ToString::to_string).collect());
    }

    while !p.is_empty() {
        let v = p.first()?.to_string();
        let r_v = r.iter().chain([&v]).cloned().collect();
        let p_v = p.intersection(edges.get(&v)?).cloned().collect();
        let x_v = x.intersection(edges.get(&v)?).cloned().collect();
        bron_kerbosch(&r_v, p_v, x_v, edges, cliques);
        p.remove(&v);
        x.insert(v);
    }

    None
}

fn part_b(lines: &[String]) -> String {
    // Find the maximum clique in the graph
    // Use Bron-Kerbosch to generate all maximal cliques and then find the biggest
    // https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm

    let edges = parse(lines);
    let vertices: BTreeSet<_> = edges.keys().map(ToString::to_string).collect();

    let mut cliques = vec![];
    bron_kerbosch(
        &BTreeSet::new(),
        vertices,
        BTreeSet::new(),
        &edges,
        &mut cliques,
    );

    cliques
        .into_iter()
        .max_by_key(BTreeSet::len)
        .map(|s| s.into_iter().join(","))
        .unwrap()
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

    const TEST_INPUT: &str = "kh-tc
                              qp-kh
                              de-cg
                              ka-co
                              yn-aq
                              qp-ub
                              cg-tb
                              vc-aq
                              tb-ka
                              wh-tc
                              yn-cg
                              kh-ub
                              ta-co
                              de-co
                              tc-td
                              tb-wq
                              wh-td
                              ta-ka
                              td-qp
                              aq-cg
                              wq-ub
                              ub-vc
                              de-ta
                              wq-aq
                              wq-vc
                              wh-yn
                              ka-de
                              kh-ta
                              co-tc
                              wh-qp
                              tb-vc
                              td-yn";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(&lines), 7);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), "co,de,ka,ta");
    }
}
