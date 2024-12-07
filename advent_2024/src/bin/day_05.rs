use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> (HashMap<usize, BTreeSet<usize>>, Vec<Vec<usize>>) {
    let mut past_break = false;
    let mut rules: HashMap<usize, BTreeSet<usize>> = HashMap::new();
    let mut updates = vec![];

    for line in lines {
        if line.is_empty() {
            past_break = true;
            continue;
        }

        if past_break {
            updates.push(line.split(',').flat_map(str::parse).collect());
        } else {
            let mut bits = line.split('|');
            let before: usize = bits.next().map(|s| s.parse().unwrap()).unwrap();
            let after: usize = bits.next().map(|s| s.parse().unwrap()).unwrap();
            rules
                .entry(before)
                .and_modify(|l: _| {
                    l.insert(after);
                })
                .or_insert_with(|| {
                    let mut s = BTreeSet::new();
                    s.insert(after);
                    s
                });
        }
    }

    (rules, updates)
}

fn categorise(
    rules: &HashMap<usize, BTreeSet<usize>>,
    updates: Vec<Vec<usize>>,
) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    updates.into_iter().partition(|l| {
        let mut ok = true;
        for (idx, e) in l.iter().enumerate() {
            if let Some(followers) = rules.get(e) {
                let before: &BTreeSet<_> = &l[..=idx].iter().copied().collect();
                if followers.intersection(before).count() > 0 {
                    ok = false;
                    break;
                }
            }
        }
        ok
    })
}

fn part_a(lines: &[String]) -> usize {
    let (rules, updates) = parse(lines);
    let (correct, _) = categorise(&rules, updates);
    correct.into_iter().map(|l| l[l.len() / 2]).sum()
}

fn no_incoming(outgoing_edges: &HashMap<usize, BTreeSet<usize>>, m: usize) -> bool {
    let targets: BTreeSet<_> = outgoing_edges.values().flatten().collect();
    !targets.contains(&m)
}

fn kahn(rules: &HashMap<usize, BTreeSet<usize>>, input: Vec<usize>) -> Vec<usize> {
    // Kahn's Algorithm
    // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm

    let input: BTreeSet<usize> = input.into_iter().collect();

    // Prereq:
    // Create a subset of rules that only considers this sequence
    // Create a list of all target nodes so we can later determine a "root" node
    let mut all_targets: BTreeSet<usize> = BTreeSet::new();
    let mut outgoing_edges = HashMap::new();
    for n in &input {
        if let Some(l) = rules.get(n) {
            let targets = l
                .iter()
                .filter(|v| input.contains(v))
                .copied()
                .collect::<BTreeSet<_>>();

            all_targets.extend(targets.iter().copied());
            outgoing_edges.insert(*n, targets);
        }
    }

    let mut s: BTreeSet<usize> = input.difference(&all_targets).copied().collect();
    let mut l = vec![];
    while let Some(n) = s.pop_first() {
        l.push(n);

        if let Some(ms) = outgoing_edges.get(&n) {
            for m in ms.clone() {
                outgoing_edges.entry(n).and_modify(|l| {
                    l.remove(&m);
                });

                if no_incoming(&outgoing_edges, m) {
                    s.insert(m);
                }
            }
        }
    }

    l
}

fn part_b(lines: &[String]) -> usize {
    let (rules, updates) = parse(lines);
    let (_, incorrect) = categorise(&rules, updates);

    incorrect
        .into_iter()
        .map(|l| kahn(&rules, l))
        .map(|l| l[l.len() / 2])
        .sum()
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

    const TEST_INPUT: &str = "47|53
                              97|13
                              97|61
                              97|47
                              75|29
                              61|13
                              75|53
                              29|13
                              97|29
                              53|29
                              61|53
                              97|53
                              61|29
                              47|13
                              75|47
                              97|75
                              47|61
                              75|61
                              47|29
                              75|13
                              53|13

                              75,47,61,53,29
                              97,61,53,29,13
                              75,29,13
                              75,97,47,61,53
                              61,13,29
                              97,13,75,29,47";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 143);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 123);
    }
}
