use advent_2019::run_program;
use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<isize> {
    return lines[0]
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
}

fn part_a(lines: &[String]) -> isize {
    let mut memory = parse(lines);
    let output = run_program(&mut memory, &[1]);
    let dc = *output.iter().last().unwrap();
    assert_eq!(output.iter().sum::<isize>(), dc, "Error in execution");
    dc
}

fn part_b(lines: &[String]) -> isize {
    let mut memory = parse(lines);
    let output = run_program(&mut memory, &[5]);
    let dc = *output.iter().last().unwrap();
    assert_eq!(output.iter().sum::<isize>(), dc, "Error in execution");
    dc
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
    println!("Part A result = {}", part_a(lines.as_slice()));
    println!("Part B result = {}", part_b(lines.as_slice()));
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}
