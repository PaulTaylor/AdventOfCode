use humantime::format_duration;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

struct Instr {
    dir: char,
    dist: usize,
}

impl From<&str> for Instr {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        Instr {
            dir: chars.next().unwrap(),
            dist: chars.collect::<String>().parse().unwrap(),
        }
    }
}

fn parse(lines: &[String]) -> (Vec<Instr>, Vec<Instr>) {
    (
        lines[0].split(',').map(Instr::from).collect(),
        lines[1].split(',').map(Instr::from).collect(),
    )
}

fn generate_coords(wire: &[Instr]) -> Vec<(isize, isize)> {
    let mut coords = Vec::new();
    let (mut row, mut col) = (0, 0);
    for &Instr { dir, dist } in wire {
        for _step in 0..dist {
            match dir {
                'R' => col += 1,
                'U' => row += 1,
                'L' => col -= 1,
                'D' => row -= 1,
                x => panic!("Unknown direction {x}"),
            }

            // Update the grid
            coords.push((row, col));
        }
    }
    coords
}

fn part_a(lines: &[String]) -> usize {
    let (w1, w2) = parse(lines);

    let c1: HashSet<(isize, isize)> = HashSet::from_iter(generate_coords(&w1));
    let c2 = HashSet::from_iter(generate_coords(&w2));

    c1.intersection(&c2)
        .map(|(row, col)| row.abs() + col.abs())
        .min()
        .unwrap()
        .try_into()
        .unwrap()
}

fn part_b(lines: &[String]) -> usize {
    let (w1, w2) = parse(lines);

    let c1 = generate_coords(&w1);
    let c2 = generate_coords(&w2);
    let c1_set: HashSet<(isize, isize)> = c1.iter().copied().collect();
    let c2_set = c2.iter().copied().collect();

    let dists: Vec<_> = c1_set
        .intersection(&c2_set)
        .map(|cand| {
            let dist1 = c1.iter().position(|x| x == cand).unwrap();
            let dist2 = c2.iter().position(|x| x == cand).unwrap();
            (dist1 + 1, dist2 + 1)
        })
        .map(|(d1, d2)| d1 + d2)
        .collect();

    dists.into_iter().min().unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUTS: &[(&str, usize, usize); 3] = &[
        (
            "R8,U5,L5,D3
    U7,R6,D4,L4",
            6,
            30,
        ),
        (
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83",
            159,
            610,
        ),
        (
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            135,
            410,
        ),
    ];

    #[test]
    fn test_a() {
        for (inp, actual, _) in TEST_INPUTS {
            let lines: Vec<_> = inp.lines().map(|l| l.trim().to_string()).collect();
            assert_eq!(part_a(lines.as_slice()), *actual);
        }
    }

    #[test]
    fn test_b() {
        for (inp, _, actual) in TEST_INPUTS {
            let lines: Vec<_> = inp.lines().map(|l| l.trim().to_string()).collect();
            assert_eq!(part_b(lines.as_slice()), *actual);
        }
    }
}
