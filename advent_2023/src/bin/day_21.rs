use humantime::format_duration;
use indicatif::ProgressIterator;
use regex::Regex;
use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

type AResult<T> = anyhow::Result<T>;

#[allow(clippy::cast_possible_wrap)]
fn parse(lines: &[String]) -> ((isize, isize), BTreeSet<(isize, isize)>) {
    let start = lines
        .iter()
        .enumerate()
        .find_map(|(rid, row)| {
            row.char_indices().find_map(|(cid, c)| {
                if c == 'S' {
                    Some((rid as isize, cid as isize))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut rocks = BTreeSet::new();
    for (rid, row) in lines.iter().enumerate() {
        for (cid, c) in row.char_indices() {
            if c == '#' {
                rocks.insert((rid as isize, cid as isize));
            }
        }
    }

    (start, rocks)
}

#[allow(clippy::cast_possible_wrap)]
fn solve(lines: &[String], steps: &[usize]) -> Vec<usize> {
    let (start, rocks) = parse(lines);
    let rows = lines.len() as isize;
    let cols = lines[0].len() as isize;

    let mut locs = BTreeSet::new();
    locs.insert(start);

    let max_steps = *steps.iter().max().unwrap();

    let mut outputs = vec![];
    for step in (0..max_steps).progress() {
        // Generate a set create from all 4 moves for each possible location
        let next: BTreeSet<_> = locs
            .into_iter()
            .flat_map(|(rid, cid)| {
                //
                let n = (rid - 1, cid);
                let e = (rid, cid + 1);
                let s = (rid + 1, cid);
                let w = (rid, cid - 1);
                vec![n, e, s, w]
            })
            .collect();

        // Remove the locations of the rocks
        locs = next
            .into_iter()
            .filter(|&(rid, cid)| !rocks.contains(&(rid.rem_euclid(rows), cid.rem_euclid(cols))))
            .collect();

        if steps.contains(&(step + 1)) {
            outputs.push(locs.len());
        }
    }

    outputs
}

fn part_a(lines: &[String]) -> usize {
    solve(lines, &[64])[0]
}

#[allow(clippy::similar_names)]
fn part_b(lines: &[String]) -> AResult<i128> {
    // Obs: Visited counts follow a (ragged) quadratic - but too close to be coincidence.
    // Obs: The weird target number / grid size is a multiple of 202300.XXXXX - coincidence??
    // Obs: steps(grid_size), steps(grid_size*2), steps(grid_size*3) follow a clean quadratic
    //
    // If I can workout the offset (remainder in the division above), and then do run to 3 periods
    // of (grid_size * i) + offset.  Then I can get the quadratic parameters from Wolfram Alpha and
    // then calculate the 202300'th element.
    //
    const TARGET_STEPS: i128 = 26_501_365;
    let lines_len = lines.len() as i128;
    let (whole_periods, offset): (i128, usize) = (
        TARGET_STEPS / lines_len,
        TARGET_STEPS.rem_euclid(lines_len) as usize,
    );

    println!("Number of required periods = {whole_periods}, offset = {offset}");

    let to_calc = [
        offset,
        offset + lines.len(),
        offset + (lines.len() * 2),
        offset + (lines.len() * 3),
    ];
    let results = solve(lines, &to_calc);

    println!(
        "Now visit: https://www.wolframalpha.com/ and input the following:
    \"fit quadratic {{{{0,{}}},{{1,{}}},{{2,{}}},{{3,{}}}}}\"",
        results[0], results[1], results[2], results[3]
    );

    println!("Enter the x^2 term:");
    let mut x2_str = String::new();
    std::io::stdin().read_line(&mut x2_str)?;
    println!("Enter the x term (with the - if appropriate):");
    let mut x_str = String::new();
    std::io::stdin().read_line(&mut x_str)?;
    let mut c_str = String::new();
    println!("Enter the constant term (with the - if appropriate):");
    std::io::stdin().read_line(&mut c_str)?;

    let x2: i128 = x2_str.trim().parse().expect("A positive integer x^2 term");
    let x: i128 = x_str.trim().parse().expect("An integer x term");
    let c: i128 = c_str.trim().parse().expect("An integer constant term");

    // Gotcha: ^ is not the operator for powers in rust 🤦 (it's Bitwise exclusive OR)
    Ok((x2 * (whole_periods.pow(2))) + (x * whole_periods) + c)
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

    const TEST_INPUT: &str = "...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ...........";

    #[test]
    fn test_solve() {
        let lines: Vec<_> = TEST_INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(solve(&lines, &[6, 10, 50, 100]), [16, 50, 1594, 6536]);
    }
}
