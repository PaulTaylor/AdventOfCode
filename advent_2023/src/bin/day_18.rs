use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

struct Direction(char, isize);

fn calculate_area(directions: &[Direction]) -> isize {
    let mut vertices = vec![(0, 0)];
    let mut length = 0isize;
    for dir in directions {
        let &(last_row, last_col) = vertices.iter().last().unwrap();
        let next = match dir {
            Direction('R', n) => (last_row, last_col + n),
            Direction('L', n) => (last_row, last_col - n),
            Direction('U', n) => (last_row - n, last_col),
            Direction('D', n) => (last_row + n, last_col),
            _ => unreachable!(),
        };
        vertices.push(next);
        length += dir.1;
    }

    let mut area = 0;
    for pair in vertices.windows(2) {
        let i = pair[0];
        let i_1 = pair[1];
        area += (i_1.0 + i.0) * (i_1.1 - i.1);
    }

    // Add 1 to account for the origin block and add the perimeter length to account for
    // the fact that the shoelace method only counts the internal space
    1 + (area.abs() + length) / 2
}

fn part_a(lines: &[String]) -> isize {
    let directions: Vec<_> = lines
        .iter()
        .filter_map(|l| {
            let mut bits = l.split_whitespace();
            Some(Direction(
                bits.next()?.chars().next()?,
                bits.next()?.parse().ok()?,
            ))
        })
        .collect();

    calculate_area(&directions)
}

fn part_b(lines: &[String]) -> AResult<isize> {
    let hex_pattern = Regex::new(r"\(#(\w{5})(\w)\)")?;
    let lookup: Vec<_> = "RDLU".chars().collect();
    let directions: Vec<_> = lines
        .iter()
        .filter_map(|l| {
            let caps = hex_pattern.captures(l)?;
            let dist_str = caps.get(1)?.as_str();
            let dir_str = caps.get(2)?.as_str();

            Some(Direction(
                lookup[dir_str.parse::<usize>().ok()?],
                isize::from_str_radix(dist_str, 16).ok()?,
            ))
        })
        .collect();
    Ok(calculate_area(&directions))
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
    println!("Part B result = {}", part_b(lines.as_slice())?);
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 62);
    }

    #[test]
    fn test_b() -> AResult<()> {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines)?, 952_408_144_115);
        Ok(())
    }
}
