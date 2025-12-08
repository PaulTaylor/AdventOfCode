use humantime::format_duration;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{BTreeSet, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;
type Coord = (isize, isize, isize);

fn parse(lines: &[String]) -> (Vec<HashSet<Coord>>, BTreeSet<(isize, Coord, Coord)>) {
    let junctions: Vec<Coord> = lines
        .into_iter()
        .map(|line| -> Coord {
            line.split(',')
                .filter_map(|s| s.parse().ok())
                .collect_tuple()
                .expect("Bad line format")
        })
        .collect();

    let mut distances = BTreeSet::new();
    for (i1, j1) in junctions.iter().enumerate() {
        let (x1, y1, z1) = j1;
        for j2 in junctions.iter().skip(i1 + 1) {
            let (x2, y2, z2) = j2;
            let dist = ((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)).isqrt();
            distances.insert((dist, *j1, *j2));
        }
    }

    let circuits: Vec<_> = junctions
        .into_iter()
        .map(|a| HashSet::from_iter([a]))
        .collect();

    (circuits, distances)
}

fn solve(lines: &[String], n_conn: usize) -> (usize, usize) {
    let (mut circuits, mut distances) = parse(lines);

    let mut a_res = 0;
    let mut connection_count = 1;
    while let Some((_d, u, v)) = distances.pop_first() {
        let (&u_idx, &v_idx) = [
            circuits.iter().position(|s| s.contains(&u)).unwrap(),
            circuits.iter().position(|s| s.contains(&v)).unwrap(),
        ]
        .iter()
        .sorted_unstable()
        .collect_tuple()
        .unwrap();

        if u_idx != v_idx {
            let v_set = circuits.remove(v_idx);
            let u_set = circuits.get_mut(u_idx).unwrap();
            u_set.extend(v_set);
        }

        if connection_count == n_conn {
            a_res = circuits.iter().map(|s| s.len()).k_largest(3).product()
        }

        if circuits.len() == 1 {
            return (a_res, (u.0 * v.0) as usize);
        }

        connection_count += 1;
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
    let (a_res, b_res) = solve(&lines, 1000);
    println!("Part A result = {}", a_res);
    println!("Part B result = {}", b_res);
    let end = Instant::now();

    println!("Run took {}", format_duration(end - start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "162,817,812
                              57,618,57
                              906,360,560
                              592,479,940
                              352,342,300
                              466,668,158
                              542,29,236
                              431,825,988
                              739,650,466
                              52,470,668
                              216,146,977
                              819,987,18
                              117,168,530
                              805,96,715
                              346,949,466
                              970,615,88
                              941,993,340
                              862,61,35
                              984,92,344
                              425,690,689";

    #[test]
    fn test_a() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(solve(lines.as_slice(), 10).0, 40);
    }

    #[test]
    fn test_b() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(solve(&lines, usize::MAX).1, 25272);
    }
}
