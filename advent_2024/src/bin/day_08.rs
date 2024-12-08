use conv::ValueFrom;
use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type ParsedInput = (
    HashMap<char, Vec<(isize, isize)>>,
    Range<isize>,
    Range<isize>,
);

fn parse(lines: &[String]) -> ParsedInput {
    let mut out: HashMap<char, Vec<_>> = HashMap::new();
    for (y, row) in lines.iter().enumerate() {
        row.char_indices()
            .filter_map(move |(x, c)| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => Some((
                    (isize::value_from(x).unwrap(), isize::value_from(y).unwrap()),
                    c,
                )),
                _ => None,
            })
            .for_each(|((x, y), c)| {
                out.entry(c)
                    .and_modify(|l| l.push((x, y)))
                    .or_insert_with(|| vec![(x, y)]);
            });
    }

    let x_bounds = 0..isize::value_from(lines[0].len()).unwrap();
    let y_bounds = 0..isize::value_from(lines.len()).unwrap();

    (out, x_bounds, y_bounds)
}

fn part_a(input_lines: &[String]) -> usize {
    let (antennae, x_bounds, y_bounds) = parse(input_lines);

    let mut antinodes = BTreeSet::new();

    for (_freq, points) in antennae {
        for (idx, (x1, y1)) in points.iter().enumerate() {
            for (x2, y2) in &points[idx + 1..] {
                let x_diff = x2 - x1;
                let y_diff = y2 - y1;

                if x_bounds.contains(&(x1 - x_diff)) && y_bounds.contains(&(y1 - y_diff)) {
                    antinodes.insert((x1 - x_diff, y1 - y_diff));
                }
                if x_bounds.contains(&(x2 + x_diff)) && y_bounds.contains(&(y2 + y_diff)) {
                    antinodes.insert((x2 + x_diff, y2 + y_diff));
                }
            }
        }
    }

    antinodes.len()
}

fn part_b(input_lines: &[String]) -> usize {
    let (antennae, x_bounds, y_bounds) = parse(input_lines);
    let mut antinodes = BTreeSet::new();

    for (_freq, points) in antennae {
        for &(x1, y1) in &points {
            for &(x2, y2) in &points {
                if (x1, y1) == (x2, y2) {
                    continue;
                }

                let (x_step, y_step) = (x2 - x1, y2 - y1);
                let (mut x3, mut y3) = (x1, y1);

                while x_bounds.contains(&x3) && y_bounds.contains(&y3) {
                    antinodes.insert((x3, y3));
                    (x3, y3) = (x3 + x_step, y3 + y_step);
                }
            }
        }
    }

    antinodes.len()
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

    const TEST_INPUTS: [(&str, usize); 4] = [
        ("..........\n..........\n..........\n....a.....\n..........\n.....a....\n..........\n..........\n..........\n..........",2),
        ("..........\n..........\n..........\n....a.....\n........a.\n.....a....\n..........\n..........\n..........\n..........", 4),
        ("..........\n..........\n..........\n....a.....\n........a.\n.....a....\n..........\n......A...\n..........\n..........", 4),
        ("............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............", 14)
    ];

    const TEST_INPUTS_B : [(&str, usize); 2] = [
        ("T.........\n...T......\n.T........\n..........\n..........\n..........\n..........\n..........\n..........\n..........", 9), 
        ("............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............", 34)
    ];

    #[test]
    fn test_a() {
        for (input, target) in TEST_INPUTS {
            let lines: Vec<_> = input.lines().map(|l| l.trim().to_string()).collect();
            assert_eq!(part_a(lines.as_slice()), target);
        }
    }

    #[test]
    fn test_b() {
        for (input, target) in TEST_INPUTS_B {
            let lines: Vec<_> = input.lines().map(|l| l.trim().to_string()).collect();
            assert_eq!(part_b(lines.as_slice()), target);
        }
    }
}
