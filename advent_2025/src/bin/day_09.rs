use humantime::format_duration;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

fn part_a(lines: &[String]) -> usize {
    let coords: Vec<(usize, usize)> = lines
        .iter()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut acc = 0;
    for (x1, y1) in &coords {
        for (x2, y2) in &coords {
            if x2 > x1 && y2 > y1 {
                acc = std::cmp::max((1 + x2 - x1) * (1 + y2 - y1), acc);
            }
        }
    }
    acc
}

fn part_b(lines: &[String]) -> usize {
    let corners: Vec<(usize, usize)> = lines
        .iter()
        .map(|s| {
            s.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut perimeter: Vec<_> = corners.windows(2).map(|v| (v[0], v[1])).collect();
    perimeter.push((*corners.last().unwrap(), *corners.first().unwrap()));

    let mut areas = BTreeSet::new();
    for (x1, y1) in &corners {
        for (x2, y2) in &corners {
            let area = (1 + x1.abs_diff(*x2)) * (1 + y1.abs_diff(*y2));
            areas.insert((area, (x1, y1), (x2, y2)));
        }
    }

    while let Some((area, (x1, y1), (x2, y2))) = areas.pop_last() {
        let top_left = (x1.min(x2), y1.min(y2));
        let bottom_right = (x1.max(x2), y1.max(y2));

        let mut valid = true;
        for ((lx1, ly1), (lx2, ly2)) in &perimeter {
            let xs = lx1.min(lx2);
            let xe = lx1.max(lx2);
            let ys = ly1.min(ly2);
            let ye = ly1.max(ly2);

            if xs == xe {
                // Vertical line
                if top_left.0 < xs
                    && bottom_right.0 > xs
                    && !(top_left.1 >= ye || bottom_right.1 <= ys)
                {
                    valid = false;
                    break;
                }
            } else if ys == ye {
                // Horizontal line
                if top_left.1 < ys
                    && bottom_right.1 > ys
                    && !(top_left.1 >= xe || bottom_right.1 <= xs)
                {
                    valid = false;
                    break;
                }
            } else {
                panic!()
            }
        }

        if valid {
            return area;
        }
    }

    panic!()
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

    const TEST_INPUT: &str = "7,1
                              11,1
                              11,7
                              9,7
                              9,5
                              2,5
                              2,3
                              7,3";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_a(lines.as_slice()), 50);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(part_b(&lines), 24);

        assert_eq!(
            part_b(
                &"4,2
13,2
13,4
8,4
8,6
11,6
11,10
4,10"
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            ),
            40
        )
    }
}
