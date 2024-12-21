use humantime::format_duration;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn generate_paths(
    singles: &[(char, char, char)],
    vertices: &str,
) -> HashMap<(char, char), Vec<String>> {
    let mut out = HashMap::new();
    for start in vertices.chars() {
        let mut queue = BTreeSet::new();
        let mut dist = HashMap::new();
        dist.insert(start, 0);
        queue.insert((0, start));

        let mut prev = HashMap::new();

        while let Some((_, u)) = queue.pop_first() {
            let neighbours = singles.iter().filter(|(s, _, _)| *s == u);
            for (_, d, v) in neighbours {
                let alt = dist.get(&u).unwrap() + 1;
                let dv = *dist.get(&v).unwrap_or(&i32::MAX);
                if alt <= dv {
                    queue.insert((alt, *v));

                    if alt < dv {
                        prev.insert(*v, vec![(u, d)]);
                        dist.insert(*v, alt);
                    }

                    if alt == dv {
                        prev.entry(*v).and_modify(|l| l.push((u, d)));
                    }
                }
            }
        }

        // Reconstruct paths
        for target in vertices.chars() {
            let paths = rp(target, &prev, "");
            out.insert((start, target), paths);
        }
    }
    out
}

fn rp(target: char, prev: &HashMap<char, Vec<(char, &char)>>, base: &str) -> Vec<String> {
    if let Some(l) = prev.get(&target) {
        l.iter()
            .flat_map(|(p, dir)| {
                let new_base = format!("{dir}{base}");
                rp(*p, prev, &new_base)
            })
            .collect_vec()
    } else {
        vec![base.to_string()]
    }
}

fn generate_keypad_moves() -> HashMap<(char, char), Vec<String>> {
    let singles = [
        ('0', '^', '2'),
        ('0', '>', 'A'),
        ('A', '^', '3'),
        ('A', '<', '0'),
        ('1', '^', '4'),
        ('1', '>', '2'),
        ('2', '<', '1'),
        ('2', '^', '5'),
        ('2', '>', '3'),
        ('2', 'v', '0'),
        ('3', '<', '2'),
        ('3', '^', '6'),
        ('3', 'v', 'A'),
        ('4', '^', '7'),
        ('4', '>', '5'),
        ('4', 'v', '1'),
        ('5', '<', '4'),
        ('5', '^', '8'),
        ('5', '>', '6'),
        ('5', 'v', '2'),
        ('6', '<', '5'),
        ('6', '^', '9'),
        ('6', 'v', '3'),
        ('7', '>', '8'),
        ('7', 'v', '4'),
        ('8', '<', '7'),
        ('8', '>', '9'),
        ('8', 'v', '5'),
        ('9', '<', '8'),
        ('9', 'v', '6'),
    ];

    let vertices = "0123456789A";
    generate_paths(&singles, vertices)
}

fn generate_dirpad_moves() -> HashMap<(char, char), Vec<String>> {
    let singles = [
        ('<', '>', 'v'),
        ('^', '>', 'A'),
        ('^', 'v', 'v'),
        ('v', '<', '<'),
        ('v', '^', '^'),
        ('v', '>', '>'),
        ('>', '^', 'A'),
        ('>', '<', 'v'),
        ('A', '<', '^'),
        ('A', 'v', '>'),
    ];
    let vertices = "^A<v>";
    generate_paths(&singles, vertices)
}

fn get_shortest_len(
    input: &str,
    robots: usize,
    paths: &HashMap<(char, char), Vec<String>>,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    if robots == 0 {
        return input.len();
    }

    if let Some(r) = cache.get(&(input.to_string(), robots)) {
        return *r;
    }

    let mut res = 0;
    let mut prev = 'A';
    for next in input.chars() {
        let candidates = paths.get(&(prev, next)).unwrap();
        res += candidates
            .iter()
            .map(|p| get_shortest_len(&format!("{p}A"), robots - 1, paths, cache))
            .min()
            .unwrap();
        prev = next;
    }

    cache.insert((input.to_string(), robots), res);

    res
}

fn solve(lines: &[String], robots: usize) -> usize {
    let keypad_paths = generate_keypad_moves();
    let dirpad_paths = generate_dirpad_moves();

    let pattern = Regex::new(r"\d+").unwrap();
    let mut acc = 0;
    for code in lines {
        let mut cache = HashMap::new();

        let mut shortest = 0;
        let mut prev = 'A';
        for digit in code.chars() {
            let mut d_shortest = usize::MAX;
            let expansions = keypad_paths.get(&(prev, digit)).unwrap();

            for e in expansions {
                d_shortest = std::cmp::min(
                    d_shortest,
                    get_shortest_len(&format!("{e}A"), robots, &dirpad_paths, &mut cache),
                );
            }
            shortest += d_shortest;
            prev = digit;
        }

        let num_in_code: usize = pattern
            .find(code)
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or_else(|| panic!("Didn't find a number in {code}"));
        acc += num_in_code * shortest;
    }
    acc
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
    println!("Part A result = {}", solve(lines.as_slice(), 2));
    println!("Part B result = {}", solve(lines.as_slice(), 25));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "029A
                              980A
                              179A
                              456A
                              379A";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(solve(lines.as_slice(), 2), 126_384);
    }
}
