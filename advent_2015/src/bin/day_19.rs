use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> (HashMap<&str, Vec<&str>>, &str) {
    let mut lookup: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in &lines[..lines.len() - 2] {
        let mut bits = line.split(" => ");
        let src = bits.next().unwrap();
        let target = bits.next().unwrap();
        lookup
            .entry(src)
            .and_modify(|v| {
                v.push(target);
            })
            .or_insert(vec![target]);
    }

    return (lookup, lines[lines.len() - 1].as_str());
}

fn part_a(lines: &[String]) -> usize {
    let (lookup, start) = parse(lines);

    let mut states = HashSet::new();
    for i in 0..start.len() {
        for (src, targets) in &lookup {
            if start[i..].starts_with(src) {
                for target in targets {
                    let mut new = String::new();
                    new.push_str(&start[0..i]);
                    new.push_str(target);
                    new.push_str(&start[(i + src.len())..]);
                    states.insert(new);
                }
            }
        }
    }

    states.len()
}

fn recurse(
    mol: &str,
    depth: usize,
    lookup: &Vec<(&str, &str)>,
    history: &mut HashSet<String>,
) -> Option<usize> {
    if !history.insert(mol.to_string()) {
        return None;
    }

    if mol == "e" {
        return Some(depth);
    }

    for (k, v) in lookup {
        let replaced = mol.replacen(k, v, 1);
        if replaced != mol {
            if let Some(c) = recurse(&replaced, depth + 1, lookup, history) {
                return Some(c);
            }
        }
    }

    None
}

fn part_b(lines: &[String]) -> usize {
    let (lookup, molecule) = parse(lines);

    let mut rev_lookup = vec![];
    for (k, l) in lookup {
        for v in l {
            rev_lookup.push((v, k));
        }
    }

    // Performance of this algo is *highly* dependent on the order of entries in the
    // lookup list.  A descending lexical sort seems to be a good option in this case
    rev_lookup.sort_unstable();
    rev_lookup.reverse();

    let mut history = HashSet::new();
    if let Some(c) = recurse(molecule, 0, &rev_lookup, &mut history) {
        return c;
    }

    panic!();
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

    const TEST_INPUT: &str = "H => HO
                              H => OH
                              O => HH

                              HOHOHO";

    const TEST_INPUT_B: &str = "e => H
                                e => O
                                H => HO
                                H => OH
                                O => HH

                                HOHOHO";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 7);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT_B.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 6);
    }
}
