use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Instructions = (String, HashMap<(char, char), char>);

fn parse(lines: &[String]) -> AResult<Instructions> {
    let template = lines[0].clone();

    let rules: HashMap<_, _> = lines[2..]
        .iter()
        .map(|l| -> Option<((char, char), char)> {
            let mut it = l.split(" -> ");
            let mut k_it = it.next()?.chars();
            let key = (k_it.next()?, k_it.next()?);
            Some((key, it.next()?.chars().next()?))
        })
        .map(Option::unwrap)
        .collect();

    Ok((template, rules))
}

fn solve(lines: &[String], n: usize) -> AResult<u64> {
    let (templ, rules) = parse(lines)?;

    let mut counts: HashMap<(char, char), u64> = HashMap::new();
    for pair in zip(templ.chars(), templ[1..].chars()) {
        counts.entry(pair).and_modify(|v| *v += 1).or_insert(1);
    }

    for _i in 0..n {
        let mut new: HashMap<(char, char), u64> = HashMap::new();
        for ((a, b), c) in counts {
            let x = *rules.get(&(a, b)).unwrap();
            new.entry((a, x)).and_modify(|v| *v += c).or_insert(c);
            new.entry((x, b)).and_modify(|v| *v += c).or_insert(c);
        }
        counts = new;
    }

    // distil pair counts into char counts
    let mut cc: HashMap<char, u64> = HashMap::new();
    for ((c, _), f) in counts {
        cc.entry(c).and_modify(|v| *v += f).or_insert(f);
    }
    cc.entry(templ.chars().last().unwrap()).and_modify(|v| *v += 1);

    Ok(cc.values().max().unwrap() - cc.values().min().unwrap())
}

fn part_a(lines: &[String]) -> AResult<u64> {
    solve(lines, 10)
}

fn part_b(lines: &[String]) -> AResult<u64> {
    solve(lines, 40)
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

    const TEST_INPUT: &str = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    #[test]
    fn test_a() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice())?, 1588);
        Ok(())
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice())?, 2188189693529);
        Ok(())
    }
}
