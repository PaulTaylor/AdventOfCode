use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use advent_2019::run_program;

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<usize> {
    lines[0]
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn part_a(lines: &[String]) -> usize {
    let mut code = parse(lines);
    code[1] = 12;
    code[2] = 2;
    run_program(&mut code);
    code[0]
}

fn part_b(lines: &[String]) -> usize {
    let code = parse(lines);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = code.clone();
            memory[1] = noun;
            memory[2] = verb;
            run_program(&mut memory);

            if memory[0] == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("no solution found :(")
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
