use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> AResult<HashMap<&str, Vec<&str>>> {
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines {
        let mut it = line.split('-');
        let (c1, c2) = (it.next().unwrap(), it.next().unwrap());

        adj.entry(c1)
            .and_modify(|list| list.push(c2))
            .or_insert_with(|| Vec::from_iter([c2]));

        if c1 != "start" {
            // start cannot be a destination :)
            adj.entry(c2)
                .and_modify(|list| list.push(c1))
                .or_insert_with(|| Vec::from_iter([c1]));
        }
    }

    Ok(adj)
}

fn extend_path<'a>(
    existing: Vec<Vec<&'a str>>,
    adj: &HashMap<&'a str, Vec<&'a str>>,
) -> Vec<Vec<&'a str>> {
    let mut out = Vec::new();

    for path in existing {
        if path[path.len() - 1] == "end" {
            out.push(path); // reached the end of this path - no extension to do
            continue;
        }

        let last_elem = path[path.len() - 1];
        for dest in &adj[last_elem] {
            let small: HashSet<_> = path
                .iter()
                .filter(|s| &&s.to_lowercase().as_str() == s)
                .collect();

            if small.contains(dest) {
                continue; // no double visiting small caves
            }

            let mut clone = path.clone();
            clone.push(dest);
            out.push(clone);
        }
    }

    out
}

fn part_a(lines: &[String]) -> AResult<u64> {
    let adj = parse(lines)?;

    let mut paths = Vec::from_iter(adj.get("start").unwrap().iter().map(|x| vec![*x]));

    let mut prev_paths: Vec<_> = vec![];
    let mut pp_len = prev_paths.len();
    while pp_len != paths.len() {
        prev_paths = paths;
        pp_len = prev_paths.len();
        paths = extend_path(prev_paths, &adj);
    }

    Ok(paths.len() as u64)
}

fn extend_path_b<'a>(
    existing: Vec<Vec<&'a str>>,
    adj: &HashMap<&'a str, Vec<&'a str>>,
) -> Vec<Vec<&'a str>> {
    let mut out = Vec::new();

    for path in existing {
        if path[path.len() - 1] == "end" {
            out.push(path); // reached the end of this path - no extension to do
            continue;
        }

        let mut small_counts: HashMap<&str, u8> = HashMap::new();
        path.iter()
            .filter(|s| &&s.to_lowercase().as_str() == s)
            .for_each(|v| {
                small_counts
                    .entry(v)
                    .and_modify(|c| *c += 1u8)
                    .or_insert(1u8);
            });
        let small_max = small_counts.values().max().unwrap_or(&0);

        let last_elem = path[path.len() - 1];
        for dest in &adj[last_elem] {
            let prev = small_counts.get(dest).unwrap_or(&0);
            if (&(dest.to_lowercase().as_str()) != dest) || // Big cave - valid extension
                (*prev == 0) || // Small cave we've not been in before
                ((*small_max == 1) && (*prev == 1))
            // dest is a small cave, but we can visit it a second time
            {
                let mut clone = path.clone();
                clone.push(dest);
                out.push(clone);
            }
        }
    }

    out
}

fn part_b(lines: &[String]) -> AResult<u64> {
    let adj = parse(lines)?;
    let mut paths = Vec::from_iter(adj.get("start").unwrap().iter().map(|x| vec![*x]));

    let mut prev_paths: Vec<_> = vec![];
    let mut pp_len = prev_paths.len();
    while pp_len != paths.len() {
        prev_paths = paths;
        pp_len = prev_paths.len();
        paths = extend_path_b(prev_paths, &adj);
    }

    Ok(paths.len() as u64)
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
    let file = File::open(format!("./data/day_{ex}_a.txt"))?;
    let lines: Vec<String> = BufReader::new(file).lines().map(Result::unwrap).collect();

    // Run the solutions
    let start = Instant::now();
    println!("Part A result = {}", part_a(lines.as_slice())?);
    println!("Part B result = {}", part_b(lines.as_slice())?);
    let end = Instant::now();

    println!("Run took {}ms", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 226);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 3509);
        Ok(())
    }
}
