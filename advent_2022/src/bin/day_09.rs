use humantime::format_duration;
use regex::Regex;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn parse(lines: &[String]) -> Vec<(char, i32)> {
    lines
        .iter()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .map(|l| (l[0].chars().next().unwrap(), l[1].parse().unwrap()))
        .collect()
}

fn calculate_diff(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    (h.0 - t.0, h.1 - t.1)
}

fn solve(lines: &[String], knots: usize) -> usize {
    let instr = parse(lines);
    let mut positions: Vec<_> = (0..knots).map(|_| (0, 0)).collect();
    let mut t_visited: HashSet<(i32, i32)> = HashSet::from_iter([(0, 0)]);

    for (dir, quant) in instr {
        for _ in 0..quant {
            positions[0] = match (dir, positions[0]) {
                ('U', (x, y)) => (x, y + 1),
                ('D', (x, y)) => (x, y - 1),
                ('L', (x, y)) => (x - 1, y),
                ('R', (x, y)) => (x + 1, y),
                _ => panic!(),
            };

            // Update the rest of the knots in turn
            for ki in 1..positions.len() {
                let prev = positions[ki - 1];
                let mut curr = positions[ki];

                let (x_dist, y_dist) = calculate_diff(prev, curr);
                if x_dist.abs() < 2 && y_dist.abs() < 2 {
                    continue;
                }

                curr = (curr.0 + x_dist.signum(), curr.1 + y_dist.signum());
                if ki == knots - 1 {
                    t_visited.insert(curr); // track the positions of the final knot
                }
                positions[ki] = curr;
            }
        }
    }
    t_visited.len()
}

fn part_a(lines: &[String]) -> usize {
    solve(lines, 2)
}

fn part_b(lines: &[String]) -> usize {
    solve(lines, 10)
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

    const TEST_INPUT: &str = "R 4
                              U 4
                              L 3
                              D 1
                              R 4
                              D 1
                              L 5
                              R 2";

    const TEST_LONGER: &str = "R 5
                               U 8
                               L 8
                               D 3
                               R 17
                               D 10
                               L 25
                               U 20";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 13);
    }

    #[test]
    fn test_solve() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(solve(lines.as_slice(), 2), 13);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 1);

        let lines: Vec<_> = TEST_LONGER.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(lines.as_slice()), 36);
    }
}
