use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn part_a(lines: &[String]) -> usize {
    lines[0].split(',').map(hash).sum()
}

fn hash(s: &str) -> usize {
    let mut cv = 0usize;

    for c in s.chars() {
        cv += c as usize;
        cv *= 17;
        cv %= 256;
    }

    cv
}

#[derive(Debug)]
struct Lens<'a>(&'a str, usize);

fn part_b(lines: &[String]) -> usize {
    let instructions = lines[0].split(',');
    let mut boxes: Vec<Vec<Lens>> = (0..256).map(|_| Vec::new()).collect();

    let pattern = Regex::new(r"(\w+)([=-])(\d*)").unwrap();

    for ins in instructions {
        let (_, groups): (_, [&str; 3]) = pattern.captures(ins).expect("a match").extract();

        match groups {
            [label, "-", ""] => {
                let box_id = hash(label);
                boxes[box_id].retain(|l| l.0 != label);
            }
            [label, "=", fl] => {
                let box_id = hash(label);
                let fl = fl.parse::<usize>().unwrap();

                let existing = boxes[box_id].iter_mut().find(|l| l.0 == label);
                match existing {
                    // Rather than do a real replace - we'll just
                    // update the focal length for the existing object
                    Some(l) => l.1 = fl,
                    None => boxes[box_id].push(Lens(label, fl)),
                }
            }
            _ => unreachable!(),
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(box_id, b)| {
            b.iter()
                .enumerate()
                .map(|(idx, l)| (box_id + 1) * (idx + 1) * l.1)
                .sum::<usize>()
        })
        .sum()
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

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(&lines), 1320);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 145);
    }
}
