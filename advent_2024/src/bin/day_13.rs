use conv::*;
use humantime::format_duration;
use itertools::Itertools;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<[isize; 6]> {
    let mut out = vec![];
    let pattern = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();
    for idx in (0..lines.len()).step_by(4) {
        let mut prob: [_; 6] = [0; 6];
        for offset in 0..3 {
            let (_, [x, y]): (_, [&str; 2]) =
                pattern.captures(&lines[idx + offset]).unwrap().extract();
            prob[offset * 2] = x.parse().unwrap();
            prob[(offset * 2) + 1] = y.parse().unwrap();
        }
        out.push(prob);
    }
    out
}

fn solve(problem: [isize; 6], a_cost: isize, b_cost: isize) -> Option<isize> {
    // No interesting code here just simple implementation of pen and paper algebra
    let [ax, ay, bx, by, tx, ty] = problem;

    let a = ((by * tx) - (bx * ty)) / ((ax * by) - (ay * bx));
    let b = (tx - (a * ax)) / bx;

    if tx == ((a * ax) + (b * bx)) && ty == ((a * ay) + (b * by)) {
        return Some((a_cost * a) + (b_cost * b));
    }

    None
}

fn part_a(lines: &[String]) -> isize {
    parse(lines)
        .into_iter()
        .filter_map(|problem| solve(problem, 3, 1))
        .sum()
}

fn part_b(lines: &[String]) -> isize {
    let adjustment = 10_000_000_000_000;
    parse(lines)
        .into_iter()
        .update(|progress| {
            progress[4] += adjustment;
            progress[5] += adjustment;
        })
        .filter_map(|problem| solve(problem, 3, 1))
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

    const TEST_INPUT: &str = "Button A: X+94, Y+34
                              Button B: X+22, Y+67
                              Prize: X=8400, Y=5400

                              Button A: X+26, Y+66
                              Button B: X+67, Y+21
                              Prize: X=12748, Y=12176

                              Button A: X+17, Y+86
                              Button B: X+84, Y+37
                              Prize: X=7870, Y=6450

                              Button A: X+69, Y+23
                              Button B: X+27, Y+71
                              Prize: X=18641, Y=10279";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 480);
    }
}
