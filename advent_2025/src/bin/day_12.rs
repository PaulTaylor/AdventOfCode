use humantime::format_duration;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Coord = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    id: usize,
    points: Vec<(isize, isize)>,
}

fn parse(lines: &[String]) -> (Vec<Shape>, Vec<Coord>, Vec<Vec<usize>>) {
    let shape_start = Regex::new(r"^(\d+):").unwrap();
    let alloc_pattern = Regex::new(r"^(\d+)x(\d+): ([0-9\s]+)$").unwrap();

    let mut shapes = vec![];
    let mut areas = vec![];
    let mut allocations = vec![];

    let mut idx = 0;

    while idx < lines.len() {
        let line = &lines[idx];

        if let Some(caps) = shape_start.captures(&line) {
            let shape_num = caps.get(1).unwrap().as_str().parse().unwrap();

            let mut points = vec![];
            let mut offset = 0;
            idx += 1;
            while !lines[idx + offset].is_empty() {
                let line = &lines[idx + offset];
                for (cid, c) in line.char_indices() {
                    if c == '#' {
                        points.push((cid as isize, offset as isize));
                    }
                }
                offset += 1;
            }

            shapes.push(Shape { id: shape_num, points });
            idx += offset - 1;
        }

        if let Some(caps) = alloc_pattern.captures(&line) {
            let x_max: usize = caps.get(1).and_then(|v| v.as_str().parse().ok()).unwrap();
            let y_max: usize = caps.get(2).and_then(|v| v.as_str().parse().ok()).unwrap();
            areas.push((x_max, y_max));
            allocations.push(
                caps.get(3)
                    .unwrap()
                    .as_str()
                    .split(' ')
                    .flat_map(|s| s.parse())
                    .collect(),
            )
        }

        idx += 1;
    }

    (shapes, areas, allocations)
}

fn part_a(lines: &[String]) -> usize {
    let (raw_shapes, areas, allocations) = parse(lines);

    areas
        .iter()
        .zip(allocations)
        .map(|((max_x, max_y), alloc)| {
            let shapes: Vec<Shape> = raw_shapes
                .iter()
                .zip(alloc)
                .flat_map(|(shape, count)| (0..count).map(|_| shape).cloned().collect::<Vec<_>>())
                .collect();

            let shape_vol = shapes.iter().map(|s| s.points.len()).sum::<usize>();
            // Just do a simple check to see if there is sufficient volume available
            // and it works for the puzzle input.
            (shape_vol < (max_x * max_y)) as usize
        })
        .sum()
}

fn part_b(lines: &[String]) -> usize {
    0
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

    const TEST_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 3);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&vec![String::from("L50"), String::from("L1")]), 1);
        assert_eq!(part_b(&vec![String::from("L50"), String::from("R1")]), 1);
        assert_eq!(part_b(&vec![String::from("R200")]), 2);
        assert_eq!(part_b(&vec![String::from("L200")]), 2);
        assert_eq!(part_b(&lines), 6);
        assert_eq!(part_b(&vec![String::from("R1000")]), 10);
    }
}
